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
    #![allow(non_snake_case)]

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
        struct Data<'a> {
            children_count: usize,
            kind: SymbolKind,
            name: &'a str,
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
        // FIXME: tune this
        let mut data = vec![];
        let mut work = vec![Work::Node(node)];

        // Pre-compute ids for names to avoid repeated string matching.

        let language = tree.language();

        let kind_COMMAND = language.id_for_node_kind("command", true);
        let kind_ENTRYPOINT = language.id_for_node_kind("ENTRYPOINT", true);
        let kind_FUNC = language.id_for_node_kind("func", true);
        let kind_MODULE = language.id_for_node_kind("module", true);
        let kind_TYPE = language.id_for_node_kind("type", true);

        let field_COMMAND = language.field_id_for_name("command").unwrap();
        let field_FIELD = language.field_id_for_name("field").unwrap();
        let field_ID = language.field_id_for_name("id").unwrap();

        while let Some(next) = work.pop() {
            match next {
                // Construct a DocumentSymbol and pop data stack
                Work::Data => {
                    if let Some(Data {
                        children_count,
                        kind,
                        name,
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
                                Some(results.drain(results.len() - children_count ..).collect())
                            },
                            deprecated: Default::default(),
                            detail: Default::default(),
                            kind,
                            name: name.to_string(),
                            range,
                            selection_range,
                        };
                        results.push(this);
                    }
                },

                Work::Node(node) if node.kind_id() == kind_ENTRYPOINT => {
                    let mut cursor = node.walk();
                    let commands = node.children_by_field_id(field_COMMAND, &mut cursor).map(Work::Node);
                    work.extend(commands);
                },

                Work::Node(node) if node.kind_id() == kind_COMMAND => {
                    let command = node.named_child(0).expect("'command' should have a single named child");
                    work.push(Work::Node(command));
                },

                Work::Node(node) if node.kind_id() == kind_FUNC => {
                    let NameAndRanges {
                        name,
                        range,
                        selection_range,
                    } = crate::lsp::node::name_and_ranges(&document.text.as_bytes(), &node, field_ID);
                    work.push(Work::Data);
                    data.push(Data {
                        children_count: 0,
                        kind: SymbolKind::Function,
                        name,
                        range,
                        selection_range,
                    });
                },

                Work::Node(node) if node.kind_id() == kind_MODULE => {
                    let NameAndRanges {
                        name,
                        range,
                        selection_range,
                    } = crate::lsp::node::name_and_ranges(&document.text.as_bytes(), &node, field_ID);
                    work.push(Work::Data);

                    let mut children_count = 0;
                    for modulefield in node
                        .children_by_field_id(field_FIELD, &mut node.walk())
                        .filter(|node| node.kind_id() == kind_FUNC || node.kind_id() == kind_TYPE)
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
                },

                Work::Node(node) if node.kind_id() == kind_TYPE => {
                    let NameAndRanges {
                        name,
                        range,
                        selection_range,
                    } = crate::lsp::node::name_and_ranges(&document.text.as_bytes(), &node, field_ID);
                    work.push(Work::Data);
                    data.push(Data {
                        children_count: 0,
                        kind: SymbolKind::TypeParameter,
                        name,
                        range,
                        selection_range,
                    });
                },

                _ => {},
            }
        }
        response = Some(DocumentSymbolResponse::Nested(results));
    }
    Ok(response)
}
