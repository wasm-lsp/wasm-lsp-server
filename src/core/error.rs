//! Core functionality related to runtime errors.

use failure::Fail;
use tree_sitter;

/// Runtime errors for the WASM language server.
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Fail)]
pub(crate) enum Error {
    /// Error that occurs when parsing an invalid language-id string.
    #[fail(display = "core::InvalidLanguageId: {}", 0)]
    CoreInvalidLanguageId(String),
    /// Error that occurs when attempting to set an invalid language for a tree-sitter parser.
    #[fail(display = "tree_sitter::LanguageError: {}", 0)]
    TreeSitterLanguageError(tree_sitter::LanguageError),
    /// Error that occurs when attempting to create a tree-sitter query from invalid source.
    #[fail(display = "tree_sitter::QueryError")]
    TreeSitterQueryError(tree_sitter::QueryError),
}
