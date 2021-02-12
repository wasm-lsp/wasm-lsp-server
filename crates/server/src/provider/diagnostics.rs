use crate::core;

mod wast;
mod wat;

pub fn diagnostics(tree: &tree_sitter::Tree, text: &core::Text) -> Vec<lsp::Diagnostic> {
    match text.language {
        core::Language::Wast => wast::diagnostics(tree, &text.content),
        core::Language::Wat => wat::diagnostics(tree, &text.content),
    }
}
