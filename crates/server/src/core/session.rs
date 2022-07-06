//! Definitions related to the LSP session.

use crate::core::MapExt;
use async_lock::{Mutex, RwLock};

/// The LSP server session. This contains the relevant state for workspace.
pub struct Session {
    /// The pre-loaded tree-sitter languages for `.wast` and `.wat`.
    pub languages: SessionLanguages,
    /// The current server LSP capabilities configuration.
    pub server_capabilities: RwLock<lsp::ServerCapabilities>,
    /// The current client LSP capabilities configuration.
    pub client_capabilities: RwLock<Option<lsp::ClientCapabilities>>,
    client: Option<tower_lsp::Client>,
    texts: crate::core::Map<lsp::Url, crate::core::Text>,
    parsers: crate::core::Map<lsp::Url, Mutex<tree_sitter::Parser>>,
    trees: crate::core::Map<lsp::Url, Mutex<tree_sitter::Tree>>,
}

impl Session {
    /// Create a new [`Session`].
    pub fn new(languages: SessionLanguages, client: Option<tower_lsp::Client>) -> anyhow::Result<Self> {
        let server_capabilities = RwLock::new(crate::Server::capabilities());
        let client_capabilities = RwLock::new(Default::default());
        let texts = crate::core::Map::default();
        let parsers = crate::core::Map::default();
        let trees = crate::core::Map::default();
        Ok(Session {
            languages,
            server_capabilities,
            client_capabilities,
            client,
            texts,
            parsers,
            trees,
        })
    }

    /// Retrieve the handle for the LSP client.
    pub fn client(&self) -> anyhow::Result<&tower_lsp::Client> {
        self.client
            .as_ref()
            .ok_or_else(|| crate::core::Error::ClientNotInitialized.into())
    }

    /// Insert a [`crate::core::Document`] into the [`Session`].
    pub async fn insert_document(&self, uri: lsp::Url, document: crate::core::Document) -> anyhow::Result<()> {
        let result = self.texts.insert(uri.clone(), document.text()).await;
        debug_assert!(result.is_none());
        let result = self.parsers.insert(uri.clone(), Mutex::new(document.parser)).await;
        debug_assert!(result.is_none());
        let result = self.trees.insert(uri, Mutex::new(document.tree)).await;
        debug_assert!(result.is_none());
        Ok(())
    }

    /// Remove a [`crate::core::Document`] from the [`Session`].
    pub async fn remove_document(&self, uri: &lsp::Url) -> anyhow::Result<()> {
        let result = self.texts.remove(uri).await;
        debug_assert!(result.is_some());
        let result = self.parsers.remove(uri).await;
        debug_assert!(result.is_some());
        let result = self.trees.remove(uri).await;
        debug_assert!(result.is_some());
        Ok(())
    }

    /// Retrieve the LSP semantic tokens legend.
    pub async fn semantic_tokens_legend(&self) -> Option<lsp::SemanticTokensLegend> {
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

    /// Get a reference to the [`crate::core::Text`] for a [`crate::core::Document`] in the
    /// [`Session`].
    pub async fn get_text<'a>(
        &'a self,
        uri: &'a lsp::Url,
    ) -> anyhow::Result<crate::core::Ref<'a, lsp::Url, crate::core::Text>> {
        self.texts.get(uri).await.ok_or_else(|| {
            let kind = SessionResourceKind::Document;
            let uri = uri.clone();
            crate::core::Error::SessionResourceNotFound { kind, uri }.into()
        })
    }

    /// Get a mutable reference to the [`crate::core::Text`] for a [`crate::core::Document`] in the
    /// [`Session`].
    pub async fn get_mut_text<'a>(
        &'a self,
        uri: &'a lsp::Url,
    ) -> anyhow::Result<crate::core::RefMut<'a, lsp::Url, crate::core::Text>> {
        self.texts.get_mut(uri).await.ok_or_else(|| {
            let kind = SessionResourceKind::Document;
            let uri = uri.clone();
            crate::core::Error::SessionResourceNotFound { kind, uri }.into()
        })
    }

    /// Get a mutable reference to the [`tree_sitter::Parser`] for a [`crate::core::Document`] in
    /// the [`Session`].
    pub async fn get_mut_parser<'a>(
        &'a self,
        uri: &'a lsp::Url,
    ) -> anyhow::Result<crate::core::RefMut<'a, lsp::Url, Mutex<tree_sitter::Parser>>> {
        self.parsers.get_mut(uri).await.ok_or_else(|| {
            let kind = SessionResourceKind::Parser;
            let uri = uri.clone();
            crate::core::Error::SessionResourceNotFound { kind, uri }.into()
        })
    }

    /// Get a reference to the [`tree_sitter::Tree`] for a [`crate::core::Document`] in the
    /// [`Session`].
    pub async fn get_tree<'a>(
        &'a self,
        uri: &'a lsp::Url,
    ) -> anyhow::Result<crate::core::Ref<'a, lsp::Url, Mutex<tree_sitter::Tree>>> {
        self.trees.get(uri).await.ok_or_else(|| {
            let kind = SessionResourceKind::Tree;
            let uri = uri.clone();
            crate::core::Error::SessionResourceNotFound { kind, uri }.into()
        })
    }

    /// Get a mutable reference to the [`tree_sitter::Tree`] for a [`crate::core::Document`] in the
    /// [`Session`].
    pub async fn get_mut_tree<'a>(
        &'a self,
        uri: &'a lsp::Url,
    ) -> anyhow::Result<crate::core::RefMut<'a, lsp::Url, Mutex<tree_sitter::Tree>>> {
        self.trees.get_mut(uri).await.ok_or_else(|| {
            let kind = SessionResourceKind::Tree;
            let uri = uri.clone();
            crate::core::Error::SessionResourceNotFound { kind, uri }.into()
        })
    }
}

/// Pre-loaded tree-sitter languages for `.wast` and `.wat`.
pub struct SessionLanguages {
    /// Pre-loaded tree-sitter language for `.wast`.
    pub wast: tree_sitter::Language,
    /// Pre-loaded tree-sitter language for `.wat`.
    pub wat: tree_sitter::Language,
}

/// A tag representing of the kinds of session resource.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SessionResourceKind {
    /// A tag representing a [`crate::core::Document`].
    Document,
    /// A tag representing a [`tree_sitter::Parser`].
    Parser,
    /// A tag representing a [`tree_sitter::Tree`].
    Tree,
}
