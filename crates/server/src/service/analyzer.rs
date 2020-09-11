//! Computes queries about documents.

use crate::core::{
    document::Document,
    language::{wast, wat},
    session::Session,
};
use std::sync::Arc;
use tower_lsp::lsp_types::*;

#[derive(Copy, Clone, Debug)]
enum HoverComputeStatus {
    Done,
    Next,
}

// FIXME
pub(crate) async fn hover_with_session(session: Arc<Session>, params: HoverParams) -> anyhow::Result<Option<Hover>> {
    let HoverParams {
        text_document_position_params:
            TextDocumentPositionParams {
                text_document: TextDocumentIdentifier { uri, .. },
                ..
            },
        ..
    } = &params;
    let document = session.get_document(uri).await?;
    let hover = hover_with_document(&document, &params).await?;
    Ok(hover)
}

/// Compute "textDocument/hover" for a given document.
pub async fn hover_with_document(document: &Document, params: &HoverParams) -> anyhow::Result<Option<Hover>> {
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
async fn hover_for_token_range(uri: &Url, document: &Document, range: Range) -> anyhow::Result<Option<Hover>> {
    let files = codespan_reporting::files::SimpleFile::new(uri, &document.text);
    let file_id = ();
    let start = codespan_lsp::position_to_byte_index(&files, file_id, &range.start)?;
    let end = codespan_lsp::position_to_byte_index(&files, file_id, &range.end)?;

    let mut contents = vec![];
    let mut range = None;

    let tree = document.tree.lock().await;
    let node = tree.root_node();

    if let Some(mut child) = node.descendant_for_byte_range(start, end) {
        use self::HoverComputeStatus::*;
        loop {
            if let Done = try_hover_for_instr_plain(&document, &child, &mut contents, &mut range)? {
                break;
            }

            if let Done = try_hover_for_instr(&document, &child, &mut contents, &mut range)? {
                break;
            }

            if let Done = try_hover_for_module_field(&document, &child, &mut contents, &mut range)? {
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

fn try_hover_for_instr(
    document: &Document,
    child: &tree_sitter::Node<'_>,
    contents: &mut Vec<MarkedString>,
    range: &mut Option<Range>,
) -> anyhow::Result<HoverComputeStatus> {
    if [*wat::kind::INSTR, *wast::kind::INSTR].contains(&child.kind_id()) {
        let text = child.utf8_text(&document.text.as_bytes())?;
        contents.push(MarkedString::String(String::from(text)));
        *range = Some(crate::util::node::range(&child));
        Ok(HoverComputeStatus::Done)
    } else {
        Ok(HoverComputeStatus::Next)
    }
}

fn try_hover_for_instr_plain(
    document: &Document,
    child: &tree_sitter::Node<'_>,
    contents: &mut Vec<MarkedString>,
    range: &mut Option<Range>,
) -> anyhow::Result<HoverComputeStatus> {
    if [*wat::kind::INSTR_PLAIN, *wast::kind::INSTR_PLAIN].contains(&child.kind_id()) {
        let text = child.utf8_text(&document.text.as_bytes())?;
        contents.push(MarkedString::String(String::from(text)));
        *range = Some(crate::util::node::range(&child));
        Ok(HoverComputeStatus::Done)
    } else {
        Ok(HoverComputeStatus::Next)
    }
}

fn try_hover_for_module_field(
    document: &Document,
    child: &tree_sitter::Node<'_>,
    contents: &mut Vec<MarkedString>,
    range: &mut Option<Range>,
) -> anyhow::Result<HoverComputeStatus> {
    if [*wat::kind::MODULE_FIELD, *wast::kind::MODULE_FIELD].contains(&child.kind_id()) {
        let text = child.utf8_text(&document.text.as_bytes())?;
        contents.push(MarkedString::String(String::from(text)));
        *range = Some(crate::util::node::range(&child));
        Ok(HoverComputeStatus::Done)
    } else {
        Ok(HoverComputeStatus::Next)
    }
}
