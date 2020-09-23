//! Providers of the WebAssembly language server for LSP features.

use crate::core::{language::Language, session::Session};
use std::sync::Arc;
use tower_lsp::lsp_types::*;

// Provides diagnostics functionality.
pub(crate) mod diagnostics;

// Provides `textDocument/documentSymbol` functionality.
pub mod document_symbol;

// Provides `textDocument/hover` functionality.
pub mod hover;

/// Provide response for `textDocument/documentSymbols`.
pub async fn document_symbol(
    session: Arc<Session>,
    params: DocumentSymbolParams,
) -> anyhow::Result<Option<DocumentSymbolResponse>> {
    let DocumentSymbolParams {
        text_document: TextDocumentIdentifier { uri },
        ..
    } = &params;
    let document = session.get_document(uri).await?;
    let response = match document.language {
        Language::Wast => document_symbol::wast::response(&document).await,
        Language::Wat => document_symbol::wast::response(&document).await,
    };
    Ok(response)
}

// FIXME
pub(crate) async fn hover(session: Arc<Session>, params: HoverParams) -> anyhow::Result<Option<Hover>> {
    let HoverParams {
        text_document_position_params:
            TextDocumentPositionParams {
                text_document: TextDocumentIdentifier { uri, .. },
                ..
            },
        ..
    } = &params;
    let document = session.get_document(uri).await?;
    let hover = hover::response(&document, &params).await?;
    Ok(hover)
}
