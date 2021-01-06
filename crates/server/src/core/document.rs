//! Core functionality related to documents.

use crate::core::{language::Language, rope::RopeExt};
use ropey::Rope;
use std::convert::{TryFrom, TryInto};
use tokio::sync::Mutex;
use tree_sitter::{Parser, Tree};

/// The current state of a document.
pub struct Document {
    /// The language type of the document, e.g., "wasm.wast"
    pub language: Language,
    /// The current text content of the document.
    pub content: Rope,
    /// The tree-sitter parser state for the document.
    pub parser: Mutex<Parser>,
    /// The current tree-sitter parse tree of the document.
    pub tree: Mutex<Tree>,
}

impl Document {
    /// Create a new [`Document`] for the given language id and text content.
    pub fn new(
        language_id: impl TryInto<Language, Error = anyhow::Error>,
        text: impl AsRef<str>,
    ) -> anyhow::Result<Option<Self>> {
        let text = text.as_ref();

        let language = language_id.try_into()?;
        let mut parser = tree_sitter::Parser::try_from(language)?;
        let old_tree = None;
        let new_tree = parser.parse(&text[..], old_tree);
        let content = Rope::from_str(text);

        let document = new_tree.map(|tree| Document {
            language,
            content,
            parser: Mutex::new(parser),
            tree: Mutex::new(tree),
        });

        Ok(document)
    }

    /// Modify the given [`lsp::Range`] in the document.
    pub fn modify_lsp_range(&mut self, range: lsp::Range, text: impl AsRef<str>) -> anyhow::Result<()> {
        let text = text.as_ref();

        let start = self.content.lsp_position_to_utf16_offset(range.start)?;
        let end = self.content.lsp_position_to_utf16_offset(range.start)?;

        self.content.remove(start .. end);
        if !text.is_empty() {
            self.content.insert(start, text);
        }

        Ok(())
    }
}
