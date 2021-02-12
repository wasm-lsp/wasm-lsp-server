use lsp_text::RopeExt;

pub fn diagnostics(tree: &tree_sitter::Tree, content: &ropey::Rope) -> Vec<lsp::Diagnostic> {
    let mut diagnostics = vec![];
    let mut work = vec![tree.root_node()];
    let mut cursor = tree.root_node().walk();

    while let Some(node) = work.pop() {
        let range = {
            let start = content.byte_to_lsp_position(node.start_byte() as usize);
            let end = content.byte_to_lsp_position(node.end_byte() as usize);
            lsp::Range { start, end }
        };

        if node.is_error() {
            let message = String::from("ERROR node");
            let severity = Some(lsp::DiagnosticSeverity::Error);
            diagnostics.push(lsp::Diagnostic {
                message,
                range,
                severity,
                ..Default::default()
            });
            continue;
        }

        if node.is_missing() {
            let message = String::from("MISSING node");
            let severity = Some(lsp::DiagnosticSeverity::Error);
            diagnostics.push(lsp::Diagnostic {
                message,
                range,
                severity,
                ..Default::default()
            });
            continue;
        }

        if node.has_error() {
            cursor.reset(node.clone());
            work.extend(node.named_children(&mut cursor));
        }
    }

    diagnostics
}
