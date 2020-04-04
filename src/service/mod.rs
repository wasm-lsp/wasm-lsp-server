/// Computes queries about documents.
mod analyzer;

/// Collects diagnostics for documents with syntax errors, etc.
mod auditor;

/// Elaborates parse trees into structured data to be cached in the database.
pub(crate) mod elaborator;

/// Computes highlights for documents.
mod highlighter;

/// Synchronizes document changes between editor and server.
pub(crate) mod synchronizer;

/// Synthesizes typing information for documents.
mod synthesizer;
