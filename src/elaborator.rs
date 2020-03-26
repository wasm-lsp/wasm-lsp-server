/// Elaborates parse trees into structured data to be cached in the database.
use crate::document::Document;
use crate::error::Error;
use dashmap::DashMap;
use failure::Fallible;
use lsp_types::*;
use std::sync::Arc;
use tower_lsp::Client;

pub async fn tree_did_change(documents: Arc<DashMap<Url, Document>>, _: Client, uri: Url) -> Fallible<()> {
    if let Some(document) = documents.get(&uri) {
        let tree = document.tree.lock().await.clone();
        let node = tree.root_node();
        if !node.has_error() {
            log::info!("syntax well-formed");
        }
        // NOTE: else let auditor handle
        // TODO: allow partial elaboration in presence of syntax errors
    }
    Ok(())
}

pub async fn tree_did_close(_: Arc<DashMap<Url, Document>>, _: Client, _: Url) -> Fallible<()> {
    Ok(())
}

pub async fn tree_did_open(documents: Arc<DashMap<Url, Document>>, client: Client, uri: Url) -> Fallible<()> {
    self::tree_did_change(documents, client, uri).await
}

// FIXME: reorganize this to where outline is pulled from database
// FIXME: generalize this to handle all symbol kinds (more than module nodes)
pub async fn document_symbol(
    documents: Arc<DashMap<Url, Document>>,
    params: DocumentSymbolParams,
) -> jsonrpc_core::Result<Option<DocumentSymbolResponse>> {
    let DocumentSymbolParams {
        text_document: TextDocumentIdentifier { uri },
    } = params;
    let mut response = None;
    if let Some(document) = documents.get(&uri) {
        let mut results: Vec<SymbolInformation> = vec![];
        let tree = document.tree.lock().await.clone();
        let node = tree.root_node();

        // TODO: allow partial elaboration in presence of syntax errors
        if !node.has_error() {
            // prepare a query to match tree-sitter module nodes
            let language = tree.language();
            let source = "(module (id) @module-id)";
            let query = tree_sitter::Query::new(language, source).map_err(|err| {
                let code = jsonrpc_core::ErrorCode::InternalError;
                let message = format!("{}", Error::QueryError(err));
                let data = None;
                jsonrpc_core::Error { code, message, data }
            })?;

            // prepare a query cursor
            let mut query_cursor = tree_sitter::QueryCursor::new();
            let text_callback = |node: tree_sitter::Node| &document.text[node.byte_range()];
            let matches = query_cursor.matches(&query, node, text_callback);

            // iterate the query cursor and construct appropriate lsp diagnostics
            for tree_sitter::QueryMatch { captures, .. } in matches {
                for tree_sitter::QueryCapture { node, .. } in captures {
                    let start = node.start_position();
                    let end = node.end_position();
                    results.push({
                        let name = String::from(node.utf8_text(&document.text.as_bytes()).map_err(|err| {
                            let code = jsonrpc_core::ErrorCode::InternalError;
                            let message = format!("{}", Error::Utf8Error(err));
                            let data = None;
                            jsonrpc_core::Error { code, message, data }
                        })?);
                        let kind = SymbolKind::Module;
                        let deprecated = None;
                        let location = {
                            let uri = uri.clone();
                            let range = {
                                let start = Position::new(start.row as u64, start.column as u64);
                                let end = Position::new(end.row as u64, end.column as u64);
                                Range::new(start, end)
                            };
                            Location { uri, range }
                        };
                        let container_name = None;
                        SymbolInformation {
                            name,
                            kind,
                            deprecated,
                            location,
                            container_name,
                        }
                    });
                }
            }
            response = Some(DocumentSymbolResponse::Flat(results));
        }
    }
    Ok(response)
}
