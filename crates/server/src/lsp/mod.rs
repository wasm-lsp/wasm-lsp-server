//! Functionality related to implementation of the Language Server Protocol (LSP).

// Definitions for the request handlers.
mod api;

// LSP related definitions shared between server crates.
pub(crate) use wasm_language_server_shared::lsp::*;

// Definitions for the server instance.
pub mod server;
