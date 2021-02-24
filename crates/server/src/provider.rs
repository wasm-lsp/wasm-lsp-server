//! LSP feature provider functions.

/// Provider definitions for LSP `textDocument/publishDiagnostics`.
pub mod diagnostics;

/// Provider definitions for LSP `textDocument/documentSymbol`
pub mod document_symbol;

/// Provider definitions for LSP `textDocument/semanticTokens/*`
pub mod semantic_tokens;

pub use diagnostics::*;
pub use document_symbol::document_symbol;
