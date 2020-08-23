//! Core functionality related to documents.

use tokio::sync::Mutex;
use tree_sitter::{Parser, Tree};
use wasm_language_server_parsers::core::language::Language;

/// The current state of a document.
pub(crate) struct Document {
    /// The language type of the document, e.g., "wasm.wast"
    pub(crate) language: Language,
    /// The tree-sitter parser state for the document.
    pub(crate) parser: Mutex<Parser>,
    /// The current text of the document.
    pub(crate) text: String,
    /// The current tree-sitter parse tree of the document.
    pub(crate) tree: Mutex<Tree>,
}
