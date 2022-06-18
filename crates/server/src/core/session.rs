//! Definitions related to the LSP session.

use crate::{core, server};
use dashmap::{
    mapref::one::{Ref, RefMut},
    DashMap,
};

#[cfg(feature = "runtime-agnostic")]
use async_lock::{Mutex, RwLock};
#[cfg(feature = "tokio")]
use tokio::sync::{Mutex, RwLock};

/// The LSP server session. This contains the relevant state for workspace.
pub struct Session {
    /// The pre-loaded tree-sitter languages for `.wast` and `.wat`.
    pub languages: SessionLanguages,
    /// The current server LSP capabilities configuration.
    pub server_capabilities: RwLock<lsp::ServerCapabilities>,
    /// The current client LSP capabilities configuration.
    pub client_capabilities: RwLock<Option<lsp::ClientCapabilities>>,
    client: Option<tower_lsp::Client>,
    texts: DashMap<lsp::Url, core::Text>,
    parsers: DashMap<lsp::Url, Mutex<tree_sitter::Parser>>,
    trees: DashMap<lsp::Url, Mutex<tree_sitter::Tree>>,
}

impl Session {
    /// Create a new [`Session`].
    pub fn new(languages: SessionLanguages, client: Option<tower_lsp::Client>) -> anyhow::Result<Self> {
        let server_capabilities = RwLock::new(server::capabilities());
        let client_capabilities = RwLock::new(Default::default());
        let texts = DashMap::new();
        let parsers = DashMap::new();
        let trees = DashMap::new();
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
            .ok_or_else(|| core::Error::ClientNotInitialized.into())
    }

    /// Insert a [`core::Document`] into the [`Session`].
    pub fn insert_document(&self, uri: lsp::Url, document: core::Document) -> anyhow::Result<()> {
        let result = self.texts.insert(uri.clone(), document.text());
        debug_assert!(result.is_none());
        let result = self.parsers.insert(uri.clone(), Mutex::new(document.parser));
        debug_assert!(result.is_none());
        let result = self.trees.insert(uri, Mutex::new(document.tree));
        debug_assert!(result.is_none());
        Ok(())
    }

    /// Remove a [`core::Document`] from the [`Session`].
    pub fn remove_document(&self, uri: &lsp::Url) -> anyhow::Result<()> {
        let result = self.texts.remove(uri);
        debug_assert!(result.is_some());
        let result = self.parsers.remove(uri);
        debug_assert!(result.is_some());
        let result = self.trees.remove(uri);
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

    /// Get a reference to the [`core::Text`] for a [`core::Document`] in the [`Session`].
    pub async fn get_text(&self, uri: &lsp::Url) -> anyhow::Result<Ref<'_, lsp::Url, core::Text>> {
        self.texts.get(uri).ok_or_else(|| {
            let kind = SessionResourceKind::Document;
            let uri = uri.clone();
            core::Error::SessionResourceNotFound { kind, uri }.into()
        })
    }

    /// Get a mutable reference to the [`core::Text`] for a [`core::Document`] in the [`Session`].
    pub async fn get_mut_text(&self, uri: &lsp::Url) -> anyhow::Result<RefMut<'_, lsp::Url, core::Text>> {
        self.texts.get_mut(uri).ok_or_else(|| {
            let kind = SessionResourceKind::Document;
            let uri = uri.clone();
            core::Error::SessionResourceNotFound { kind, uri }.into()
        })
    }

    /// Get a mutable reference to the [`tree_sitter::Parser`] for a [`core::Document`] in the
    /// [`Session`].
    pub async fn get_mut_parser(
        &self,
        uri: &lsp::Url,
    ) -> anyhow::Result<RefMut<'_, lsp::Url, Mutex<tree_sitter::Parser>>> {
        self.parsers.get_mut(uri).ok_or_else(|| {
            let kind = SessionResourceKind::Parser;
            let uri = uri.clone();
            core::Error::SessionResourceNotFound { kind, uri }.into()
        })
    }

    /// Get a reference to the [`tree_sitter::Tree`] for a [`core::Document`] in the [`Session`].
    pub async fn get_tree(&self, uri: &lsp::Url) -> anyhow::Result<Ref<'_, lsp::Url, Mutex<tree_sitter::Tree>>> {
        self.trees.get(uri).ok_or_else(|| {
            let kind = SessionResourceKind::Tree;
            let uri = uri.clone();
            core::Error::SessionResourceNotFound { kind, uri }.into()
        })
    }

    /// Get a mutable reference to the [`tree_sitter::Tree`] for a [`core::Document`] in the
    /// [`Session`].
    pub async fn get_mut_tree(&self, uri: &lsp::Url) -> anyhow::Result<RefMut<'_, lsp::Url, Mutex<tree_sitter::Tree>>> {
        self.trees.get_mut(uri).ok_or_else(|| {
            let kind = SessionResourceKind::Tree;
            let uri = uri.clone();
            core::Error::SessionResourceNotFound { kind, uri }.into()
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
    /// A tag representing a [`core::Document`].
    Document,
    /// A tag representing a [`tree_sitter::Parser`].
    Parser,
    /// A tag representing a [`tree_sitter::Tree`].
    Tree,
}
