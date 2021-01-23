#[cfg(not(target_arch = "wasm32"))]
mod native {
    use anyhow::anyhow;
    use ropey::{iter::Chunks, Rope};
    use std::convert::TryFrom;

    trait ChunkExt<'a> {
        fn next_str(&mut self) -> &'a str;
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

    pub(crate) struct ChunkWalker<'a> {
        rope: &'a Rope,
        cursor: usize,
        cursor_chunk: &'a str,
        chunks: Chunks<'a>,
    }

    impl<'a> ChunkWalker<'a> {
        fn prev_chunk(&mut self) {
            self.cursor -= self.cursor_chunk.len();
            self.cursor_chunk = self.chunks.prev_str();
            while 0 < self.cursor && self.cursor_chunk.is_empty() {
                self.cursor_chunk = self.chunks.prev_str();
            }
        }

        fn next_chunk(&mut self) {
            self.cursor += self.cursor_chunk.len();
            self.cursor_chunk = self.chunks.next_str();
            while self.cursor < self.rope.len_bytes() && self.cursor_chunk.is_empty() {
                self.cursor_chunk = self.chunks.next_str();
            }
        }

        pub(crate) fn callback_adapter(mut self) -> impl FnMut(u32, tree_sitter::Point) -> &'a [u8] {
            move |byte_idx, _position| {
                let byte_idx = byte_idx as usize;

                while byte_idx < self.cursor && 0 < self.cursor {
                    self.prev_chunk();
                }

                while byte_idx >= self.cursor + self.cursor_chunk.len() && byte_idx < self.rope.len_bytes() {
                    self.next_chunk();
                }

                &self.cursor_chunk.as_bytes()[byte_idx - self.cursor ..]
            }
        }
    }

    pub(crate) trait RopeExt<'a> {
        fn chunk_walker(&'a self, byte_idx: usize) -> ChunkWalker<'a>;
        fn lsp_position_to_byte(&self, position: lsp::Position) -> anyhow::Result<u32>;
        fn lsp_position_to_utf16_cu(&self, position: lsp::Position) -> anyhow::Result<u32>;
        fn lsp_range_to_tree_sitter_range(&self, range: lsp::Range) -> anyhow::Result<tree_sitter::Range>;
        fn byte_to_lsp_position(&self, offset: usize) -> lsp::Position;
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

        fn lsp_position_to_byte(&self, position: lsp::Position) -> anyhow::Result<u32> {
            let line_idx = position.line as usize;
            let character_byte_idx = {
                let utf16_cu_idx = position.character as usize;
                let char_idx = self.utf16_cu_to_char(utf16_cu_idx);
                self.char_to_byte(char_idx)
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

            let result = self.line_to_char(line_idx) + char_offset;
            let result = u32::try_from(result).unwrap();
            Ok(result)
        }

        fn lsp_position_to_utf16_cu(&self, position: lsp::Position) -> anyhow::Result<u32> {
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

            let result = self.line_to_char(line_idx) + char_offset;
            let result = u32::try_from(result).unwrap();
            Ok(result)
        }

        fn lsp_range_to_tree_sitter_range(&self, range: lsp::Range) -> anyhow::Result<tree_sitter::Range> {
            let start_byte = self.lsp_position_to_byte(range.start)?;
            let end_byte = self.lsp_position_to_byte(range.start)?;
            let start_point = &Default::default();
            let end_point = &Default::default();
            let range = tree_sitter::Range::new(start_byte, end_byte, start_point, end_point);
            Ok(range)
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
            let row = u32::try_from(line_idx).unwrap();
            let column = u32::try_from(byte_idx - line_byte_idx).unwrap();
            tree_sitter::Point::new(row, column)
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub use native::*;

#[cfg(target_arch = "wasm32")]
mod wasm {
}

#[cfg(target_arch = "wasm32")]
pub use wasm::*;
