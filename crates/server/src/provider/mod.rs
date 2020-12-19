//! Providers of the WebAssembly language server for LSP features.

use crate::core::{language::Language, session::Session};
use lspower::lsp_types::*;
use std::sync::Arc;

// Provides diagnostics functionality.
pub(crate) mod diagnostics;

// Provides `textDocument/documentSymbol` functionality.
pub mod document_symbol;

// Provides `textDocument/hover` functionality.
pub mod hover;

// Provides `textDocument/semanticTokens/*` functionality.
pub mod semantic_tokens;

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
    let response = hover::response(&document, &params).await?;
    Ok(response)
}

// FIXME
pub(crate) async fn semantic_tokens_full(
    session: Arc<Session>,
    params: SemanticTokensParams,
) -> anyhow::Result<Option<SemanticTokensResult>> {
    let document = session.get_document(&params.text_document.uri).await?;
    let response = semantic_tokens::full::response(session.clone(), &document, params).await?;
    Ok(response)
}

pub(crate) async fn semantic_tokens_range(
    session: Arc<Session>,
    params: SemanticTokensRangeParams,
) -> anyhow::Result<Option<SemanticTokensRangeResult>> {
    let document = session.get_document(&params.text_document.uri).await?;
    let response = semantic_tokens::range::response(session.clone(), &document, params).await?;
    Ok(response)
}
