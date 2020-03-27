/// Elaborates parse trees into structured data to be cached in the database.
use crate::document::Document;
use crate::error::Error;
use dashmap::DashMap;
use failure::Fallible;
use lsp_types::*;
use std::sync::Arc;
use tower_lsp::Client;

static QUERY: &str = r"
(module
    (id) @module)

(module
    (id) @module
    (modulefield
        (import
            (name) @import-module
            (name) @import-item
            (importdesc
                (id) @importdesc))))

(module
    (id) @module
    (modulefield
        (type
            (id) @type)))

(module
    (id) @module
    (modulefield
        (func
            (id) @func)))
";

pub async fn tree_did_change(documents: Arc<DashMap<Url, Document>>, _: &Client, uri: Url) -> Fallible<()> {
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

pub async fn tree_did_close(_: Arc<DashMap<Url, Document>>, _: &Client, _: Url) -> Fallible<()> {
    Ok(())
}

pub async fn tree_did_open(documents: Arc<DashMap<Url, Document>>, client: &Client, uri: Url) -> Fallible<()> {
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
            let source = QUERY;
            let language = tree.language();
            let query = tree_sitter::Query::new(language, source).map_err(|err| {
                let code = jsonrpc_core::ErrorCode::InternalError;
                let message = format!("{}", Error::QueryError(err));
                let data = None;
                jsonrpc_core::Error { code, message, data }
            })?;

            // prepare a query cursor
            let mut query_cursor = tree_sitter::QueryCursor::new();
            let text_callback = |node: tree_sitter::Node| &document.text[node.byte_range()];
            let matches = query_cursor.captures(&query, node, text_callback);

            // FIXME: make this less redundant

            let mut data1 = Vec::<&str>::new();
            let mut data2 = Vec::<&str>::new();
            let mut data3 = Vec::<&str>::new();

            let mut state1 = 0u8;
            let mut state2 = 0u8;
            let mut state3 = 0u8;

            for (mat, index) in matches {
                let capture = mat.captures[index];
                let text = capture
                    .node
                    .utf8_text(&document.text.as_bytes())
                    .unwrap_or("")
                    .trim_start();
                log::info!(
                    "pattern: {}, capture: {}, row: {}, text: {:?}",
                    mat.pattern_index,
                    &query.capture_names()[capture.index as usize],
                    capture.node.start_position().row,
                    text
                );

                if mat.pattern_index == 0 {
                    let start = node.start_position();
                    let end = node.end_position();
                    results.push({
                        let name = String::from(text);
                        let deprecated = Default::default();
                        let location = {
                            let uri = uri.clone();
                            let range = {
                                let start = Position::new(start.row as u64, start.column as u64);
                                let end = Position::new(end.row as u64, end.column as u64);
                                Range::new(start, end)
                            };
                            Location { uri, range }
                        };
                        let container_name = Default::default();
                        let kind = SymbolKind::Module;
                        SymbolInformation {
                            name,
                            kind,
                            deprecated,
                            location,
                            container_name,
                        }
                    });
                }

                if mat.pattern_index == 1 {
                    data1.push(text);
                    state1 += 1;
                    if state1.wrapping_rem_euclid(4) == 0 {
                        if let [module, import_module, import_item, importdesc] = data1.drain(.. 4).collect::<Vec<_>>()[..] {
                            log::info!("{}, {}, {}, {}", module, import_module, import_item, importdesc);
                        }
                    }
                }

                if mat.pattern_index == 2 {
                    data2.push(text);
                    state2 += 1;
                    if state2.wrapping_rem_euclid(2) == 0 {
                        if let [module, type_] = data2.drain(.. 2).collect::<Vec<_>>()[..] {
                            log::info!("{}, {}", module, type_);
                            let start = node.start_position();
                            let end = node.end_position();
                            results.push({
                                let name = String::from(type_);
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
                                let container_name = Some(String::from(module));
                                let kind = SymbolKind::TypeParameter;
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
                }

                if mat.pattern_index == 3 {
                    data3.push(text);
                    state3 += 1;
                    if state3.wrapping_rem_euclid(2) == 0 {
                        if let [module, func] = data3.drain(.. 2).collect::<Vec<_>>()[..] {
                            log::info!("{}, {}", module, func);
                            let start = node.start_position();
                            let end = node.end_position();
                            results.push({
                                let name = String::from(func);
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
                                let container_name = Some(String::from(module));
                                let kind = SymbolKind::Function;
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
                }
            }
            response = Some(DocumentSymbolResponse::Flat(results));
        }
    }
    Ok(response)
}
