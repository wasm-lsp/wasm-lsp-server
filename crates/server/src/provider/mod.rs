//! Providers of the WebAssembly language server for LSP features.

use crate::core::{self, language::Language};
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
    session: Arc<core::Session>,
    params: lsp::DocumentSymbolParams,
) -> anyhow::Result<Option<lsp::DocumentSymbolResponse>> {
    let document = session.get_document(&params.text_document.uri).await?;
    let response = match document.language {
        Language::Wast => document_symbol::wast::response(session.clone(), params, &document).await,
        Language::Wat => document_symbol::wat::response(session.clone(), params, &document).await,
    }?;
    Ok(response)
}

// FIXME
pub(crate) async fn hover(session: Arc<core::Session>, params: lsp::HoverParams) -> anyhow::Result<Option<lsp::Hover>> {
    let response = hover::response(session, params).await?;
    Ok(response)
}

// FIXME
pub(crate) async fn semantic_tokens_full(
    session: Arc<core::Session>,
    params: lsp::SemanticTokensParams,
) -> anyhow::Result<Option<lsp::SemanticTokensResult>> {
    let document = session.get_document(&params.text_document.uri).await?;
    let response = match document.language {
        Language::Wast => semantic_tokens::wast::full(session.clone(), params, &document).await?,
        Language::Wat => semantic_tokens::wat::full(session.clone(), params, &document).await?,
    };
    Ok(response)
}

pub(crate) async fn semantic_tokens_range(
    session: Arc<core::Session>,
    params: lsp::SemanticTokensRangeParams,
) -> anyhow::Result<Option<lsp::SemanticTokensRangeResult>> {
    let document = session.get_document(&params.text_document.uri).await?;
    let response = match document.language {
        Language::Wast => semantic_tokens::wast::range(session.clone(), params, &document).await?,
        Language::Wat => semantic_tokens::wat::range(session.clone(), params, &document).await?,
    };
    Ok(response)
}
