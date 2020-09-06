//! Core functionality related to document parsers.

// FIXME: move these to a subcrate

use crate::core::{
    error::Error,
    language::{self, Language},
};
use std::convert::TryFrom;

/// Create a wast parser from the tree-sitter grammar.
pub fn wast() -> anyhow::Result<tree_sitter::Parser> {
    let language = language::wast::language();
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(language).map_err(Error::TreeSitterLanguageError)?;
    Ok(parser)
}

/// Create a wat parser from the tree-sitter grammar.
pub fn wat() -> anyhow::Result<tree_sitter::Parser> {
    let language = language::wat::language();
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(language).map_err(Error::TreeSitterLanguageError)?;
    Ok(parser)
}

/// Create a wit parser from the tree-sitter grammar.
pub fn wit() -> anyhow::Result<tree_sitter::Parser> {
    let language = language::wit::language();
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(language).map_err(Error::TreeSitterLanguageError)?;
    Ok(parser)
}

/// Create a witx parser from the tree-sitter grammar.
pub fn witx() -> anyhow::Result<tree_sitter::Parser> {
    let language = language::witx::language();
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(language).map_err(Error::TreeSitterLanguageError)?;
    Ok(parser)
}

impl TryFrom<Language> for tree_sitter::Parser {
    type Error = anyhow::Error;

    fn try_from(language: Language) -> anyhow::Result<tree_sitter::Parser> {
        match language {
            Language::Wast => wast(),
            Language::Wat => wat(),
            Language::Wit => wit(),
            Language::Witx => witx(),
        }
    }
}
