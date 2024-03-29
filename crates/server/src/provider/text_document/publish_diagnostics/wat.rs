//! Provider definitions for LSP `textDocument/publishDiagnostics` for `.wat` documents.

use crate::core::{self, node::TraceNodeWalker, range::RangeExt};
use lsp_text::RopeExt;

/// Provider function for LSP `textDocument/publishDiagnostics` for `.wat` documents.
pub fn diagnostics(tree: &tree_sitter::Tree, content: &ropey::Rope) -> Vec<lsp::Diagnostic> {
    let mut diagnostics = vec![];
    let mut walker = {
        let language = core::Language::Wat;
        let node = tree.root_node();
        TraceNodeWalker::new(language, node)
    };

    let mut previous = walker.node();
    let mut covering_error_range = None::<tree_sitter::Range>;

    loop {
        if walker.done {
            break;
        }

        let current = walker.node();

        if current.is_error() {
            let range = current.range();
            match covering_error_range {
                Some(ref error_range) if error_range.contains(&range) => {
                    previous = current;
                    walker.goto_next();
                    continue;
                },
                _ => {
                    covering_error_range = Some(range.clone());
                },
            }
            let message = String::from("ERROR node");
            let range = content.tree_sitter_range_to_lsp_range(range);
            let severity = Some(lsp::DiagnosticSeverity::ERROR);
            diagnostics.push(lsp::Diagnostic {
                range,
                severity,
                message,
                ..Default::default()
            });
            previous = current;
            walker.goto_next();
            continue;
        }

        if current.is_missing() {
            let range = current.range();
            match covering_error_range {
                Some(ref error_range) if error_range.contains(&range) => {
                    previous = current;
                    walker.goto_next();
                    continue;
                },
                _ => {
                    covering_error_range = Some(range.clone());
                },
            }
            let message = format!(r#"expected "{}" after "{}""#, current.kind(), previous.kind());
            let range = content.tree_sitter_range_to_lsp_range(range);
            let severity = Some(lsp::DiagnosticSeverity::ERROR);
            diagnostics.push(lsp::Diagnostic {
                range,
                severity,
                message,
                ..Default::default()
            });
            previous = current;
            walker.goto_next();
            continue;
        }

        // catch all case
        previous = current;
        walker.goto_next();
    }

    diagnostics.reverse();
    diagnostics
}
