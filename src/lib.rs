#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

//! The WASM language server.

// Command-line interface for the WASM language server.
pub mod cli;
// Core functionality for the WASM language server.
mod core;
// Functionality related to implementation of the Language Server Protocol (LSP).
pub mod lsp;
// Definitions related to the wasm-language-server crate.
mod package;
// Services (components) of the WASM language server.
mod service;
// Various utility functionality, e.g., for handling lsp or tree-sitter data.
mod util;

use tree_sitter::Language;

extern {
    #[allow(dead_code)]
    #[doc(hidden)]
    fn tree_sitter_wast() -> Language;

    #[allow(dead_code)]
    #[doc(hidden)]
    fn tree_sitter_wat() -> Language;

    #[allow(dead_code)]
    #[doc(hidden)]
    fn tree_sitter_wit() -> Language;

    #[allow(dead_code)]
    #[doc(hidden)]
    fn tree_sitter_witx() -> Language;
}
