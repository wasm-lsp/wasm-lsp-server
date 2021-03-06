//! LSP server capabilities and server instance.

use crate::{core, handler};
use lspower::jsonrpc;
use std::sync::Arc;

/// The WebAssembly language server structure.
pub struct Server {
    /// Reference to the LSP client.
    pub client: lspower::Client,
    /// Reference to the LSP session.
    pub session: Arc<core::Session>,
}

impl Server {
    /// Create a new [`Server`] instance.
    pub fn new(client: lspower::Client) -> anyhow::Result<Self> {
        let session = Arc::new(core::Session::new(Some(client.clone()))?);
        Ok(Server { client, session })
    }
}

/// Convenience function for building [`lsp::ServerCapabilities`] for [Server].
pub fn capabilities() -> lsp::ServerCapabilities {
    let document_symbol_provider = Some(lsp::OneOf::Left(true));

    let semantic_tokens_provider = {
        let token_types = vec![
            lsp::SemanticTokenType::COMMENT,
            lsp::SemanticTokenType::FUNCTION,
            lsp::SemanticTokenType::KEYWORD,
            lsp::SemanticTokenType::NAMESPACE,
            lsp::SemanticTokenType::OPERATOR,
            lsp::SemanticTokenType::PARAMETER,
            lsp::SemanticTokenType::STRING,
            lsp::SemanticTokenType::TYPE,
            lsp::SemanticTokenType::TYPE_PARAMETER,
            lsp::SemanticTokenType::VARIABLE,
        ];
        let token_modifiers = Default::default();

        let options = lsp::SemanticTokensOptions {
            legend: lsp::SemanticTokensLegend {
                token_types,
                token_modifiers,
            },
            range: Some(true),
            full: Some(lsp::SemanticTokensFullOptions::Bool(true)),
            ..Default::default()
        };
        Some(lsp::SemanticTokensServerCapabilities::SemanticTokensOptions(options))
    };

    let text_document_sync = {
        let options = lsp::TextDocumentSyncOptions {
            open_close: Some(true),
            change: Some(lsp::TextDocumentSyncKind::Incremental),
            ..Default::default()
        };
        Some(lsp::TextDocumentSyncCapability::Options(options))
    };

    lsp::ServerCapabilities {
        text_document_sync,
        document_symbol_provider,
        semantic_tokens_provider,
        ..Default::default()
    }
}

#[lspower::async_trait]
impl lspower::LanguageServer for Server {
    async fn initialize(&self, params: lsp::InitializeParams) -> jsonrpc::Result<lsp::InitializeResult> {
        *self.session.client_capabilities.write().await = Some(params.capabilities);
        let capabilities = capabilities();
        Ok(lsp::InitializeResult {
            capabilities,
            ..lsp::InitializeResult::default()
        })
    }

    async fn initialized(&self, _: lsp::InitializedParams) {
        let typ = lsp::MessageType::Info;
        let message = "WebAssembly language server initialized!";
        self.client.log_message(typ, message).await;
    }

    async fn shutdown(&self) -> jsonrpc::Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: lsp::DidOpenTextDocumentParams) {
        let session = self.session.clone();
        handler::text_document::did_open(session, params).await.unwrap()
    }

    async fn did_change(&self, params: lsp::DidChangeTextDocumentParams) {
        let session = self.session.clone();
        handler::text_document::did_change(session, params).await.unwrap()
    }

    async fn did_close(&self, params: lsp::DidCloseTextDocumentParams) {
        let session = self.session.clone();
        handler::text_document::did_close(session, params).await.unwrap()
    }

    async fn document_symbol(
        &self,
        params: lsp::DocumentSymbolParams,
    ) -> jsonrpc::Result<Option<lsp::DocumentSymbolResponse>> {
        let session = self.session.clone();
        let result = handler::text_document::document_symbol(session, params).await;
        Ok(result.map_err(core::IntoJsonRpcError)?)
    }

    async fn semantic_tokens_full(
        &self,
        params: lsp::SemanticTokensParams,
    ) -> jsonrpc::Result<Option<lsp::SemanticTokensResult>> {
        let session = self.session.clone();
        let result = handler::text_document::semantic_tokens::full(session, params).await;
        Ok(result.map_err(core::IntoJsonRpcError)?)
    }

    async fn semantic_tokens_range(
        &self,
        params: lsp::SemanticTokensRangeParams,
    ) -> jsonrpc::Result<Option<lsp::SemanticTokensRangeResult>> {
        let session = self.session.clone();
        let result = handler::text_document::semantic_tokens::range(session, params).await;
        Ok(result.map_err(core::IntoJsonRpcError)?)
    }
}
