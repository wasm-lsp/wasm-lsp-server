/// Provider definitions for LSP `textDocument/publishDiagnostics` for `.wast` documents.
pub mod wast;

/// Provider definitions for LSP `textDocument/publishDiagnostics` for `.wat` documents.
pub mod wat;

/// Provider function for LSP `textDocument/publishDiagnostics`.
pub fn diagnostics(tree: &tree_sitter::Tree, text: &crate::core::Text) -> Vec<lsp::Diagnostic> {
    match text.language {
        crate::core::Language::Wast => wast::diagnostics(tree, &text.content),
        crate::core::Language::Wat => wat::diagnostics(tree, &text.content),
    }
}
