//! Various utility functionality, e.g., for handling lsp or tree-sitter data.

pub(crate) mod character {
    pub(crate) mod line {
        use crate::core::error::Error;

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
                Err(Error::ColumnOutOfBounds {
                    given: offset as usize,
                    max: line_text.len(),
                }
                .into())
            }
        }
    }
}

pub(crate) mod line {
    use crate::core::{document::Document, error::Error};

    pub(crate) fn range(document: &Document, line_index: usize) -> Option<std::ops::Range<usize>> {
        let (start, end) = super::line::span(document, line_index).ok()?;
        Some(start .. end)
    }

    fn span(document: &Document, line_index: usize) -> anyhow::Result<(usize, usize)> {
        let line_starts = super::line::starts(document).collect::<Vec<_>>();
        let this_start = super::line::start(document, &line_starts, line_index)?;
        let next_start = super::line::start(document, &line_starts, line_index + 1)?;
        Ok((this_start, next_start))
    }

    pub(crate) fn start(document: &Document, line_starts: &[usize], line_index: usize) -> anyhow::Result<usize> {
        use std::cmp::Ordering;
        match line_index.cmp(&line_starts.len()) {
            Ordering::Less => Ok(line_starts[line_index]),
            Ordering::Equal => Ok(document.text.len()),
            Ordering::Greater => Err(Error::LineOutOfBounds {
                given: line_index,
                max: line_starts.len(),
            }
            .into()),
        }
    }

    pub(crate) fn starts<'a>(document: &'a Document) -> impl 'a + Iterator<Item = usize> {
        let source = document.text.as_str();
        std::iter::once(0).chain(source.match_indices('\n').map(|i| i.0 + 1))
    }
}

/// Functions for creation of lsp data from tree-sitter nodes.
pub(crate) mod node {
    /// Functions for creation of lsp position data from tree-sitter nodes.
    mod position {
        /// Creates an lsp position from the starting position of a tree-sitter node.
        pub(crate) fn start(node: &tree_sitter::Node) -> lspower::lsp_types::Position {
            let tree_sitter::Point { row, column } = node.start_position();
            lspower::lsp_types::Position::new(row as u32, column as u32)
        }

        /// Creates an lsp position from the ending position of a tree-sitter node.
        pub(crate) fn end(node: &tree_sitter::Node) -> lspower::lsp_types::Position {
            let tree_sitter::Point { row, column } = node.end_position();
            lspower::lsp_types::Position::new(row as u32, column as u32)
        }
    }

    /// Creates an lsp range from the range of a tree-sitter node.
    pub(crate) fn range(node: &tree_sitter::Node) -> lspower::lsp_types::Range {
        lspower::lsp_types::Range::new(position::start(node), position::end(node))
    }
}

pub(crate) mod position {
    use crate::core::document::Document;
    use lspower::lsp_types::*;

    pub(crate) fn byte_index(document: &Document, position: &Position) -> anyhow::Result<usize> {
        let source = document.text.as_str();
        let line_span: std::ops::Range<usize> = super::line::range(document, position.line as usize).unwrap();
        let line_text = source.get(line_span.clone()).unwrap();
        let character = position.character as usize;
        let byte_offset = super::character::line::offset(line_text, character)?;
        Ok(line_span.start + byte_offset)
    }
}
