/// Elaborates parse trees into structured data to be cached in the database.
use crate::document::Document;
use crate::error::Error;
use dashmap::DashMap;
use failure::Fallible;
use lsp_types::*;
use std::sync::Arc;
use tower_lsp::Client;
use tree_sitter::QueryMatch;

mod queries {
    pub mod document_symbol {
        pub static QUERY: &str = r"
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

        pub mod patterns {
            pub static MODULE: usize = 0;
            pub static IMPORT: usize = 1;
            pub static TYPE: usize = 2;
            pub static FUNC: usize = 3;
        }
    }
}

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
            let source = queries::document_symbol::QUERY;
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

            // prepare the stack machine
            let mut stack: [Vec<&str>; 4] = [vec![], vec![], vec![], vec![]];
            let mut state: [u8; 4] = [0, 0, 0, 0];

            for (
                QueryMatch {
                    captures,
                    pattern_index,
                    ..
                },
                index,
            ) in matches
            {
                let capture = captures[index];
                let text = capture
                    .node
                    .utf8_text(&document.text.as_bytes())
                    .unwrap_or("")
                    .trim_start();

                stack[pattern_index].push(text);
                state[pattern_index] += 1;

                log::info!(
                    "pattern: {}, capture: {}, row: {}, text: {:?}",
                    pattern_index,
                    &query.capture_names()[capture.index as usize],
                    capture.node.start_position().row,
                    text
                );

                if pattern_index == queries::document_symbol::patterns::MODULE {
                    results.push({
                        SymbolInformation {
                            name: String::from(text),
                            kind: SymbolKind::Module,
                            deprecated: Default::default(),
                            location: crate::lsp::node::location(uri.clone(), &node),
                            container_name: Default::default(),
                        }
                    });
                }

                if pattern_index == queries::document_symbol::patterns::IMPORT
                    && state[pattern_index].wrapping_rem_euclid(4) == 0
                {
                    if let [module, import_module, import_item, importdesc] =
                        stack[pattern_index].drain(.. 4).collect::<Vec<_>>()[..]
                    {
                        log::info!("{}, {}, {}, {}", module, import_module, import_item, importdesc);
                    }
                }

                if pattern_index == queries::document_symbol::patterns::TYPE
                    && state[pattern_index].wrapping_rem_euclid(2) == 0
                {
                    if let [module, type_] = stack[pattern_index].drain(.. 2).collect::<Vec<_>>()[..] {
                        log::info!("{}, {}", module, type_);
                        results.push({
                            SymbolInformation {
                                name: String::from(type_),
                                kind: SymbolKind::TypeParameter,
                                deprecated: Default::default(),
                                location: crate::lsp::node::location(uri.clone(), &node),
                                container_name: Some(String::from(module)),
                            }
                        });
                    }
                }

                if pattern_index == queries::document_symbol::patterns::FUNC
                    && state[pattern_index].wrapping_rem_euclid(2) == 0
                {
                    if let [module, func] = stack[pattern_index].drain(.. 2).collect::<Vec<_>>()[..] {
                        log::info!("{}, {}", module, func);
                        results.push({
                            SymbolInformation {
                                name: String::from(func),
                                kind: SymbolKind::Function,
                                deprecated: Default::default(),
                                location: crate::lsp::node::location(uri.clone(), &node),
                                container_name: Some(String::from(module)),
                            }
                        });
                    }
                }
            }

            response = Some(DocumentSymbolResponse::Flat(results));
        }
    }
    Ok(response)
}
