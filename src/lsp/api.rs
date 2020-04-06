//! Definitions for the request handlers.

use crate::lsp::server::Server;
use jsonrpc_core::Result;
use log;
use lsp_types::*;
use tower_lsp::{Client, LanguageServer};

#[tower_lsp::async_trait]
impl LanguageServer for Server {
    fn initialize(&self, _: &Client, params: InitializeParams) -> Result<InitializeResult> {
        log::info!("{:?}", params);
        let capabilities = crate::lsp::capabilities::capabilities();
        Ok(InitializeResult {
            capabilities,
            ..InitializeResult::default()
        })
    }

    async fn initialized(&self, client: &Client, _: InitializedParams) {
        client.log_message(MessageType::Info, "server initialized!");
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    // FIXME: remove on next tower-lsp release
    #[allow(unsafe_code)]
    async fn did_open(&self, client: &Client, params: DidOpenTextDocumentParams) {
        let client: &'static Client = unsafe { std::mem::transmute(client) };
        crate::service::synchronizer::document::open(self.session.clone(), client, params)
            .await
            .unwrap()
    }

    // FIXME: remove on next tower-lsp release
    #[allow(unsafe_code)]
    async fn did_change(&self, client: &Client, params: DidChangeTextDocumentParams) {
        let client: &'static Client = unsafe { std::mem::transmute(client) };
        crate::service::synchronizer::document::change(self.session.clone(), client, params)
            .await
            .unwrap()
    }

    // FIXME: remove on next tower-lsp release
    #[allow(unsafe_code)]
    async fn did_close(&self, client: &Client, params: DidCloseTextDocumentParams) {
        let client: &'static Client = unsafe { std::mem::transmute(client) };
        crate::service::synchronizer::document::close(self.session.clone(), client, params)
            .await
            .unwrap()
    }

    async fn document_symbol(&self, params: DocumentSymbolParams) -> Result<Option<DocumentSymbolResponse>> {
        let result = crate::service::elaborator::document_symbol(self.session.clone(), params).await;
        Ok(result.map_err(crate::core::error::IntoJsonRpcError)?)
    }
}
