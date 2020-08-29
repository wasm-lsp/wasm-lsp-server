//! Computes queries about documents.

use crate::core::{document::Document, session::Session};
use std::sync::Arc;
use tower_lsp::lsp_types::*;
use wasm_language_server_parsers::core::language::{wast, wat};
use wasm_language_server_shared::core::error::{Error, Fallible};

// FIXME
pub(crate) async fn hover(session: Arc<Session>, params: HoverParams) -> Fallible<Option<Hover>> {
    let HoverParams {
        text_document_position_params:
            TextDocumentPositionParams {
                text_document: TextDocumentIdentifier { uri, .. },
                position,
                ..
            },
        ..
    } = &params;
    if let Some(document) = session.get_document(uri).await? {
        let range = Range::new(position.clone(), position.clone());
        let hover = hover_for_token_range(&uri, &document, range).await?;
        Ok(hover)
    } else {
        Err(Error::DocumentNotFound(uri.clone()).into())
    }
}

// FIXME
async fn hover_for_token_range(uri: &Url, document: &Document, range: Range) -> Fallible<Option<Hover>> {
    let files = codespan_reporting::files::SimpleFile::new(uri, &document.text);
    let file_id = ();
    let start = codespan_lsp::position_to_byte_index(&files, file_id, &range.start)?;
    let end = codespan_lsp::position_to_byte_index(&files, file_id, &range.end)?;

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

            if [*wat::kind::MODULE_FIELD, *wast::kind::MODULE_FIELD].contains(&child.kind_id()) {
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
