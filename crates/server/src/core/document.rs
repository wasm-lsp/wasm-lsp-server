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

    /// Build a [`DocumentEdit`] from an [`lsp::TextDocumentContentChangeEvent`].
    pub fn build_edit<'a>(&self, change: &'a lsp::TextDocumentContentChangeEvent) -> anyhow::Result<DocumentEdit<'a>> {
        let text = change.text.as_str();
        let text_bytes = text.as_bytes();
        let text_end_byte_idx = text_bytes.len() - 1;

        let range = if let Some(range) = change.range {
            range
        } else {
            let start = self.content.byte_to_lsp_position(0);
            let end = self.content.byte_to_lsp_position(text_end_byte_idx);
            lsp::Range {
                start,
                end,
            }
        };

        let start_char_idx = {
            let start_offset = self.content.lsp_position_to_utf16_cu(range.start)?;
            self.content.utf16_cu_to_char(start_offset)
        };

        let end_char_idx = {
            let end_offset = self.content.lsp_position_to_utf16_cu(range.end)?;
            self.content.utf16_cu_to_char(end_offset)
        };

        let start_byte = self.content.char_to_byte(start_char_idx);
        let old_end_byte = {
            let end_char = self.content.char(end_char_idx);
            self.content.char_to_byte(end_char_idx) + end_char.len_utf8()
        };
        let new_end_byte = start_byte + text_end_byte_idx;

        let start_position = self.content.byte_to_tree_sitter_point(start_byte);
        let old_end_position = self.content.byte_to_tree_sitter_point(old_end_byte);
        let new_end_position = {
            let mut last_line = change.text.as_str();
            let mut line_count = 0;

            for line in change.text.lines() {
                last_line = line;
                line_count += 1;
            }

            let row = start_position.row + line_count;
            let column = {
                let padding = if line_count > 0 {
                    0
                } else {
                    old_end_byte
                };
                padding + last_line.as_bytes().len()
            };
            tree_sitter::Point {
                row,
                column,
            }
        };

        let input_edit = tree_sitter::InputEdit {
            start_byte,
            old_end_byte,
            new_end_byte,
            start_position,
            old_end_position,
            new_end_position,
        };

        Ok(DocumentEdit {
            input_edit,
            start_char_idx,
            end_char_idx,
            text,
        })
    }

    /// Modify the given [`lsp::Range`] in the document.
    pub fn apply_edit(&mut self, edit: &DocumentEdit) {
        self.content.remove(edit.start_char_idx .. edit.end_char_idx);
        if !edit.text.is_empty() {
            self.content.insert(edit.start_char_idx, &edit.text);
        }
    }
}

/// A description of an edit to a [`Document`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DocumentEdit<'a> {
    /// The input edit structure used for modifying the syntax tree.
    pub input_edit: tree_sitter::InputEdit,
    /// The starting index (as character offset) of the edit's modification range.
    pub start_char_idx: usize,
    /// The ending index (as character offset) of the edit's modification range.
    pub end_char_idx: usize,
    /// The text of the edit.
    pub text: &'a str,
}
