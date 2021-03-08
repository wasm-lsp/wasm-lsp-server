//! Functionality related to [`tree_sitter::Parser`].

use crate::language::Language;
use std::convert::TryFrom;

/// Create a `.wast` parser from the tree-sitter grammar.
pub fn wast() -> anyhow::Result<tree_sitter::Parser> {
    let language = wasm_lsp_languages::wast();
    let mut parser = tree_sitter::Parser::new()?;
    parser.set_language(&language)?;
    Ok(parser)
}

/// Create a `.wat` parser from the tree-sitter grammar.
pub fn wat() -> anyhow::Result<tree_sitter::Parser> {
    let language = wasm_lsp_languages::wat();
    let mut parser = tree_sitter::Parser::new()?;
    parser.set_language(&language)?;
    Ok(parser)
}

impl TryFrom<Language> for tree_sitter::Parser {
    type Error = anyhow::Error;

    fn try_from(language: Language) -> anyhow::Result<tree_sitter::Parser> {
        match language {
            Language::Wast => wast(),
            Language::Wat => wat(),
        }
    }
}
