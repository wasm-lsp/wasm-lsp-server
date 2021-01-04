//! Core functionality related to documents.

use crate::core::language::Language;
use ropey::Rope;
use std::convert::TryFrom;
use tokio::sync::Mutex;
use tree_sitter::{Parser, Tree};

/// The current state of a document.
pub struct Document {
    /// The language type of the document, e.g., "wasm.wast"
    pub language: Language,
    /// The current text of the document.
    pub rope: Rope,
    /// The tree-sitter parser state for the document.
    pub parser: Mutex<Parser>,
    /// The current tree-sitter parse tree of the document.
    pub tree: Mutex<Tree>,
}

impl Document {
    /// Create a new Document for the given `language_id` and `text`.
    pub fn new(language_id: &str, text: String) -> anyhow::Result<Option<Self>> {
        let language = Language::try_from(language_id)?;
        let mut parser = tree_sitter::Parser::try_from(language)?;
        let old_tree = None;
        let tree = parser.parse(&text[..], old_tree);
        let rope = Rope::from(text);
        let document = tree.map(|tree| Document {
            language,
            rope,
            parser: Mutex::new(parser),
            tree: Mutex::new(tree),
        });
        Ok(document)
    }
}
