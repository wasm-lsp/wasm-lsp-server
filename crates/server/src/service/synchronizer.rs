pub(crate) mod document {
    use crate::core;
    use std::sync::Arc;

    pub(crate) async fn change(
        _session: Arc<core::Session>,
        _params: lsp::DidChangeTextDocumentParams,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    pub(crate) async fn close(
        _session: Arc<core::Session>,
        _params: lsp::DidCloseTextDocumentParams,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    pub(crate) async fn open(
        _session: Arc<core::Session>,
        _params: lsp::DidOpenTextDocumentParams,
    ) -> anyhow::Result<()> {
        Ok(())
    }
}
