//! Various utility functionality, e.g., for handling lsp or tree-sitter data.

pub(crate) mod character {
    pub(crate) mod line {
        use crate::core;

        pub(crate) fn offset(line_text: &str, character: usize) -> anyhow::Result<usize> {
            let mut offset = 0;

            let mut chars = line_text.chars();
            while let Some(c) = chars.next() {
                if offset == character {
                    return Ok(line_text.len() - chars.as_str().len() - c.len_utf8());
                }
                offset += c.len_utf16();
            }

            // Handle positions after the last character on the line
            if offset == character {
                Ok(line_text.len())
            } else {
                Err(core::Error::ColumnOutOfBounds {
                    given: offset as usize,
                    max: line_text.len(),
                }
                .into())
            }
        }
    }
}

pub(crate) mod line {
    use crate::core;

    pub(crate) fn range(document: &core::Document, line_index: usize) -> Option<std::ops::Range<usize>> {
        let (start, end) = super::line::span(document, line_index).ok()?;
        Some(start .. end)
    }

    fn span(document: &core::Document, line_index: usize) -> anyhow::Result<(usize, usize)> {
        let source = document.content.chunks().collect::<String>();
        let source = source.as_str();
        let line_starts = super::line::starts(source).collect::<Vec<_>>();
        let this_start = super::line::start(document, &line_starts, line_index)?;
        let next_start = super::line::start(document, &line_starts, line_index + 1)?;
        Ok((this_start, next_start))
    }

    pub(crate) fn start(document: &core::Document, line_starts: &[usize], line_index: usize) -> anyhow::Result<usize> {
        use std::cmp::Ordering;
        match line_index.cmp(&line_starts.len()) {
            Ordering::Less => Ok(line_starts[line_index]),
            Ordering::Equal => Ok(document.content.len_bytes()),
            Ordering::Greater => Err(core::Error::LineOutOfBounds {
                given: line_index,
                max: line_starts.len(),
            }
            .into()),
        }
    }

    pub(crate) fn starts(source: &str) -> impl '_ + Iterator<Item = usize> {
        std::iter::once(0).chain(source.match_indices('\n').map(|i| i.0 + 1))
    }
}

/// Functions for creation of lsp data from tree-sitter nodes.
pub(crate) mod node {
    /// Functions for creation of lsp position data from tree-sitter nodes.
    mod position {
        /// Creates an lsp position from the starting position of a tree-sitter node.
        pub(crate) fn start(node: &tree_sitter::Node) -> lsp::Position {
            let tree_sitter::Point { row, column } = node.start_position();
            lsp::Position::new(row as u32, column as u32)
        }

        /// Creates an lsp position from the ending position of a tree-sitter node.
        pub(crate) fn end(node: &tree_sitter::Node) -> lsp::Position {
            let tree_sitter::Point { row, column } = node.end_position();
            lsp::Position::new(row as u32, column as u32)
        }
    }

    /// Creates an lsp range from the range of a tree-sitter node.
    pub(crate) fn range(node: &tree_sitter::Node) -> lsp::Range {
        lsp::Range::new(position::start(node), position::end(node))
    }
}

pub(crate) mod position {
    use crate::core;

    pub(crate) fn byte_index(document: &core::Document, position: &lsp::Position) -> anyhow::Result<usize> {
        let source = document.content.chunks().collect::<String>();
        let source = source.as_str();
        let line_index = position.line as usize;
        let line_span: std::ops::Range<usize> = super::line::range(document, line_index).unwrap();
        let line_text = source.get(line_span.clone()).unwrap();
        let character = position.character as usize;
        let byte_offset = super::character::line::offset(line_text, character)?;
        Ok(line_span.start + byte_offset)
    }
}
