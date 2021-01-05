//! Definitions for the request handlers.

use crate::{core::error, lsp::server::Server, provider, service::synchronizer};
use lspower::{jsonrpc::Result, LanguageServer};

#[lspower::async_trait]
impl LanguageServer for Server {
    async fn initialize(&self, params: lsp::InitializeParams) -> Result<lsp::InitializeResult> {
        // Receive and store the client capabilities.
        *self.session.client_capabilities.write().await = Some(params.capabilities);
        // Return the server capabilities.
        let capabilities = crate::lsp::server::capabilities();
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

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: lsp::DidOpenTextDocumentParams) {
        let session = self.session.clone();
        synchronizer::document::open(session, params).await.unwrap()
    }

    async fn did_change(&self, params: lsp::DidChangeTextDocumentParams) {
        let session = self.session.clone();
        synchronizer::document::change(session, params).await.unwrap()
    }

    async fn did_close(&self, params: lsp::DidCloseTextDocumentParams) {
        let session = self.session.clone();
        synchronizer::document::close(session, params).await.unwrap()
    }

    async fn document_symbol(&self, params: lsp::DocumentSymbolParams) -> Result<Option<lsp::DocumentSymbolResponse>> {
        let session = self.session.clone();
        let result = provider::document_symbol(session, params).await;
        Ok(result.map_err(error::IntoJsonRpcError)?)
    }

    async fn hover(&self, params: lsp::HoverParams) -> Result<Option<lsp::Hover>> {
        let session = self.session.clone();
        let result = provider::hover(session, params).await;
        Ok(result.map_err(error::IntoJsonRpcError)?)
    }

    async fn semantic_tokens_full(&self, params: lsp::SemanticTokensParams) -> Result<Option<lsp::SemanticTokensResult>> {
        let session = self.session.clone();
        let result = provider::semantic_tokens_full(session, params).await;
        Ok(result.map_err(error::IntoJsonRpcError)?)
    }

    async fn semantic_tokens_range(
        &self,
        params: lsp::SemanticTokensRangeParams,
    ) -> Result<Option<lsp::SemanticTokensRangeResult>> {
        let session = self.session.clone();
        let result = provider::semantic_tokens_range(session, params).await;
        Ok(result.map_err(error::IntoJsonRpcError)?)
    }
}
