//! Testing functionality for the WebAssembly language server.

#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

//! JSON-RPC-related definitions.
pub mod jsonrpc;

/// LSP-related definitions.
pub mod lsp;

/// Service-related definitions.
pub mod service;
