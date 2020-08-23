//! Core functionality related to document parsers.

// FIXME: move these to a subcrate

use crate::core::language::{self, Language};
use wasm_language_server_shared::core::error::{Error, Fallible};

/// Create a wast parser from the tree-sitter grammar.
pub fn wast() -> Fallible<tree_sitter::Parser> {
    let language = language::wast::language();
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(language).map_err(Error::TreeSitterLanguageError)?;
    Ok(parser)
}

/// Create a wat parser from the tree-sitter grammar.
pub fn wat() -> Fallible<tree_sitter::Parser> {
    let language = language::wat::language();
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(language).map_err(Error::TreeSitterLanguageError)?;
    Ok(parser)
}

/// Create a wit parser from the tree-sitter grammar.
pub fn wit() -> Fallible<tree_sitter::Parser> {
    let language = language::wit::language();
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(language).map_err(Error::TreeSitterLanguageError)?;
    Ok(parser)
}

/// Create a witx parser from the tree-sitter grammar.
pub fn witx() -> Fallible<tree_sitter::Parser> {
    let language = language::witx::language();
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(language).map_err(Error::TreeSitterLanguageError)?;
    Ok(parser)
}

/// Create a parser for the given `language`.
pub fn try_from(language: Language) -> Fallible<tree_sitter::Parser> {
    match language {
        Language::Wast => wast(),
        Language::Wat => wat(),
        Language::Wit => wit(),
        Language::Witx => witx(),
    }
}
