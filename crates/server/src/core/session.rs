//! Core functionality related to the LSP server session.

use crate::core::{
    database::{Database, DocumentStatus},
    document::Document,
    error::Error,
};
use dashmap::{
    mapref::one::{Ref, RefMut},
    DashMap,
};
use lspower::{lsp_types::*, Client};
use tokio::sync::RwLock;
use zerocopy::AsBytes;

/// Represents the current state of the LSP service.
pub struct Session {
    /// The LSP server capabilities.
    pub(crate) server_capabilities: RwLock<ServerCapabilities>,
    /// The LSP client capabilities.
    pub(crate) client_capabilities: RwLock<Option<ClientCapabilities>>,
    /// The LSP client handle.
    client: Option<Client>,
    /// The document metadata database.
    database: Database,
    /// The store of currently open documents.
    documents: DashMap<Url, Document>,
}

impl Session {
    /// Create a new session.
    pub fn new(client: Option<Client>) -> anyhow::Result<Self> {
        let server_capabilities = RwLock::new(crate::lsp::server::capabilities());
        let client_capabilities = RwLock::new(Default::default());
        let database = Database::new()?;
        let documents = DashMap::new();
        Ok(Session {
            server_capabilities,
            client_capabilities,
            client,
            database,
            documents,
        })
    }

    pub(crate) fn client(&self) -> anyhow::Result<&Client> {
        self.client.as_ref().ok_or_else(|| Error::ClientNotInitialized.into())
    }

    /// Insert an opened document into the session. Updates the documents hashmap and sets the
    /// document status in the database to "opened". Notifies subscribers to the document status.
    pub fn insert_document(&self, uri: Url, document: Document) -> anyhow::Result<Option<Document>> {
        let result = self.documents.insert(uri.clone(), document);
        let status = DocumentStatus::opened();
        let status = status.as_bytes();
        self.database.trees.documents.insert(&uri[..], status)?;
        Ok(result)
    }

    /// Remove a closed document from the session. Updates the documents hashmap and sets the
    /// document status in the database to "closed". Notifies subscribers to the document status.
    pub fn remove_document(&self, uri: &Url) -> anyhow::Result<Option<(Url, Document)>> {
        let result = self.documents.remove(uri);
        let status = DocumentStatus::closed();
        let status = status.as_bytes();
        self.database.trees.documents.insert(&uri[..], status)?;
        Ok(result)
    }

    /// FIXME: we should be able to avoid cloning here
    pub(crate) async fn semantic_tokens_legend(&self) -> Option<SemanticTokensLegend> {
        let capabilities = self.server_capabilities.read().await;
        if let Some(capabilities) = &capabilities.semantic_tokens_provider {
            match capabilities {
                SemanticTokensServerCapabilities::SemanticTokensOptions(options) => Some(options.legend.clone()),
                SemanticTokensServerCapabilities::SemanticTokensRegistrationOptions(options) => {
                    Some(options.semantic_tokens_options.legend.clone())
                },
            }
        } else {
            None
        }
    }

    /// Get a reference to a document associated with the session, if possible.
    pub async fn get_document(&self, uri: &Url) -> anyhow::Result<Ref<'_, Url, Document>> {
        self.documents
            .get(uri)
            .ok_or_else(|| Error::DocumentNotFound(uri.clone()).into())
    }

    /// Get a mutable reference to a document associated with the session, if possible.
    pub async fn get_mut_document(&self, uri: &Url) -> anyhow::Result<RefMut<'_, Url, Document>> {
        self.documents
            .get_mut(uri)
            .ok_or_else(|| Error::DocumentNotFound(uri.clone()).into())
    }
}

#[cfg(test)]
mod tests {
    use super::Session;
    use crate::core::{database::DocumentStatus, document::Document, error::Error};
    use lspower::lsp_types::*;
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
        use super::Session;
        use crate::core::error::Error;
        use lspower::lsp_types::*;

        #[tokio::test]
        async fn get_document() -> anyhow::Result<()> {
            let client = None;
            let session = Session::new(client)?;
            let uri = Url::parse("inmemory:///test")?;
            let result = session.get_document(&uri).await;

            assert!(result.is_err());
            if let Err(error) = result {
                let expected = &Error::DocumentNotFound(uri.clone());
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
            let session = Session::new(client)?;
            let uri = Url::parse("inmemory:///test")?;
            let result = session.get_mut_document(&uri).await;

            assert!(result.is_err());
            if let Err(error) = result {
                let expected = &Error::DocumentNotFound(uri.clone());
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

        let uri = Url::parse("inmemory:///test")?;
        let language_id = "wasm.wast";
        let text = String::new();
        let document = Document::new(language_id, text)?.unwrap();

        session.insert_document(uri.clone(), document)?;

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

        let uri = Url::parse("inmemory:///test")?;
        let language_id = "wasm.wast";
        let text = String::new();
        let document = Document::new(language_id, text)?.unwrap();

        session.insert_document(uri.clone(), document)?;

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
