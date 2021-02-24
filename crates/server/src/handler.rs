//! LSP message handler functions.

/// LSP message handler functions for `textDocument/*`.
pub mod text_document {
    use crate::{core, provider};
    use lsp_text::RopeExt;
    use std::sync::Arc;

    /// LSP message handler function for `textDocument/didChange`.
    pub async fn did_change(
        session: Arc<core::Session>,
        params: lsp::DidChangeTextDocumentParams,
    ) -> anyhow::Result<()> {
        let uri = &params.text_document.uri;
        let mut text = session.get_mut_text(uri).await?;

        let edits = params
            .content_changes
            .iter()
            .map(|change| text.content.build_edit(change))
            .collect::<Result<Vec<_>, _>>()?;

        for edit in &edits {
            text.content.apply_edit(edit);
        }

        if let Some(tree) = core::Document::change(session.clone(), uri, &text.content, &edits).await? {
            let diagnostics = provider::diagnostics(&tree, &text);
            let version = Default::default();
            session
                .client()?
                .publish_diagnostics(uri.clone(), diagnostics, version)
                .await;
        }

        Ok(())
    }

    /// LSP message handler function for `textDocument/didClose`.
    pub async fn did_close(session: Arc<core::Session>, params: lsp::DidCloseTextDocumentParams) -> anyhow::Result<()> {
        let uri = params.text_document.uri;
        session.remove_document(&uri)?;
        let diagnostics = Default::default();
        let version = Default::default();
        session.client()?.publish_diagnostics(uri, diagnostics, version).await;
        Ok(())
    }

    /// LSP message handler function for `textDocument/didOpen`.
    pub async fn did_open(session: Arc<core::Session>, params: lsp::DidOpenTextDocumentParams) -> anyhow::Result<()> {
        let uri = params.text_document.uri.clone();

        if let Some(document) = core::Document::open(params)? {
            let tree = document.tree.clone();
            let text = document.text();
            session.insert_document(uri.clone(), document)?;
            let diagnostics = provider::diagnostics(&tree, &text);
            let version = Default::default();
            session.client()?.publish_diagnostics(uri, diagnostics, version).await;
        } else {
            log::warn!("'textDocument/didOpen' failed :: uri: {:#?}", uri);
        }

        Ok(())
    }

    /// LSP message handler function for `textDocument/documentSymbol`.
    pub async fn document_symbol(
        session: Arc<core::Session>,
        params: lsp::DocumentSymbolParams,
    ) -> anyhow::Result<Option<lsp::DocumentSymbolResponse>> {
        provider::document_symbol(session, params).await
    }

    /// LSP message handler function for `textDocument/semanticTokens/*`.
    pub mod semantic_tokens {
        use crate::{core, provider};
        use std::sync::Arc;

        /// LSP message handler function for `textDocument/semanticTokens/full`.
        pub async fn full(
            session: Arc<core::Session>,
            params: lsp::SemanticTokensParams,
        ) -> anyhow::Result<Option<lsp::SemanticTokensResult>> {
            provider::semantic_tokens::full(session, params).await
        }

        /// LSP message handler function for `textDocument/semanticTokens/range`.
        pub async fn range(
            session: Arc<core::Session>,
            params: lsp::SemanticTokensRangeParams,
        ) -> anyhow::Result<Option<lsp::SemanticTokensRangeResult>> {
            provider::semantic_tokens::range(session, params).await
        }
    }
}
