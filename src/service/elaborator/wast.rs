//! Elaborator definitions specific to ".wast" files.

use crate::{
    core::{language::wast, session::Session},
    service::elaborator::document_symbol::{self, Data, Work},
    util::node::{symbol_range, SymbolRange},
};
use lsp_types::*;
use std::sync::Arc;

/// Compute the symbols for a given document.
pub(crate) async fn document_symbol(
    session: Arc<Session>,
    params: DocumentSymbolParams,
) -> jsonrpc_core::Result<Option<DocumentSymbolResponse>> {
    let DocumentSymbolParams {
        text_document: TextDocumentIdentifier { uri },
    } = params;

    // Prepare the response.
    let mut response = None;

    // Attempt to obtain the document.
    if let Some(document) = session.documents.get(&uri) {
        // Vector to collect document symbols into as they are constructed.
        let mut syms: Vec<DocumentSymbol> = vec![];

        // Prepare a filter to discard uninteresting module-child nodes.
        let modulefield_filter = |node: &tree_sitter::Node| {
            [
                *wast::kind::DATA,
                *wast::kind::ELEM,
                *wast::kind::FUNC,
                *wast::kind::GLOBAL,
                *wast::kind::MEMORY,
                *wast::kind::TABLE,
                *wast::kind::TYPE,
            ]
            .contains(&node.kind_id())
        };

        // Prepare the syntax tree.
        let tree = document.tree.lock().await.clone();
        let node = tree.root_node();

        // Prepare the stack machine:
        //   data: contains data for constructing upcoming DocumentSymbols
        //   work: contains remaining tree_sitter nodes to process
        // FIXME: tune this
        let mut data = vec![];
        let mut work = vec![Work::Node(node)];

        // Convenience macro for processing document symbol nodes.
        macro_rules! push {
            ($empty_name:expr, $kind:expr) => {
                document_symbol::push(
                    &document,
                    *wast::field::ID,
                    &mut data,
                    &mut work,
                    &node,
                    $empty_name,
                    $kind,
                )
            };
        }

        // The stack machine work loop.
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

                Work::Node(node) if node.kind_id() == *wast::kind::ENTRYPOINT => {
                    let mut cursor = node.walk();
                    let commands = node
                        .children_by_field_id(*wast::field::COMMAND, &mut cursor)
                        .map(Work::Node);
                    work.extend(commands);
                },

                Work::Node(node) if node.kind_id() == *wast::kind::COMMAND => {
                    let command = node.named_child(0).expect("'command' should have a single named child");
                    work.push(Work::Node(command));
                },

                Work::Node(node) if node.kind_id() == *wast::kind::MODULE => {
                    let SymbolRange {
                        name,
                        range,
                        selection_range,
                    } = symbol_range(&document.text.as_bytes(), "<module>", &node, *wast::field::ID);
                    work.push(Work::Data);

                    let mut children_count = 0;
                    for modulefield in node
                        .children_by_field_id(*wast::field::FIELD, &mut node.walk())
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

                Work::Node(node) if node.kind_id() == *wast::kind::DATA => {
                    push!("<data>", SymbolKind::Key);
                },

                Work::Node(node) if node.kind_id() == *wast::kind::ELEM => {
                    push!("<elem>", SymbolKind::Field);
                },

                Work::Node(node) if node.kind_id() == *wast::kind::FUNC => {
                    push!("<func>", SymbolKind::Function);
                },

                Work::Node(node) if node.kind_id() == *wast::kind::GLOBAL => {
                    push!("<global>", SymbolKind::Event);
                },

                Work::Node(node) if node.kind_id() == *wast::kind::MEMORY => {
                    push!("<memory>", SymbolKind::Array);
                },

                Work::Node(node) if node.kind_id() == *wast::kind::TABLE => {
                    push!("<table>", SymbolKind::Interface);
                },

                Work::Node(node) if node.kind_id() == *wast::kind::TYPE => {
                    push!("<type>", SymbolKind::TypeParameter);
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
