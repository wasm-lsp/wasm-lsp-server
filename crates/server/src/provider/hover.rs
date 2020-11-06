//! Provides `textDocument/hover` functionality.

use crate::core::{
    document::Document,
    error::Error,
    language::{wast, wat},
};
use tower_lsp::lsp_types::*;

// FIXME: move to util
fn character_to_line_offset(line_text: &str, character: u64) -> anyhow::Result<usize> {
    let mut offset = 0;

    let mut chars = line_text.chars();
    while let Some(c) = chars.next() {
        if offset == character {
            return Ok(line_text.len() - chars.as_str().len() - c.len_utf8());
        }
        offset += c.len_utf16() as u64;
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

// FIXME: move to util
fn line_range(document: &Document, line_index: usize) -> Option<std::ops::Range<usize>> {
    let (start, end) = line_span(document, line_index).ok()?;
    Some(start .. end)
}

// FIXME: move to util
fn line_span(document: &Document, line_index: usize) -> anyhow::Result<(usize, usize)> {
    let line_starts = line_starts(document).collect::<Vec<_>>();
    let this_start = line_start(document, &line_starts, line_index)?;
    let next_start = line_start(document, &line_starts, line_index + 1)?;
    Ok((this_start, next_start))
}

// FIXME: move to util
fn line_start(document: &Document, line_starts: &[usize], line_index: usize) -> anyhow::Result<usize> {
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

// FIXME: move to util
fn line_starts<'a>(document: &'a Document) -> impl 'a + Iterator<Item = usize> {
    let source = document.text.as_str();
    std::iter::once(0).chain(source.match_indices('\n').map(|i| i.0 + 1))
}

// FIXME: move to util
fn position_to_byte_index(document: &Document, position: &Position) -> anyhow::Result<usize> {
    let source = document.text.as_str();
    let line_span: std::ops::Range<usize> = line_range(document, position.line as usize).unwrap();
    let line_text = source.get(line_span.clone()).unwrap();
    let byte_offset = character_to_line_offset(line_text, position.character)?;
    Ok(line_span.start + byte_offset)
}

/// Compute "textDocument/hover" for a given document.
pub async fn response(document: &Document, params: &HoverParams) -> anyhow::Result<Option<Hover>> {
    let HoverParams {
        text_document_position_params:
            TextDocumentPositionParams {
                text_document: TextDocumentIdentifier { uri, .. },
                position,
                ..
            },
        ..
    } = params;
    let range = Range::new(*position, *position);
    let hover = hover_for_token_range(&uri, &document, range).await?;
    Ok(hover)
}

// FIXME
async fn hover_for_token_range(_uri: &Url, document: &Document, range: Range) -> anyhow::Result<Option<Hover>> {
    let module_fields: &[u16] = &[
        *wast::kind::MODULE_FIELD_DATA,
        *wast::kind::MODULE_FIELD_ELEM,
        *wast::kind::MODULE_FIELD_FUNC,
        *wast::kind::MODULE_FIELD_GLOBAL,
        *wast::kind::MODULE_FIELD_MEMORY,
        *wast::kind::MODULE_FIELD_TABLE,
        *wast::kind::MODULE_FIELD_TYPE,
        *wat::kind::MODULE_FIELD_DATA,
        *wat::kind::MODULE_FIELD_ELEM,
        *wat::kind::MODULE_FIELD_FUNC,
        *wat::kind::MODULE_FIELD_GLOBAL,
        *wat::kind::MODULE_FIELD_MEMORY,
        *wat::kind::MODULE_FIELD_TABLE,
        *wat::kind::MODULE_FIELD_TYPE,
    ];

    let start = position_to_byte_index(document, &range.start)?;
    let end = position_to_byte_index(document, &range.end)?;

    let mut contents = vec![];
    let mut range = None;

    let tree = document.tree.lock().await;
    let node = tree.root_node();

    if let Some(mut child) = node.descendant_for_byte_range(start, end) {
        loop {
            if [*wat::kind::INSTR_PLAIN, *wast::kind::INSTR_PLAIN].contains(&child.kind_id()) {
                let text = child.utf8_text(&document.text.as_bytes())?;
                contents.push(MarkedString::String(String::from(text)));
                range = Some(crate::util::node::range(&child));
                break;
            }

            if [*wat::kind::INSTR, *wast::kind::INSTR].contains(&child.kind_id()) {
                let text = child.utf8_text(&document.text.as_bytes())?;
                contents.push(MarkedString::String(String::from(text)));
                range = Some(crate::util::node::range(&child));
                break;
            }

            if module_fields.contains(&child.kind_id()) {
                let text = child.utf8_text(&document.text.as_bytes())?;
                contents.push(MarkedString::String(String::from(text)));
                range = Some(crate::util::node::range(&child));
                break;
            }

            if let Some(parent) = child.parent() {
                child = parent;
            } else {
                break;
            }
        }
    }

    if contents.is_empty() {
        Ok(None)
    } else {
        Ok(Some(Hover {
            contents: HoverContents::Array(contents),
            range,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::document::Document;

    #[test]
    fn character_to_line_offset_ok() {
        let line_text = "text";
        let character = line_text.len() as u64;
        let result = character_to_line_offset(line_text, character);
        assert!(result.is_ok())
    }

    #[test]
    fn character_to_line_offset_out_of_bounds() {
        let line_text = "text";
        let character = line_text.len() as u64 + 1;
        let result = character_to_line_offset(line_text, character);
        assert!(result.is_err())
    }

    #[test]
    fn line_start_ok() {
        let language_id = "wasm.wast";
        let text = String::from("(module)");
        let result = Document::new(language_id, text);
        assert!(result.is_ok());
        if let Ok(option) = result {
            assert!(option.is_some());
            if let Some(ref document) = option {
                let line_starts = line_starts(document).collect::<Vec<_>>();
                let line_index = 1;
                let result = line_start(document, &line_starts, line_index);
                assert!(result.is_ok())
            }
        }
    }

    #[test]
    fn line_start_out_of_bounds() {
        let language_id = "wasm.wast";
        let text = String::from("(module)");
        let result = Document::new(language_id, text);
        assert!(result.is_ok());
        if let Ok(option) = result {
            assert!(option.is_some());
            if let Some(ref document) = option {
                let line_starts = line_starts(document).collect::<Vec<_>>();
                let line_index = 2;
                let result = line_start(document, &line_starts, line_index);
                assert!(result.is_err())
            }
        }
    }
}
