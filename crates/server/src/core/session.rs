use crate::{
    core::{document::Document, error::Error},
    server,
};
use dashmap::{
    mapref::one::{Ref, RefMut},
    DashMap,
};

#[cfg(feature = "runtime-agnostic")]
use async_lock::{Mutex, RwLock};
#[cfg(feature = "tokio")]
use tokio::sync::{Mutex, RwLock};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SessionResourceKind {
    Document,
    Parser,
    Tree,
}

pub struct Session {
    pub(crate) server_capabilities: RwLock<lsp::ServerCapabilities>,
    pub(crate) client_capabilities: RwLock<Option<lsp::ClientCapabilities>>,
    client: Option<lspower::Client>,
    documents: DashMap<lsp::Url, Document>,
    pub(crate) parsers: DashMap<lsp::Url, Mutex<tree_sitter::Parser>>,
    pub(crate) trees: DashMap<lsp::Url, Mutex<tree_sitter::Tree>>,
}

impl Session {
    pub fn new(client: Option<lspower::Client>) -> anyhow::Result<Self> {
        let server_capabilities = RwLock::new(server::capabilities());
        let client_capabilities = RwLock::new(Default::default());
        let documents = DashMap::new();
        let parsers = DashMap::new();
        let trees = DashMap::new();
        Ok(Session {
            server_capabilities,
            client_capabilities,
            client,
            documents,
            parsers,
            trees,
        })
    }

    pub(crate) fn client(&self) -> anyhow::Result<&lspower::Client> {
        self.client.as_ref().ok_or_else(|| Error::ClientNotInitialized.into())
    }

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
        Ok(())
    }

    pub fn remove_document(&self, uri: &lsp::Url) -> anyhow::Result<()> {
        let result = self.documents.remove(uri);
        debug_assert!(result.is_some());
        let result = self.parsers.remove(uri);
        debug_assert!(result.is_some());
        let result = self.trees.remove(uri);
        debug_assert!(result.is_some());
        Ok(())
    }

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

    pub async fn get_document(&self, uri: &lsp::Url) -> anyhow::Result<Ref<'_, lsp::Url, Document>> {
        self.documents.get(uri).ok_or_else(|| {
            let kind = SessionResourceKind::Document;
            let uri = uri.clone();
            Error::SessionResourceNotFound { kind, uri }.into()
        })
    }

    pub async fn get_mut_document(&self, uri: &lsp::Url) -> anyhow::Result<RefMut<'_, lsp::Url, Document>> {
        self.documents.get_mut(uri).ok_or_else(|| {
            let kind = SessionResourceKind::Document;
            let uri = uri.clone();
            Error::SessionResourceNotFound { kind, uri }.into()
        })
    }

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

    pub async fn get_tree(&self, uri: &lsp::Url) -> anyhow::Result<Ref<'_, lsp::Url, Mutex<tree_sitter::Tree>>> {
        self.trees.get(uri).ok_or_else(|| {
            let kind = SessionResourceKind::Tree;
            let uri = uri.clone();
            Error::SessionResourceNotFound { kind, uri }.into()
        })
    }

    pub async fn get_mut_tree(&self, uri: &lsp::Url) -> anyhow::Result<RefMut<'_, lsp::Url, Mutex<tree_sitter::Tree>>> {
        self.trees.get_mut(uri).ok_or_else(|| {
            let kind = SessionResourceKind::Tree;
            let uri = uri.clone();
            Error::SessionResourceNotFound { kind, uri }.into()
        })
    }
}
