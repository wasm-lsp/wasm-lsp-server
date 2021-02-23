use crate::core::{self, node::NodeWalker};
use lsp_text::RopeExt;

pub fn diagnostics(tree: &tree_sitter::Tree, content: &ropey::Rope) -> Vec<lsp::Diagnostic> {
    let mut diagnostics = vec![];
    let mut walker = {
        let language = core::Language::Wast;
        let node = tree.root_node();
        NodeWalker::new(language, node)
    };

    loop {
        if walker.done {
            break;
        }

        let node = walker.node();

        if node.is_error() {
            let message = String::from("ERROR node");
            let range = content.tree_sitter_range_to_lsp_range(node.range());
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
            let message = String::from("MISSING node");
            let range = content.tree_sitter_range_to_lsp_range(node.range());
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
