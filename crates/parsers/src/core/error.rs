//! Core definitions related to runtime errors.

use thiserror::Error;

/// Runtime errors for the WebAssembly parsers.
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Error)]
pub enum Error {
    /// Error that occurs when parsing an invalid language-id string.
    #[error("InvalidLanguageId: {0}")]
    InvalidLanguageId(String),
    /// Error that occurs when `tree_sitter::Parser::parse` returns `None`.
    #[error("tree_sitter::ParseNone")]
    TreeSitterParseNone,
    /// Error that occurs when attempting to set an invalid language for a tree-sitter parser.
    #[error("tree_sitter::LanguageError: {0}")]
    TreeSitterLanguageError(tree_sitter::LanguageError),
}
