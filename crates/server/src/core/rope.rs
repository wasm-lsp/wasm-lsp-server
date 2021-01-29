use anyhow::anyhow;
use bytes::Bytes;
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

pub(crate) struct ChunkWalker {
    rope: Rope,
    cursor: usize,
    cursor_chunk: &'static str,
    chunks: Chunks<'static>,
}

impl ChunkWalker {
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

    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn callback_adapter(mut self) -> impl FnMut(u32, tree_sitter::Point) -> Bytes {
        move |start_index, _position| {
            let start_index = start_index as usize;

            while start_index < self.cursor && 0 < self.cursor {
                self.prev_chunk();
            }

            while start_index >= self.cursor + self.cursor_chunk.len() && start_index < self.rope.len_bytes() {
                self.next_chunk();
            }

            let bytes = self.cursor_chunk.as_bytes();
            let bytes = &bytes[start_index - self.cursor ..];
            Bytes::from_static(bytes)
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub(crate) fn callback_adapter(mut self) -> impl FnMut(u32, Option<tree_sitter::Point>, Option<u32>) -> Bytes {
        move |start_index, _position, end_index| {
            let start_index = start_index as usize;

            while start_index < self.cursor && 0 < self.cursor {
                self.prev_chunk();
            }

            while start_index >= self.cursor + self.cursor_chunk.len() && start_index < self.rope.len_bytes() {
                self.next_chunk();
            }

            let bytes = self.cursor_chunk.as_bytes();
            let end_index = end_index.map(|i| i as usize).unwrap_or_else(|| bytes.len());
            let bytes = &bytes[start_index - self.cursor .. end_index];
            Bytes::from_static(bytes)
        }
    }
}

pub(crate) trait RopeExt {
    fn chunk_walker(self, byte_idx: usize) -> ChunkWalker;
    fn lsp_position_to_byte(&self, position: lsp::Position) -> anyhow::Result<u32>;
    fn lsp_position_to_utf16_cu(&self, position: lsp::Position) -> anyhow::Result<u32>;
    fn lsp_range_to_tree_sitter_range(&self, range: lsp::Range) -> anyhow::Result<tree_sitter::Range>;
    fn byte_to_lsp_position(&self, offset: usize) -> lsp::Position;
    fn byte_to_tree_sitter_point(&self, offset: usize) -> tree_sitter::Point;
}

impl RopeExt for Rope {
    #[allow(unsafe_code)]
    fn chunk_walker(self, byte_idx: usize) -> ChunkWalker {
        let this: &'static Rope = unsafe { std::mem::transmute::<_, _>(&self) };
        let (mut chunks, chunk_byte_idx, ..) = this.chunks_at_byte(byte_idx);
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
