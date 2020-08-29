//! Definitions for the server feature capabilities.

use tower_lsp::lsp_types::*;

/// Compute the server capabilities.
pub(crate) fn capabilities() -> ServerCapabilities {
    let document_symbol_provider = Some(true);

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
