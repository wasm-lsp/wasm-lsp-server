//! The implementation of the WebAssembly language server.

#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]
#![allow(clippy::if_same_then_else)]
#![allow(clippy::needless_lifetimes)]

/// Core definitions for server functionality.
pub mod core;

/// LSP message handler functions.
pub mod handler;

/// Build-time metadata for the server.
pub mod metadata;

/// LSP feature provider functions.
pub mod provider;

/// LSP server capabilities and server instance.
pub mod server;

pub use server::*;
