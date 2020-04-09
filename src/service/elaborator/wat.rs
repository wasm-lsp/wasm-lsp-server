//! Elaborator definitions specific to ".wat" files.

use crate::{
    core::{document::Document, language::wat},
    service::elaborator::document_symbol::{self, Data, Work},
    util::node::{symbol_range, SymbolRange},
};
use lsp_types::*;

/// Compute the symbols for a given document.
pub(crate) async fn document_symbol(document: &Document) -> Option<DocumentSymbolResponse> {
    // Vector to collect document symbols into as they are constructed.
    let mut syms: Vec<DocumentSymbol> = vec![];

    // Prepare a filter to discard uninteresting module-child nodes.
    let modulefield_filter = |node: &tree_sitter::Node| {
        [
            *wat::kind::DATA,
            *wat::kind::ELEM,
            *wat::kind::FUNC,
            *wat::kind::GLOBAL,
            *wat::kind::MEMORY,
            *wat::kind::TABLE,
            *wat::kind::TYPE,
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
        ($node:expr, $empty_name:expr, $kind:expr) => {
            document_symbol::push(
                &document,
                *wat::field::ID,
                &mut data,
                &mut work,
                $node,
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

            Work::Node(node) if node.kind_id() == *wat::kind::ENTRYPOINT => {
                let module = node
                    .named_child(0)
                    .expect("'ENTRYPOINT' should have a single named child");
                work.push(Work::Node(module));
            },

            Work::Node(node) if node.kind_id() == *wat::kind::MODULE => {
                let SymbolRange {
                    name,
                    range,
                    selection_range,
                } = symbol_range(&document.text.as_bytes(), "<module>", &node, *wat::field::ID);
                work.push(Work::Data);

                let mut children_count = 0;
                for modulefield in node
                    .children_by_field_id(*wat::field::FIELD, &mut node.walk())
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

            Work::Node(node) if node.kind_id() == *wat::kind::MODULE_INLINE => {
                for modulefield in node
                    .children_by_field_id(*wat::field::FIELD, &mut node.walk())
                    .filter(modulefield_filter)
                {
                    work.push(Work::Node(modulefield));
                }
            },

            Work::Node(node) if node.kind_id() == *wat::kind::DATA => {
                push!(&node, "<data>", SymbolKind::Key);
            },

            Work::Node(node) if node.kind_id() == *wat::kind::ELEM => {
                push!(&node, "<elem>", SymbolKind::Field);
            },

            Work::Node(node) if node.kind_id() == *wat::kind::FUNC => {
                push!(&node, "<func>", SymbolKind::Function);
            },

            Work::Node(node) if node.kind_id() == *wat::kind::GLOBAL => {
                push!(&node, "<global>", SymbolKind::Event);
            },

            Work::Node(node) if node.kind_id() == *wat::kind::MEMORY => {
                push!(&node, "<memory>", SymbolKind::Array);
            },

            Work::Node(node) if node.kind_id() == *wat::kind::TABLE => {
                push!(&node, "<table>", SymbolKind::Interface);
            },

            Work::Node(node) if node.kind_id() == *wat::kind::TYPE => {
                push!(&node, "<type>", SymbolKind::TypeParameter);
            },

            _ => {},
        }
    }
    // Collect the syms vec into a new vec in reverse so that document symbols are returned in the
    // correct order. Note that children nodes are reversed _as the symbols are nested_.
    let results = syms.into_iter().rev().collect();

    Some(DocumentSymbolResponse::Nested(results))
}
