//! Definitions for the server instance.

use crate::core::session::Session;
use lspower::{lsp_types::*, Client};
use std::sync::Arc;

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

    let semantic_tokens_provider = {
        let options = SemanticTokensOptions {
            work_done_progress_options: Default::default(),
            legend: SemanticTokensLegend {
                token_types: Default::default(),
                token_modifiers: Default::default(),
            },
            range: Default::default(),
            full: Some(SemanticTokensFullOptions::Bool(true)),
        };
        Some(SemanticTokensServerCapabilities::SemanticTokensOptions(options))
    };

    let text_document_sync = {
        let options = TextDocumentSyncOptions {
            open_close: Some(true),
            change: Some(TextDocumentSyncKind::Full),
            ..Default::default()
        };
        Some(TextDocumentSyncCapability::Options(options))
    };

    ServerCapabilities {
        document_symbol_provider,
        hover_provider,
        semantic_tokens_provider,
        text_document_sync,
        ..Default::default()
    }
}
