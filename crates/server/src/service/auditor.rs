//! Collects diagnostics for documents with syntax and validation errors.

/// Functions related to processing parse tree events for a document.
pub(crate) mod tree {
    use crate::core::{document::Document, session::Session};
    use std::sync::Arc;
    use tower_lsp::lsp_types::*;

    fn diagnostics_for_error(document: &Document, diagnostics: &mut Vec<Diagnostic>, error: wast::Error) {
        let input = &document.text;
        diagnostics.push({
            let span = error.span();
            let (line, col) = span.linecol_in(input);
            let range = {
                let start = Position::new(line as u64, col as u64);
                let end = Position::new(line as u64, col as u64);
                Range::new(start, end)
            };
            let severity = Some(DiagnosticSeverity::Error);
            let code = Default::default();
            let source = Some(String::from("wast"));
            let message = error.message();
            let related_information = Default::default();
            let tags = Default::default();
            Diagnostic::new(range, severity, code, source, message, related_information, tags)
        });
    }

    // Compute diagnostics for a change event given a `document` and `tree`.
    // NOTE: Currently we only use the tree-sitter grammar to check for the
    // presence of errors and then use the `wast` crate for the actual error
    // reporting (because tree-sitter does not provide detailed errors yet).
    fn diagnostics_for_change(document: &Document, tree: tree_sitter::Tree) -> anyhow::Result<Vec<Diagnostic>> {
        let mut diagnostics = vec![];

        if tree.root_node().has_error() {
            match ::wast::parser::ParseBuffer::new(&document.text) {
                Err(error) => {
                    diagnostics_for_error(document, &mut diagnostics, error);
                },
                Ok(buffer) => {
                    if let Err(error) = ::wast::parser::parse::<::wast::Wast>(&buffer) {
                        diagnostics_for_error(document, &mut diagnostics, error);
                    }
                },
            }
        } else {
            // NOTE: else let elaborator handle
        }

        Ok(diagnostics)
    }

    /// Handle a parse tree "change" event.
    pub(crate) async fn change(session: Arc<Session>, uri: Url) -> anyhow::Result<()> {
        let document = session.get_document(&uri).await?;
        let tree = document.tree.lock().await.clone();
        let diagnostics = diagnostics_for_change(&document, tree)?;
        let version = Default::default();
        session.client()?.publish_diagnostics(uri, diagnostics, version).await;
        Ok(())
    }

    /// Handle a parse tree "close" event.
    pub(crate) async fn close(session: Arc<Session>, uri: Url) -> anyhow::Result<()> {
        // clear diagnostics on tree close
        // FIXME: handle this properly
        let diagnostics = Default::default();
        let version = Default::default();
        session.client()?.publish_diagnostics(uri, diagnostics, version).await;
        Ok(())
    }

    /// Handle a parse tree "open" event.
    pub(crate) async fn open(session: Arc<Session>, uri: Url) -> anyhow::Result<()> {
        self::change(session, uri).await
    }
}
