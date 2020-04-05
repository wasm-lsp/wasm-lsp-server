//! Elaborator definitions specific to ".wast" files.

use crate::core::session::Session;
use lsp_types::*;
use std::sync::Arc;

/// Compute the symbols for a given document.
pub(crate) async fn document_symbol(
    session: Arc<Session>,
    params: DocumentSymbolParams,
) -> jsonrpc_core::Result<Option<DocumentSymbolResponse>> {
    #![allow(non_snake_case)]

    let DocumentSymbolParams {
        text_document: TextDocumentIdentifier { uri },
    } = params;

    // Prepare the response.
    let mut response = None;

    // Attempt to obtain the document.
    if let Some(document) = session.documents.get(&uri) {
        use crate::{
            service::elaborator::document_symbol::{Data, Work},
            util::node::NameAndRanges,
        };

        let mut syms: Vec<DocumentSymbol> = vec![];

        // Prepare the syntax tree.
        let tree = document.tree.lock().await.clone();
        let node = tree.root_node();

        // Prepare the stack machine:
        //   data: contains data for constructing upcoming DocumentSymbols
        //   work: contains remaining tree_sitter nodes to process
        // FIXME: tune this
        let mut data = vec![];
        let mut work = vec![Work::Node(node)];

        // Pre-compute ids for names to avoid repeated string matching.
        let language = tree.language();

        let kind_COMMAND = language.id_for_node_kind("command", true);
        let kind_DATA = language.id_for_node_kind("data", true);
        let kind_ELEM = language.id_for_node_kind("elem", true);
        let kind_ENTRYPOINT = language.id_for_node_kind("ENTRYPOINT", true);
        let kind_FUNC = language.id_for_node_kind("func", true);
        let kind_GLOBAL = language.id_for_node_kind("global", true);
        let kind_MEMORY = language.id_for_node_kind("mem", true);
        let kind_MODULE = language.id_for_node_kind("module", true);
        let kind_TABLE = language.id_for_node_kind("table", true);
        let kind_TYPE = language.id_for_node_kind("type", true);

        let field_COMMAND = language.field_id_for_name("command").unwrap();
        let field_FIELD = language.field_id_for_name("field").unwrap();
        let field_ID = language.field_id_for_name("id").unwrap();

        let modulefield_filter = |node: &tree_sitter::Node| {
            [
                kind_DATA,
                kind_ELEM,
                kind_FUNC,
                kind_GLOBAL,
                kind_MEMORY,
                kind_TABLE,
                kind_TYPE,
            ]
            .contains(&node.kind_id())
        };

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
                            children: if syms.is_empty() {
                                None
                            } else {
                                // Drain the syms array by the number of children nodes we counted for this
                                // DocumentSymbol. This allows us to properly reconstruct symbol nesting.
                                let children = syms.drain(syms.len() - children_count ..);
                                // Process the nodes in reverse (because tree-sitter returns later nodes first).
                                let children = children.rev();
                                Some(children.collect())
                            },
                            deprecated: Default::default(),
                            detail: Default::default(),
                            kind,
                            name: name.to_string(),
                            range,
                            selection_range,
                        };
                        syms.push(this);
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

                Work::Node(node) if node.kind_id() == kind_DATA => {
                    let NameAndRanges {
                        name,
                        range,
                        selection_range,
                    } = crate::util::node::name_and_ranges(&document.text.as_bytes(), "<data>", &node, field_ID);
                    work.push(Work::Data);
                    data.push(Data {
                        children_count: 0,
                        kind: SymbolKind::Key,
                        name,
                        range,
                        selection_range,
                    });
                },

                Work::Node(node) if node.kind_id() == kind_ELEM => {
                    let NameAndRanges {
                        name,
                        range,
                        selection_range,
                    } = crate::util::node::name_and_ranges(&document.text.as_bytes(), "<elem>", &node, field_ID);
                    work.push(Work::Data);
                    data.push(Data {
                        children_count: 0,
                        kind: SymbolKind::Field,
                        name,
                        range,
                        selection_range,
                    });
                },

                Work::Node(node) if node.kind_id() == kind_FUNC => {
                    let NameAndRanges {
                        name,
                        range,
                        selection_range,
                    } = crate::util::node::name_and_ranges(&document.text.as_bytes(), "<func>", &node, field_ID);
                    work.push(Work::Data);
                    data.push(Data {
                        children_count: 0,
                        kind: SymbolKind::Function,
                        name,
                        range,
                        selection_range,
                    });
                },

                Work::Node(node) if node.kind_id() == kind_GLOBAL => {
                    let NameAndRanges {
                        name,
                        range,
                        selection_range,
                    } = crate::util::node::name_and_ranges(&document.text.as_bytes(), "<global>", &node, field_ID);
                    work.push(Work::Data);
                    data.push(Data {
                        children_count: 0,
                        kind: SymbolKind::Event,
                        name,
                        range,
                        selection_range,
                    });
                },

                Work::Node(node) if node.kind_id() == kind_MEMORY => {
                    let NameAndRanges {
                        name,
                        range,
                        selection_range,
                    } = crate::util::node::name_and_ranges(&document.text.as_bytes(), "<memory>", &node, field_ID);
                    work.push(Work::Data);
                    data.push(Data {
                        children_count: 0,
                        kind: SymbolKind::Array,
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
                    } = crate::util::node::name_and_ranges(&document.text.as_bytes(), "<module>", &node, field_ID);
                    work.push(Work::Data);

                    let mut children_count = 0;
                    for modulefield in node
                        .children_by_field_id(field_FIELD, &mut node.walk())
                        .filter(modulefield_filter)
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

                Work::Node(node) if node.kind_id() == kind_TABLE => {
                    let NameAndRanges {
                        name,
                        range,
                        selection_range,
                    } = crate::util::node::name_and_ranges(&document.text.as_bytes(), "<table>", &node, field_ID);
                    work.push(Work::Data);
                    data.push(Data {
                        children_count: 0,
                        kind: SymbolKind::Interface,
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
                    } = crate::util::node::name_and_ranges(&document.text.as_bytes(), "<type>", &node, field_ID);
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
        // Collect the syms vec into a new vec in reverse so that document symbols are returned in the
        // correct order. Note that children nodes are reversed _as the symbols are nested_.
        let results = syms.into_iter().rev().collect();

        response = Some(DocumentSymbolResponse::Nested(results));
    }
    Ok(response)
}
