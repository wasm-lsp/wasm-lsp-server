//! Services (components) of the WASM language server.

// Computes queries about documents.
pub(crate) mod analyzer;

// Collects diagnostics for documents with syntax and validation errors.
mod auditor;

// Elaborates parse trees into structured data to be cached in the database.
pub mod elaborator;

// Computes highlights for documents.
mod highlighter;

// Synchronizes document changes between editor and server.
pub(crate) mod synchronizer;

// Synthesizes typing information for documents.
mod synthesizer;
