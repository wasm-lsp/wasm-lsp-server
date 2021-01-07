//! Core functionality related to working with ropes.

use anyhow::anyhow;
use ropey::{iter::Chunks, Rope};

/// Convenience trait for working with [`Chunks`].
trait ChunkExt<'a> {
    /// Walk to the next string slice (or return the empty slice).
    fn next_str(&mut self) -> &'a str;
    /// Walk to the previous string slice (or return the empty slice).
    fn prev_str(&mut self) -> &'a str;
}

impl<'a> ChunkExt<'a> for Chunks<'a> {
    fn next_str(&mut self) -> &'a str {
        self.next().unwrap_or("")
    }

    fn prev_str(&mut self) -> &'a str {
        self.prev().unwrap_or("")
    }
}

/// State structure used for building a tree-sitter parser callback adapter.
pub(crate) struct ChunkWalker<'a> {
    /// The underlying Rope to walk across.
    rope: &'a Rope,
    /// The byte index of the current rope offset. Always resides on the 0th index of some slice.
    cursor: usize,
    /// The current slice we are traversing (may be an empty slice).
    cursor_chunk: &'a str,
    /// The underlying chunk iterator from the rope.
    chunks: Chunks<'a>,
}

impl<'a> ChunkWalker<'a> {
    /// Walk to the previous chunk, adjusting state, and skipping empty chunks.
    fn prev_chunk(&mut self) {
        self.cursor -= self.cursor_chunk.len();
        self.cursor_chunk = self.chunks.prev_str();
        // Skip over empty chunks.
        while 0 < self.cursor && self.cursor_chunk.is_empty() {
            self.cursor_chunk = self.chunks.prev_str();
        }
    }

    /// Walk to the next chunk, adjusting state, and skipping empty chunks.
    fn next_chunk(&mut self) {
        self.cursor += self.cursor_chunk.len();
        self.cursor_chunk = self.chunks.next_str();
        // Skip over empty chunks.
        while self.cursor < self.rope.len_bytes() && self.cursor_chunk.is_empty() {
            self.cursor_chunk = self.chunks.next_str();
        }
    }

    /// Consume the [`ChunkWalker`] and return a callback for use with
    /// [`tree_sitter::Parser::parse_with`].
    pub(crate) fn callback_adapter(mut self) -> impl FnMut(usize, tree_sitter::Point) -> &'a [u8] {
        move |byte_idx, _position| {
            // Scan backward first if necessary.
            while byte_idx < self.cursor && 0 < self.cursor {
                self.prev_chunk();
            }

            // Otherwise scan forward if necessary.
            while byte_idx >= self.cursor + self.cursor_chunk.len() && byte_idx < self.rope.len_bytes() {
                self.next_chunk();
            }

            // Otherwise slice into the current chunk with the given index.
            &self.cursor_chunk.as_bytes()[byte_idx - self.cursor ..]
        }
    }
}

/// Extension trait for [`Rope`].
pub(crate) trait RopeExt<'a> {
    /// Build a [`ChunkWalker`] given an appropriate structure and a starting byte offset.
    fn chunk_walker(&'a self, byte_idx: usize) -> ChunkWalker<'a>;

    /// Translate an [`lsp::Position`] to a utf8 offset.
    fn lsp_position_to_byte(&self, position: lsp::Position) -> anyhow::Result<usize>;

    /// Translate an [`lsp::Position`] to a utf16 offset.
    fn lsp_position_to_utf16_cu(&self, position: lsp::Position) -> anyhow::Result<usize>;

    /// Translate an [`lsp::Range`] to a [`tree_sitter::Range`].
    fn lsp_range_to_tree_sitter_range(&self, range: lsp::Range) -> anyhow::Result<tree_sitter::Range>;

    /// Translate a utf8 offset to an [`lsp::Position`].
    fn byte_to_lsp_position(&self, offset: usize) -> lsp::Position;

    /// Translate a utf8 offset to a [`tree_sitter::Point`].
    fn byte_to_tree_sitter_point(&self, offset: usize) -> tree_sitter::Point;
}

impl<'a> RopeExt<'a> for Rope {
    fn chunk_walker(&'a self, byte_idx: usize) -> ChunkWalker<'a> {
        let (mut chunks, chunk_byte_idx, ..) = self.chunks_at_byte(byte_idx);
        let cursor = chunk_byte_idx;
        let cursor_chunk = chunks.next_str();
        ChunkWalker {
            rope: self,
            cursor,
            cursor_chunk,
            chunks,
        }
    }

    fn lsp_position_to_byte(&self, position: lsp::Position) -> anyhow::Result<usize> {
        let line_idx = position.line as usize;
        let character_byte_idx = {
            let utf16_cu_idx = position.character as usize;
            let char_idx = self.utf16_cu_to_char(utf16_cu_idx);
            let byte_idx = self.char_to_byte(char_idx);
            byte_idx
        };

        let mut utf8_offset = 0usize;
        let mut char_offset = 0usize;

        for c in self.line(line_idx).chars() {
            if utf8_offset == character_byte_idx {
                break;
            }

            if utf8_offset > character_byte_idx {
                return Err(anyhow!("character is not on an offset boundary"));
            }

            utf8_offset += c.len_utf8();
            char_offset += 1;
        }

        Ok(self.line_to_char(line_idx) + char_offset)
    }

    fn lsp_position_to_utf16_cu(&self, position: lsp::Position) -> anyhow::Result<usize> {
        let line_idx = position.line as usize;
        let character_utf16_cu_idx = position.character as usize;

        let mut utf16_offset = 0usize;
        let mut char_offset = 0usize;

        for c in self.line(line_idx).chars() {
            if utf16_offset == character_utf16_cu_idx {
                break;
            }

            if utf16_offset > character_utf16_cu_idx {
                return Err(anyhow!("character is not on an offset boundary"));
            }

            utf16_offset += c.len_utf16();
            char_offset += 1;
        }

        Ok(self.line_to_char(line_idx) + char_offset)
    }

    fn lsp_range_to_tree_sitter_range(&self, range: lsp::Range) -> anyhow::Result<tree_sitter::Range> {
        let start_byte = self.lsp_position_to_byte(range.start)?;
        let end_byte = self.lsp_position_to_byte(range.start)?;

        let start_point = Default::default();
        let end_point = Default::default();

        Ok(tree_sitter::Range {
            start_byte,
            end_byte,
            start_point,
            end_point,
        })
    }

    fn byte_to_lsp_position(&self, byte_idx: usize) -> lsp::Position {
        let line_idx = self.byte_to_line(byte_idx);

        let line_utf16_cu_idx = {
            let char_idx = self.line_to_char(line_idx);
            self.char_to_utf16_cu(char_idx)
        };

        let character_utf16_cu_idx = {
            let char_idx = self.byte_to_char(byte_idx);
            self.char_to_utf16_cu(char_idx)
        };

        let line = line_idx;
        let character = character_utf16_cu_idx - line_utf16_cu_idx;

        lsp::Position::new(line as u32, character as u32)
    }

    fn byte_to_tree_sitter_point(&self, byte_idx: usize) -> tree_sitter::Point {
        let line_idx = self.byte_to_line(byte_idx);
        let line_byte_idx = self.line_to_byte(line_idx);
        tree_sitter::Point::new(line_idx, byte_idx - line_byte_idx)
    }
}
