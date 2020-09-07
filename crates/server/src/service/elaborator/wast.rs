//! Elaborator definitions specific to ".wast" files.

use crate::{
    core::{document::Document, language::wast},
    service::elaborator::document_symbol::{symbol_range, Data, SymbolRange, Work},
};
use tower_lsp::lsp_types::*;

/// Compute the symbols for a given document.
pub(crate) async fn document_symbol(document: &Document) -> Option<DocumentSymbolResponse> {
    // Vector to collect document symbols into as they are constructed.
    let mut syms: Vec<DocumentSymbol> = vec![];

    // Prepare a filter to discard uninteresting module-child nodes.
    let module_field_filter = |node: &tree_sitter::Node| {
        [
            *wast::kind::MODULE_FIELD_DATA,
            *wast::kind::MODULE_FIELD_ELEM,
            *wast::kind::MODULE_FIELD_FUNC,
            *wast::kind::MODULE_FIELD_GLOBAL,
            *wast::kind::MODULE_FIELD_MEMORY,
            *wast::kind::MODULE_FIELD_TABLE,
            *wast::kind::MODULE_FIELD_TYPE,
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

    // The stack machine work loop.
    while let Some(next) = work.pop() {
        match next {
            // Construct a DocumentSymbol and pop data stack
            Work::Data => {
                if let Some(Data {
                    node,
                    children_count,
                    kind,
                    name_hint,
                }) = data.pop()
                {
                    let SymbolRange {
                        name,
                        range,
                        selection_range,
                    } = symbol_range(&document.text.as_bytes(), node, name_hint, *wast::field::IDENTIFIER);

                    // FIXME
                    #[allow(deprecated)]
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

            Work::Node(node) if node.kind_id() == *wast::kind::PARSE => {
                let mut cursor = node.walk();
                let commands = node
                    .children(&mut cursor)
                    .filter(|it| [*wast::kind::COMMAND, *wast::kind::MODULE_FIELD].contains(&it.kind_id()))
                    .map(Work::Node);
                work.extend(commands);
            },

            Work::Node(node) if node.kind_id() == *wast::kind::COMMAND => {
                debug_assert!(node.child_count() == 1);
                if let Some(command) = node.named_child(0) {
                    work.push(Work::Node(command));
                }
            },

            Work::Node(node) if node.kind_id() == *wast::kind::MODULE => {
                work.push(Work::Data);

                let mut children_count = 0;
                for child in node.children(&mut node.walk()) {
                    if child.kind_id() == *wast::kind::MODULE_FIELD {
                        debug_assert!(child.child_count() == 1);
                        if let Some(module_field) = child.named_child(0) {
                            if module_field_filter(&module_field) {
                                work.push(Work::Node(module_field));
                                children_count += 1;
                            }
                        }
                    }
                }

                data.push(Data {
                    node,
                    children_count,
                    kind: SymbolKind::Module,
                    name_hint: "module",
                });
            },

            Work::Node(node) if node.kind_id() == *wast::kind::MODULE_FIELD => {
                debug_assert!(node.child_count() == 1);
                if let Some(module_field) = node.named_child(0) {
                    if module_field_filter(&module_field) {
                        work.push(Work::Node(module_field));
                    }
                }
            },

            Work::Node(node) if node.kind_id() == *wast::kind::MODULE_FIELD_DATA => {
                work.push(Work::Data);
                data.push(Data {
                    node,
                    children_count: 0,
                    kind: SymbolKind::Key,
                    name_hint: "data",
                });
            },

            Work::Node(node) if node.kind_id() == *wast::kind::MODULE_FIELD_ELEM => {
                work.push(Work::Data);
                data.push(Data {
                    node,
                    children_count: 0,
                    kind: SymbolKind::Field,
                    name_hint: "elem",
                });
            },

            Work::Node(node) if node.kind_id() == *wast::kind::MODULE_FIELD_FUNC => {
                work.push(Work::Data);
                data.push(Data {
                    node,
                    children_count: 0,
                    kind: SymbolKind::Function,
                    name_hint: "func",
                });
            },

            Work::Node(node) if node.kind_id() == *wast::kind::MODULE_FIELD_GLOBAL => {
                work.push(Work::Data);
                data.push(Data {
                    node,
                    children_count: 0,
                    kind: SymbolKind::Event,
                    name_hint: "global",
                });
            },

            Work::Node(node) if node.kind_id() == *wast::kind::MODULE_FIELD_MEMORY => {
                work.push(Work::Data);
                data.push(Data {
                    node,
                    children_count: 0,
                    kind: SymbolKind::Array,
                    name_hint: "memory",
                });
            },

            Work::Node(node) if node.kind_id() == *wast::kind::MODULE_FIELD_TABLE => {
                work.push(Work::Data);
                data.push(Data {
                    node,
                    children_count: 0,
                    kind: SymbolKind::Interface,
                    name_hint: "table",
                });
            },

            Work::Node(node) if node.kind_id() == *wast::kind::MODULE_FIELD_TYPE => {
                work.push(Work::Data);
                data.push(Data {
                    node,
                    children_count: 0,
                    kind: SymbolKind::TypeParameter,
                    name_hint: "type",
                });
            },

            _ => {},
        }
    }
    // Collect the syms vec into a new vec in reverse so that document symbols are returned in the
    // correct order. Note that children nodes are reversed _as the symbols are nested_.
    let results = syms.into_iter().rev().collect();

    Some(DocumentSymbolResponse::Nested(results))
}
