//! Parsers for the WebAssembly language server.

#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

/// Functionality related to [`tree_sitter::Language`].
pub mod language;

/// Functionality related to [`tree_sitter::Node`].
pub mod node;

/// Functionality related to [`tree_sitter::Range`].
pub mod range;
