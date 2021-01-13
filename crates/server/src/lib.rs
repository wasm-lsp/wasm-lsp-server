//! The WebAssembly language server.

#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

// Core functionality for the WASM language server.
pub mod core;

// Definitions related to the wasm-language-server crate.
pub mod package;

// Providers of the WebAssembly language server for LSP features.
pub mod provider;

// The implementation of the Language Server Protocol (LSP).
pub mod server;

// Services (components) of the WebAssembly language server.
pub mod service;

// Various utility functionality, e.g., for handling lsp or tree-sitter data.
mod util;
