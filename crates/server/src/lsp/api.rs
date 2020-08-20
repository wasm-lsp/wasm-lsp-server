//! Definitions for the request handlers.

use crate::lsp::server::Server;
use tower_lsp::{jsonrpc::Result, lsp_types::*, LanguageServer};

#[tower_lsp::async_trait]
impl LanguageServer for Server {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        log::info!("{:?}", params);
        let capabilities = crate::lsp::capabilities::capabilities();
        Ok(InitializeResult {
            capabilities,
            ..InitializeResult::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client.log_message(MessageType::Info, "server initialized!").await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        crate::service::synchronizer::document::open(self.session.clone(), params)
            .await
            .unwrap()
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        crate::service::synchronizer::document::change(self.session.clone(), params)
            .await
            .unwrap()
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        crate::service::synchronizer::document::close(self.session.clone(), params)
            .await
            .unwrap()
    }

    async fn document_symbol(&self, params: DocumentSymbolParams) -> Result<Option<DocumentSymbolResponse>> {
        let result = crate::service::elaborator::document_symbol(self.session.clone(), params).await;
        Ok(result.map_err(crate::core::error::IntoJsonRpcError)?)
    }
}
