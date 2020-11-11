//! Definitions for the server instance.

use crate::core::session::Session;
use std::sync::Arc;
use tower_lsp::{lsp_types::*, Client};

/// The WASM language server instance.
pub struct Server {
    /// The LSP client handle.
    pub(crate) client: Client,
    /// The current state of the server.
    pub(crate) session: Arc<Session>,
}

impl Server {
    /// Create a new server.
    pub fn new(client: Client) -> anyhow::Result<Self> {
        let session = Arc::new(Session::new(Some(client.clone()))?);
        Ok(Server { client, session })
    }
}

/// Compute the server capabilities.
pub fn capabilities() -> ServerCapabilities {
    let document_symbol_provider = Some(OneOf::Left(true));

    let hover_provider = Some(HoverProviderCapability::Simple(true));

    let text_document_sync = Some(TextDocumentSyncCapability::Options(TextDocumentSyncOptions {
        open_close: Some(true),
        change: Some(TextDocumentSyncKind::Full),
        ..Default::default()
    }));

    ServerCapabilities {
        document_symbol_provider,
        hover_provider,
        text_document_sync,
        ..Default::default()
    }
}
