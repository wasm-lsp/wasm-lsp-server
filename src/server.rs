use crate::session::Session;
use jsonrpc_core::Result;
use log;
use tower_lsp::{lsp_types::*, Client, LanguageServer};

#[tower_lsp::async_trait]
impl LanguageServer for Session {
    fn initialize(&self, _: &Client, params: InitializeParams) -> Result<InitializeResult> {
        log::info!("{:?}", params);
        // let document_symbol_provider = Some(false);
        // let semantic_tokens_provider = Some(SemanticTokensServerCapabilities::SemanticTokensRegistrationOptions(
        //     SemanticTokensRegistrationOptions {
        //         semantic_tokens_options: SemanticTokensOptions {
        //             document_provider: Some(SemanticTokensDocumentProvider::Bool(false)),
        //             legend: SemanticTokensLegend {
        //                 token_modifiers: vec![],
        //                 token_types: vec![],
        //             },
        //             range_provider: Some(false),
        //             work_done_progress_options: WorkDoneProgressOptions {
        //                 work_done_progress: Some(false),
        //             },
        //         },
        //         static_registration_options: StaticRegistrationOptions { id: None },
        //         text_document_registration_options: TextDocumentRegistrationOptions {
        //             document_selector: None,
        //         },
        //     },
        // ));
        let text_document_sync = Some(TextDocumentSyncCapability::Options(TextDocumentSyncOptions {
            open_close: Some(true),
            change: Some(TextDocumentSyncKind::Full),
            ..TextDocumentSyncOptions::default()
        }));
        // let workspace = Some(WorkspaceCapability {
        //     workspace_folders: Some(WorkspaceFolderCapability {
        //         supported: Some(false),
        //         change_notifications: Some(WorkspaceFolderCapabilityChangeNotifications::Bool(false)),
        //     }),
        // });
        // let workspace_symbol_provider = Some(false);
        let capabilities = ServerCapabilities {
            // document_symbol_provider,
            // semantic_tokens_provider,
            text_document_sync,
            // workspace,
            // workspace_symbol_provider,
            ..ServerCapabilities::default()
        };
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
        self.synchronizer.did_open(client, params).await
    }

    async fn did_change(&self, client: &Client, params: DidChangeTextDocumentParams) {
        self.synchronizer.did_change(client, params).await
    }

    async fn did_close(&self, client: &Client, params: DidCloseTextDocumentParams) {
        self.synchronizer.did_close(client, params).await
    }
}
