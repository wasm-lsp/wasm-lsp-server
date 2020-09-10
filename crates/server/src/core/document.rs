//! Core functionality related to documents.

use crate::core::language::Language;
use std::convert::TryFrom;
use tokio::sync::Mutex;
use tree_sitter::{Parser, Tree};

/// The current state of a document.
pub struct Document {
    /// The language type of the document, e.g., "wasm.wast"
    pub language: Language,
    /// The tree-sitter parser state for the document.
    pub parser: Mutex<Parser>,
    /// The current text of the document.
    pub text: String,
    /// The current tree-sitter parse tree of the document.
    pub tree: Mutex<Tree>,
}

impl Document {
    /// Create a new Document for the given `language_id` and `text`.
    pub fn new(language_id: &str, text: String) -> anyhow::Result<Self> {
        use wasm_language_server_parsers::core::error::Error::TreeSitterParseNone;
        let language = Language::try_from(language_id)?;
        let mut parser = tree_sitter::Parser::try_from(language)?;
        let old_tree = None;
        let tree = parser.parse(&text[..], old_tree).ok_or(TreeSitterParseNone)?;
        Ok(Document {
            language,
            parser: Mutex::new(parser),
            text,
            tree: Mutex::new(tree),
        })
    }
}
