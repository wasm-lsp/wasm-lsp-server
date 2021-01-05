//! Providers of the WebAssembly language server for LSP features.

use crate::core::{language::Language, session::Session};
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
    params: lsp::DocumentSymbolParams,
) -> anyhow::Result<Option<lsp::DocumentSymbolResponse>> {
    let lsp::DocumentSymbolParams {
        text_document: lsp::TextDocumentIdentifier { uri },
        ..
    } = &params;
    let document = session.get_document(uri).await?;
    let response = match document.language {
        Language::Wast => document_symbol::wast::response(&document).await,
        Language::Wat => document_symbol::wat::response(&document).await,
    };
    Ok(response)
}

// FIXME
pub(crate) async fn hover(session: Arc<Session>, params: lsp::HoverParams) -> anyhow::Result<Option<lsp::Hover>> {
    let lsp::HoverParams {
        text_document_position_params:
        lsp::TextDocumentPositionParams {
                text_document: lsp::TextDocumentIdentifier { uri, .. },
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
    params: lsp::SemanticTokensParams,
) -> anyhow::Result<Option<lsp::SemanticTokensResult>> {
    let document = session.get_document(&params.text_document.uri).await?;
    let response = match document.language {
        Language::Wast => semantic_tokens::wast::full(session.clone(), &document, params).await?,
        Language::Wat => semantic_tokens::wat::full(session.clone(), &document, params).await?,
    };
    Ok(response)
}

pub(crate) async fn semantic_tokens_range(
    session: Arc<Session>,
    params: lsp::SemanticTokensRangeParams,
) -> anyhow::Result<Option<lsp::SemanticTokensRangeResult>> {
    let document = session.get_document(&params.text_document.uri).await?;
    let response = match document.language {
        Language::Wast => semantic_tokens::wast::range(session.clone(), &document, params).await?,
        Language::Wat => semantic_tokens::wat::range(session.clone(), &document, params).await?,
    };
    Ok(response)
}
