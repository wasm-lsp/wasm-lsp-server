//! Provides `textDocument/documentSymbol` functionality.

use crate::core::document::Document;
use tower_lsp::lsp_types::*;

fn for_error(document: &Document, error: wast::Error) -> Diagnostic {
    let range = {
        let input = &document.text;
        let span = error.span();
        let (line, col) = span.linecol_in(input);
        // NOTE: wast only gives us the start position so we use that twice
        let pos = Position::new(line as u64, col as u64);
        Range::new(pos, pos)
    };
    let severity = Some(DiagnosticSeverity::Error);
    let code = Default::default();
    let source = Some(String::from("wast"));
    let message = error.message();
    let related_information = Default::default();
    let tags = Default::default();
    Diagnostic::new(range, severity, code, source, message, related_information, tags)
}

// Compute diagnostics for a change event given a `document` and `tree`.
// NOTE: Currently we only use the tree-sitter grammar to check for the
// presence of errors and then use the `wast` crate for the actual error
// reporting (because tree-sitter does not provide detailed errors yet).
fn for_change(document: &Document, tree: tree_sitter::Tree) -> anyhow::Result<Vec<Diagnostic>> {
    let mut diagnostics = vec![];
    if tree.root_node().has_error() || cfg!(debug_assertions) {
        match ::wast::parser::ParseBuffer::new(&document.text) {
            Err(error) => {
                diagnostics.push(super::diagnostics::for_error(document, error));
            },
            Ok(buffer) => {
                if let Err(error) = ::wast::parser::parse::<::wast::Wast>(&buffer) {
                    diagnostics.push(super::diagnostics::for_error(document, error));
                }
            },
        }
    }
    Ok(diagnostics)
}

/// Functions related to processing parse tree events for a document.
pub(crate) mod tree {
    use crate::core::session::Session;
    use std::sync::Arc;
    use tower_lsp::lsp_types::*;

    /// Handle a parse tree "change" event.
    pub(crate) async fn change(session: Arc<Session>, uri: Url) -> anyhow::Result<()> {
        let document = session.get_document(&uri).await?;
        let tree = document.tree.lock().await.clone();
        let diagnostics = super::for_change(&document, tree)?;
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
