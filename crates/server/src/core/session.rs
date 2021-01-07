//! Core functionality related to the LSP server session.

use crate::{
    core::{
        database::{Database, DocumentStatus},
        document::Document,
        error::Error,
    },
    server,
};
use dashmap::{
    mapref::one::{Ref, RefMut},
    DashMap,
};
use tokio::sync::{Mutex, RwLock};
use zerocopy::AsBytes;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SessionResourceKind {
    Document,
    Parser,
    Tree,
}

/// Represents the current state of the LSP service.
pub struct Session {
    /// The LSP server capabilities.
    pub(crate) server_capabilities: RwLock<lsp::ServerCapabilities>,
    /// The LSP client capabilities.
    pub(crate) client_capabilities: RwLock<Option<lsp::ClientCapabilities>>,
    /// The LSP client handle.
    client: Option<lspower::Client>,
    /// The document metadata database.
    database: Database,
    /// The store of currently open documents.
    documents: DashMap<lsp::Url, Document>,
    /// The store of currently open parsers (one per synax tree).
    pub(crate) parsers: DashMap<lsp::Url, Mutex<tree_sitter::Parser>>,
    /// The store of currently open syntax trees (one per document).
    pub(crate) trees: DashMap<lsp::Url, Mutex<tree_sitter::Tree>>,
}

impl Session {
    /// Create a new session.
    pub fn new(client: Option<lspower::Client>) -> anyhow::Result<Self> {
        let server_capabilities = RwLock::new(server::capabilities());
        let client_capabilities = RwLock::new(Default::default());
        let database = Database::new()?;
        let documents = DashMap::new();
        let parsers = DashMap::new();
        let trees = DashMap::new();
        Ok(Session {
            server_capabilities,
            client_capabilities,
            client,
            database,
            documents,
            parsers,
            trees,
        })
    }

    pub(crate) fn client(&self) -> anyhow::Result<&lspower::Client> {
        self.client.as_ref().ok_or_else(|| Error::ClientNotInitialized.into())
    }

    /// Insert an opened document into the session. Updates the documents hashmap and sets the
    /// document status in the database to "opened". Notifies subscribers to the document status.
    pub fn insert_document(
        &self,
        uri: lsp::Url,
        document: Document,
        parser: tree_sitter::Parser,
        tree: tree_sitter::Tree,
    ) -> anyhow::Result<()> {
        let result = self.documents.insert(uri.clone(), document);
        debug_assert!(result.is_none());
        let result = self.parsers.insert(uri.clone(), Mutex::new(parser));
        debug_assert!(result.is_none());
        let result = self.trees.insert(uri.clone(), Mutex::new(tree));
        debug_assert!(result.is_none());
        let status = DocumentStatus::opened();
        let status = status.as_bytes();
        self.database.trees.documents.insert(&uri[..], status)?;
        Ok(())
    }

    /// Remove a closed document from the session. Updates the documents hashmap and sets the
    /// document status in the database to "closed". Notifies subscribers to the document status.
    pub fn remove_document(&self, uri: &lsp::Url) -> anyhow::Result<()> {
        let result = self.documents.remove(uri);
        debug_assert!(result.is_some());
        let result = self.parsers.remove(uri);
        debug_assert!(result.is_some());
        let result = self.trees.remove(uri);
        debug_assert!(result.is_some());
        let status = DocumentStatus::closed();
        let status = status.as_bytes();
        self.database.trees.documents.insert(&uri[..], status)?;
        Ok(())
    }

    /// FIXME: we should be able to avoid cloning here
    pub(crate) async fn semantic_tokens_legend(&self) -> Option<lsp::SemanticTokensLegend> {
        let capabilities = self.server_capabilities.read().await;
        if let Some(capabilities) = &capabilities.semantic_tokens_provider {
            match capabilities {
                lsp::SemanticTokensServerCapabilities::SemanticTokensOptions(options) => Some(options.legend.clone()),
                lsp::SemanticTokensServerCapabilities::SemanticTokensRegistrationOptions(options) => {
                    Some(options.semantic_tokens_options.legend.clone())
                },
            }
        } else {
            None
        }
    }

    /// Get a reference to a document associated with the session, if possible.
    pub async fn get_document(&self, uri: &lsp::Url) -> anyhow::Result<Ref<'_, lsp::Url, Document>> {
        self.documents.get(uri).ok_or_else(|| {
            let kind = SessionResourceKind::Document;
            let uri = uri.clone();
            Error::SessionResourceNotFound { kind, uri }.into()
        })
    }

    /// Get a mutable reference to a document associated with the session, if possible.
    pub async fn get_mut_document(&self, uri: &lsp::Url) -> anyhow::Result<RefMut<'_, lsp::Url, Document>> {
        self.documents.get_mut(uri).ok_or_else(|| {
            let kind = SessionResourceKind::Document;
            let uri = uri.clone();
            Error::SessionResourceNotFound { kind, uri }.into()
        })
    }

    /// Get a mutable reference to a parser associated with the session, if possible.
    pub async fn get_mut_parser(
        &self,
        uri: &lsp::Url,
    ) -> anyhow::Result<RefMut<'_, lsp::Url, Mutex<tree_sitter::Parser>>> {
        self.parsers.get_mut(uri).ok_or_else(|| {
            let kind = SessionResourceKind::Parser;
            let uri = uri.clone();
            Error::SessionResourceNotFound { kind, uri }.into()
        })
    }

    /// Get a reference to a tree associated with the session, if possible.
    pub async fn get_tree(&self, uri: &lsp::Url) -> anyhow::Result<Ref<'_, lsp::Url, Mutex<tree_sitter::Tree>>> {
        self.trees.get(uri).ok_or_else(|| {
            let kind = SessionResourceKind::Tree;
            let uri = uri.clone();
            Error::SessionResourceNotFound { kind, uri }.into()
        })
    }

    /// Get a mutable reference to a tree associated with the session, if possible.
    pub async fn get_mut_tree(&self, uri: &lsp::Url) -> anyhow::Result<RefMut<'_, lsp::Url, Mutex<tree_sitter::Tree>>> {
        self.trees.get_mut(uri).ok_or_else(|| {
            let kind = SessionResourceKind::Tree;
            let uri = uri.clone();
            Error::SessionResourceNotFound { kind, uri }.into()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Session;
    use crate::core::{database::DocumentStatus, document::Document, error::Error, language::Language};
    use std::convert::TryFrom;
    use zerocopy::AsBytes;

    #[tokio::test]
    async fn client_not_initialized() -> anyhow::Result<()> {
        let client = None;
        let session = Session::new(client)?;
        let result = session.client();

        assert!(result.is_err());
        if let Err(error) = result {
            let expected = &Error::ClientNotInitialized;
            let actual = error.root_cause().downcast_ref::<Error>();
            assert!(actual.is_some());
            if let Some(actual) = actual {
                assert_eq!(expected, actual);
            }
        }

        Ok(())
    }

    mod document_not_found {
        use crate::core::{self, error::Error};

        #[tokio::test]
        async fn get_document() -> anyhow::Result<()> {
            let client = None;
            let session = core::Session::new(client)?;
            let uri = lsp::Url::parse("inmemory:///test")?;
            let result = session.get_document(&uri).await;

            assert!(result.is_err());
            if let Err(error) = result {
                let expected = {
                    let kind = core::SessionResourceKind::Document;
                    let uri = uri.clone();
                    &Error::SessionResourceNotFound { kind, uri }
                };
                let actual = error.root_cause().downcast_ref::<Error>();
                assert!(actual.is_some());
                if let Some(actual) = actual {
                    assert_eq!(expected, actual);
                }
            }

            Ok(())
        }

        #[tokio::test]
        async fn get_mut_document() -> anyhow::Result<()> {
            let client = None;
            let session = core::Session::new(client)?;
            let uri = lsp::Url::parse("inmemory:///test")?;
            let result = session.get_mut_document(&uri).await;

            assert!(result.is_err());
            if let Err(error) = result {
                let expected = {
                    let kind = core::SessionResourceKind::Document;
                    let uri = uri.clone();
                    &Error::SessionResourceNotFound { kind, uri }
                };
                let actual = error.root_cause().downcast_ref::<Error>();
                assert!(actual.is_some());
                if let Some(actual) = actual {
                    assert_eq!(expected, actual);
                }
            }

            Ok(())
        }
    }

    #[tokio::test]
    async fn insert_document() -> anyhow::Result<()> {
        let client = None;
        let session = Session::new(client)?;

        let prefix = vec![];
        let mut subscriber = session.database.trees.documents.watch_prefix(prefix);

        let uri = lsp::Url::parse("inmemory:///test")?;

        let language_id = "wasm.wast";
        let text = String::new();
        let language = Language::try_from(language_id)?;
        let mut parser = tree_sitter::Parser::try_from(language)?;
        let old_tree = None;
        let tree = parser.parse(&text[..], old_tree);
        let document = Document::new(language_id, text)?;

        if let Some(tree) = tree {
            session.insert_document(uri.clone(), document, parser, tree)?;
        }

        while let Some(event) = (&mut subscriber).await {
            if let sled::Event::Insert { key, value } = event {
                if &*key == uri.as_ref().as_bytes() && value == DocumentStatus::opened().as_bytes() {
                    assert!(session.documents.get(&uri).is_some());
                    return Ok(());
                }
            }
        }

        unreachable!()
    }

    #[tokio::test]
    async fn remove_document() -> anyhow::Result<()> {
        let client = None;
        let session = Session::new(client)?;

        let prefix = vec![];
        let mut subscriber = session.database.trees.documents.watch_prefix(prefix);

        let uri = lsp::Url::parse("inmemory:///test")?;

        let language_id = "wasm.wast";
        let text = String::new();
        let language = Language::try_from(language_id)?;
        let mut parser = tree_sitter::Parser::try_from(language)?;
        let old_tree = None;
        let tree = parser.parse(&text[..], old_tree);
        let document = Document::new(language_id, text)?;

        if let Some(tree) = tree {
            session.insert_document(uri.clone(), document, parser, tree)?;
        }

        while let Some(event) = (&mut subscriber).await {
            if let sled::Event::Insert { key, value } = event {
                if &*key == uri.as_ref().as_bytes() && value == DocumentStatus::opened().as_bytes() {
                    assert!(session.documents.get(&uri).is_some());
                    break;
                }
            }
        }

        session.remove_document(&uri)?;

        while let Some(event) = (&mut subscriber).await {
            if let sled::Event::Insert { key, value } = event {
                if &*key == uri.as_ref().as_bytes() && value == DocumentStatus::closed().as_bytes() {
                    assert!(session.documents.get(&uri).is_none());
                    return Ok(());
                }
            }
        }

        unreachable!()
    }
}
