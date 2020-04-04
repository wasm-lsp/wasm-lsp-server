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

    async fn did_open(&self, client: &Client, params: DidOpenTextDocumentParams) {
        let documents = self.session.documents.clone();
        // FIXME: remove on next tower-lsp release
        let client: &'static Client = unsafe { std::mem::transmute(client) };
        crate::service::synchronizer::tasks_did_open(documents, client, params)
            .await
            .unwrap()
    }

    async fn did_change(&self, client: &Client, params: DidChangeTextDocumentParams) {
        let documents = self.session.documents.clone();
        // FIXME: remove on next tower-lsp release
        let client: &'static Client = unsafe { std::mem::transmute(client) };
        crate::service::synchronizer::tasks_did_change(documents, client, params)
            .await
            .unwrap()
    }

    async fn did_close(&self, client: &Client, params: DidCloseTextDocumentParams) {
        let documents = self.session.documents.clone();
        // FIXME: remove on next tower-lsp release
        let client: &'static Client = unsafe { std::mem::transmute(client) };
        crate::service::synchronizer::tasks_did_close(documents, client, params)
            .await
            .unwrap()
    }

    async fn document_symbol(&self, params: DocumentSymbolParams) -> Result<Option<DocumentSymbolResponse>> {
        crate::service::elaborator::document_symbol(self.session.documents.clone(), params).await
    }
}
