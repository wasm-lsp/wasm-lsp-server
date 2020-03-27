/// Elaborates parse trees into structured data to be cached in the database.
use crate::document::Document;
use dashmap::DashMap;
use failure::Fallible;
use lsp_types::*;
use std::sync::Arc;
use tower_lsp::Client;

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

#[allow(dead_code)]
pub async fn tree_did_open(documents: Arc<DashMap<Url, Document>>, client: &Client, uri: Url) -> Fallible<()> {
    self::tree_did_change(documents, client, uri).await
}

// FIXME: reorganize this to where outline is pulled from database
pub async fn document_symbol(
    documents: Arc<DashMap<Url, Document>>,
    params: DocumentSymbolParams,
) -> jsonrpc_core::Result<Option<DocumentSymbolResponse>> {
    let DocumentSymbolParams {
        text_document: TextDocumentIdentifier { uri },
    } = params;

    // Prepare the response.
    let mut response = None;

    // Attempt to obtain the document.
    if let Some(document) = documents.get(&uri) {
        let mut results: Vec<DocumentSymbol> = vec![];

        // Prepare the syntax tree.
        let tree = document.tree.lock().await.clone();
        let node = tree.root_node();

        // Define local data structures for the stack machine.
        #[derive(Clone, Debug)]
        struct Data {
            children_count: usize,
            kind: SymbolKind,
            name: String,
            range: Range,
            selection_range: Range,
        }
        #[derive(Debug)]
        enum Work<'a> {
            Data,
            Node(tree_sitter::Node<'a>),
        }
        use crate::lsp::node::NameAndRanges;

        // Prepare the stack machine:
        //   data: contains data for constructing upcoming DocumentSymbols
        //   work: contains remaining tree_sitter nodes to process
        let mut data = vec![];
        let mut work = vec![Work::Node(node)];

        // FIXME: move these somewhere else
        #[allow(non_snake_case)]
        let START = tree.language().id_for_node_kind("START", true);
        #[allow(non_snake_case)]
        let FUNC = tree.language().id_for_node_kind("func", true);
        #[allow(non_snake_case)]
        let MODULE = tree.language().id_for_node_kind("module", true);
        #[allow(non_snake_case)]
        let TYPE = tree.language().id_for_node_kind("type", true);

        while let Some(next) = work.pop() {
            match next {
                // Construct a DocumentSymbol and pop data stack
                Work::Data => {
                    if let Some(Data {
                        children_count,
                        kind,
                        ref name,
                        range,
                        selection_range,
                    }) = data.pop()
                    {
                        let this = DocumentSymbol {
                            children: if results.is_empty() {
                                None
                            } else {
                                // Drain the results array by the number of children nodes we counted for this
                                // DocumentSymbol. This allows us to properly reconstruct symbol nesting.
                                Some(results.drain(.. children_count).collect())
                            },
                            deprecated: Default::default(),
                            detail: Default::default(),
                            kind,
                            name: name.clone(),
                            range,
                            selection_range,
                        };
                        results.push(this);
                    }
                },

                Work::Node(node) => {
                    if node.kind_id() == START {
                        if let Some(module) = node.child_by_field_name("module") {
                            work.push(Work::Node(module));
                        }
                        continue;
                    }

                    if node.kind_id() == FUNC {
                        let NameAndRanges {
                            name,
                            range,
                            selection_range,
                        } = crate::lsp::node::name_and_ranges(&document.text.as_bytes(), &node, "id", Some("trim"));
                        work.push(Work::Data);
                        data.push(Data {
                            children_count: 0,
                            kind: SymbolKind::Function,
                            name,
                            range,
                            selection_range,
                        });
                        continue;
                    }

                    if node.kind_id() == MODULE {
                        let NameAndRanges {
                            name,
                            range,
                            selection_range,
                        } = crate::lsp::node::name_and_ranges(&document.text.as_bytes(), &node, "id", Some("trim"));
                        work.push(Work::Data);

                        let mut children_count = 0;
                        for modulefield in node
                            .children_by_field_name("modulefield", &mut node.walk())
                            .filter(|node| match node.kind() {
                                "func" => true,
                                "type" => true,
                                _ => false,
                            })
                        {
                            work.push(Work::Node(modulefield));
                            children_count += 1;
                        }

                        data.push(Data {
                            children_count,
                            kind: SymbolKind::Module,
                            name,
                            range,
                            selection_range,
                        });
                        continue;
                    }

                    if node.kind_id() == TYPE {
                        let NameAndRanges {
                            name,
                            range,
                            selection_range,
                        } = crate::lsp::node::name_and_ranges(&document.text.as_bytes(), &node, "id", Some("trim"));
                        work.push(Work::Data);
                        data.push(Data {
                            children_count: 0,
                            kind: SymbolKind::TypeParameter,
                            name,
                            range,
                            selection_range,
                        });
                        continue;
                    }
                },
            }
        }
        response = Some(DocumentSymbolResponse::Nested(results));
    }
    Ok(response)
}
