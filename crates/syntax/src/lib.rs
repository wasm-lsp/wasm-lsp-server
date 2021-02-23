//! Parsers for the WebAssembly language server.

#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]
#![recursion_limit = "512"]

/// Functionality related to runtime errors.
pub mod error;

/// Functionality related to [`tree-sitter::Language`].
pub mod language;

/// Functionality related to [`tree-sitter::Node`].
pub mod node;

/// Functionality related to [`tree-sitter::Parser`].
pub mod parser;

#[cfg(not(target_arch = "wasm32"))]
extern {
    #[allow(dead_code)]
    #[doc(hidden)]
    fn tree_sitter_wast() -> tree_sitter_sys::Language;

    #[allow(dead_code)]
    #[doc(hidden)]
    fn tree_sitter_wat() -> tree_sitter_sys::Language;

    #[allow(dead_code)]
    #[doc(hidden)]
    fn tree_sitter_wit() -> tree_sitter_sys::Language;

    #[allow(dead_code)]
    #[doc(hidden)]
    fn tree_sitter_witx() -> tree_sitter_sys::Language;
}