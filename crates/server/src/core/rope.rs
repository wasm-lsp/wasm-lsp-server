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

    /// Translate an [`lsp::Position`] to a utf16 offset.
    fn lsp_position_to_utf16_offset(&self, position: lsp::Position) -> anyhow::Result<usize>;
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

    fn lsp_position_to_utf16_offset(&self, position: lsp::Position) -> anyhow::Result<usize> {
        let line = self.line(position.line as usize);
        let character = position.character as usize;

        let mut utf16_offset = 0usize;
        let mut char_offset = 0usize;

        for c in line.chars() {
            if utf16_offset == character {
                break;
            }

            if utf16_offset > character {
                return Err(anyhow!("character is not on an offset boundary"));
            }

            utf16_offset += c.len_utf16();
            char_offset += 1;
        }

        Ok(self.line_to_char(position.line as usize) + char_offset)
    }
}
