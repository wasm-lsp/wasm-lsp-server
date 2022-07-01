//! LSP message handler functions.

/// LSP message handler functions for `textDocument/*`.
pub mod text_document;

use std::sync::Arc;

/// LSP message handler function for `initialize`.
pub async fn initialize(session: Arc<crate::core::Session>, params: lsp::InitializeParams) -> lsp::InitializeResult {
    // Received the client capabilities and store them in the server session
    *session.client_capabilities.write().await = Some(params.capabilities);
    // Retrieve the server capabilities for the response to the client
    let capabilities = session.server_capabilities.read().await.clone();
    lsp::InitializeResult {
        capabilities,
        ..lsp::InitializeResult::default()
    }
}
