use crate::core::{self, Language};
use std::sync::Arc;

/// Definitions for the semantic tokens builder used during tokenization.
pub mod builder;

pub mod wast;
pub mod wat;

/// LSP message handler function for `textDocument/semanticTokens/full`.
pub async fn full(
    session: Arc<core::Session>,
    params: lsp::SemanticTokensParams,
) -> anyhow::Result<Option<lsp::SemanticTokensResult>> {
    let text = session.get_text(&params.text_document.uri).await?;
    let response = match text.language {
        Language::Wast => wast::full(session.clone(), params, &text.content).await?,
        Language::Wat => wat::full(session.clone(), params, &text.content).await?,
    };
    Ok(response)
}

/// LSP message handler function for `textDocument/semanticTokens/range`.
pub async fn range(
    session: Arc<core::Session>,
    params: lsp::SemanticTokensRangeParams,
) -> anyhow::Result<Option<lsp::SemanticTokensRangeResult>> {
    let text = session.get_text(&params.text_document.uri).await?;
    let response = match text.language {
        Language::Wast => wast::range(session.clone(), params, &text.content).await?,
        Language::Wat => wat::range(session.clone(), params, &text.content).await?,
    };
    Ok(response)
}
