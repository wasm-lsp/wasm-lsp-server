//! Core functionality for the WASM language server.

// Core functionality related to the document metadata database.
pub(crate) mod database;

// Core functionality related to documents.
pub(crate) mod document;

// Core functionality related to runtime errors.
pub(crate) mod error;

// Core functionality related to document languages.
pub(crate) mod language;

// Core functionality related to document parsers.
pub(crate) mod parser;

// Core functionality related to the LSP server session.
pub(crate) mod session;
