//! Definitions for the request handlers.

use crate::{
    core::error,
    lsp::server::Server,
    service::{analyzer, elaborator, synchronizer},
};
use tower_lsp::{jsonrpc::Result, lsp_types::*, LanguageServer};

#[tower_lsp::async_trait]
impl LanguageServer for Server {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        let capabilities = crate::lsp::server::capabilities();
        Ok(InitializeResult {
            capabilities,
            ..InitializeResult::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        let typ = MessageType::Info;
        let message = "WebAssembly language server initialized!";
        self.client.log_message(typ, message).await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let session = self.session.clone();
        synchronizer::document::open(session, params).await.unwrap()
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let session = self.session.clone();
        synchronizer::document::change(session, params).await.unwrap()
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let session = self.session.clone();
        synchronizer::document::close(session, params).await.unwrap()
    }

    async fn document_symbol(&self, params: DocumentSymbolParams) -> Result<Option<DocumentSymbolResponse>> {
        let session = self.session.clone();
        let result = elaborator::document_symbol(session, params).await;
        Ok(result.map_err(error::IntoJsonRpcError)?)
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let session = self.session.clone();
        let result = analyzer::hover(session, params).await;
        Ok(result.map_err(error::IntoJsonRpcError)?)
    }
}
