//! Synchronizes document changes between editor and server

/// Functions related to processing events for a document.
pub(crate) mod document {
    use crate::core;
    use std::sync::Arc;

    /// Handle a document "change" event.
    pub(crate) async fn change(
        _session: Arc<core::Session>,
        _params: lsp::DidChangeTextDocumentParams,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    /// Handle a document "close" event.
    pub(crate) async fn close(
        _session: Arc<core::Session>,
        _params: lsp::DidCloseTextDocumentParams,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    /// Handle a document "open" event.
    pub(crate) async fn open(
        _session: Arc<core::Session>,
        _params: lsp::DidOpenTextDocumentParams,
    ) -> anyhow::Result<()> {
        Ok(())
    }
}
