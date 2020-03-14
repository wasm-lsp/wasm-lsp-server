use crate::session::Session;
use jsonrpc_core::Result;
use log;
use tower_lsp::{lsp_types::*, Client, LanguageServer};

#[tower_lsp::async_trait]
impl LanguageServer for Session {
    fn initialize(&self, _: &Client, data: InitializeParams) -> Result<InitializeResult> {
        log::info!("{:?}", data);
        let capabilities = ServerCapabilities {
            document_symbol_provider: Some(true),
            text_document_sync: Some(TextDocumentSyncCapability::Options(TextDocumentSyncOptions {
                open_close: Some(true),
                change: Some(TextDocumentSyncKind::Full),
                ..TextDocumentSyncOptions::default()
            })),
            workspace: Some(WorkspaceCapability {
                workspace_folders: Some(WorkspaceFolderCapability {
                    supported: Some(true),
                    change_notifications: Some(WorkspaceFolderCapabilityChangeNotifications::Bool(true)),
                }),
            }),
            workspace_symbol_provider: Some(true),
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
}
