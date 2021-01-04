//! Core functionality for the WASM language server.

// Core functionality related to the document metadata database.
pub(crate) mod database;

// Core functionality related to documents.
pub mod document;

// Core functionality related to runtime errors.
pub(crate) mod error;

// Core functionality related to working with ropes.
pub(crate) mod rope;

// Core functionality related to parsers.
pub(crate) use wasm_language_server_parsers::core::*;

// Core functionality related to the LSP server session.
pub mod session;
