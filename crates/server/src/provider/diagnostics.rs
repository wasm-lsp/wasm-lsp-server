use crate::core;

/// Provider definitions for LSP `textDocument/publishDiagnostics` for `.wast` documents.
pub mod wast;

/// Provider definitions for LSP `textDocument/publishDiagnostics` for `.wat` documents.
pub mod wat;

/// Provider function for LSP `textDocument/publishDiagnostics`.
pub fn diagnostics(tree: &tree_sitter::Tree, text: &core::Text) -> Vec<lsp::Diagnostic> {
    match text.language {
        core::Language::Wast => wast::diagnostics(tree, &text.content),
        core::Language::Wat => wat::diagnostics(tree, &text.content),
    }
}
