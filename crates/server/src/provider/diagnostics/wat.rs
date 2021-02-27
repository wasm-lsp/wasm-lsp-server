//! Provider definitions for LSP `textDocument/publishDiagnostics` for `.wat` documents.

use crate::core::{self, node::NodeWalker, range::RangeExt};
use lsp_text::RopeExt;

/// Provider function for LSP `textDocument/publishDiagnostics` for `.wat` documents.
pub fn diagnostics(tree: &tree_sitter::Tree, content: &ropey::Rope) -> Vec<lsp::Diagnostic> {
    let mut diagnostics = vec![];
    let mut walker = {
        let language = core::Language::Wat;
        let node = tree.root_node();
        NodeWalker::new(language, node)
    };

    let mut covering_error_range = None::<tree_sitter::Range>;

    loop {
        if walker.done {
            break;
        }

        let node = walker.node();

        if node.is_error() {
            let range = node.range();
            match covering_error_range {
                Some(ref error_range) if error_range.contains(&range) => {
                    walker.goto_next_has_error();
                    continue;
                },
                _ => {
                    covering_error_range = Some(range.clone());
                }
            }
            let message = String::from("ERROR node");
            let range = content.tree_sitter_range_to_lsp_range(range);
            let severity = Some(lsp::DiagnosticSeverity::Error);
            diagnostics.push(lsp::Diagnostic {
                message,
                range,
                severity,
                ..Default::default()
            });
            walker.goto_next_has_error();
            continue;
        }

        if node.is_missing() {
            let range = node.range();
            match covering_error_range {
                Some(ref error_range) if error_range.contains(&range) => {
                    walker.goto_next_has_error();
                    continue;
                },
                _ => {
                    covering_error_range = Some(range.clone());
                }
            }
            let message = String::from("MISSING node");
            let range = content.tree_sitter_range_to_lsp_range(range);
            let severity = Some(lsp::DiagnosticSeverity::Error);
            diagnostics.push(lsp::Diagnostic {
                message,
                range,
                severity,
                ..Default::default()
            });
            walker.goto_next_has_error();
            continue;
        }

        // catch all case
        walker.goto_next_has_error();
    }

    diagnostics.reverse();
    diagnostics
}
