use crate::{
    core::{self, language::wat, node::NodeExt},
    provider::document_symbol::{symbol_range, Data, SymbolRange, Work},
};
use std::sync::Arc;

// Document symbol provider definitions for ".wat" files.
pub async fn document_symbol(
    session: Arc<core::Session>,
    params: lsp::DocumentSymbolParams,
    content: &ropey::Rope,
) -> anyhow::Result<Option<lsp::DocumentSymbolResponse>> {
    // Vector to collect document symbols into as they are constructed.
    let mut syms: Vec<lsp::DocumentSymbol> = vec![];

    // Prepare the syntax tree.
    let tree = session.get_tree(&params.text_document.uri).await?;
    let tree = tree.lock().await;
    let node = tree.root_node();

    // Prepare the stack machine:
    //   data: contains data for constructing upcoming DocumentSymbols
    //   work: contains remaining tree_sitter nodes to process
    let mut data: Vec<Data> = vec![];
    let mut work: Vec<Work> = vec![Work::Node(node)];

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
                    } = { symbol_range(content, node, name_hint, *wat::field::IDENTIFIER) };

                    #[allow(deprecated)]
                    let sym = lsp::DocumentSymbol {
                        children: if syms.is_empty() {
                            None
                        } else {
                            // Drain the syms array by the number of children nodes we counted for this DocumentSymbol.
                            // This allows us to properly reconstruct symbol nesting.
                            let children = syms.drain(syms.len() - children_count ..);
                            let children = children.rev();
                            Some(children.collect())
                        },
                        deprecated: Default::default(),
                        detail: Default::default(),
                        kind,
                        name: name.to_string(),
                        range,
                        selection_range,
                        tags: Default::default(),
                    };
                    syms.push(sym);
                }
            },

            Work::Node(node) if *wat::kind::ROOT == node.kind_id() => {
                let mut cursor = node.walk();
                let commands = node
                    .children(&mut cursor)
                    .filter(|it| [*wat::kind::MODULE, *wat::kind::MODULE_FIELD].contains(&it.kind_id()))
                    .map(Work::Node);
                work.extend(commands);
            },

            Work::Node(node) if *wat::kind::MODULE == node.kind_id() => {
                work.push(Work::Data);

                let mut children_count = 0;
                for child in node.children(&mut node.walk()) {
                    if child.matches_subtypes(*wat::kind::MODULE_FIELD, &*wat::grouped::MODULE_FIELDS) {
                        work.push(Work::Node(child));
                        children_count += 1;
                    }
                }

                data.push(Data {
                    node,
                    children_count,
                    kind: lsp::SymbolKind::Module,
                    name_hint: "module",
                });
            },

            Work::Node(node) if *wat::kind::MODULE_FIELD == node.kind_id() => {
                let mut cursor = node.walk();
                let children = node
                    .children(&mut cursor)
                    .filter(|it| wat::grouped::MODULE_FIELDS.contains(&it.kind_id()))
                    .map(Work::Node);
                work.extend(children);
            },

            Work::Node(node) if *wat::kind::MODULE_FIELD_DATA == node.kind_id() => {
                work.push(Work::Data);
                data.push(Data {
                    node,
                    children_count: 0,
                    kind: lsp::SymbolKind::Key,
                    name_hint: "data",
                });
            },

            Work::Node(node) if *wat::kind::MODULE_FIELD_ELEM == node.kind_id() => {
                work.push(Work::Data);
                data.push(Data {
                    node,
                    children_count: 0,
                    kind: lsp::SymbolKind::Field,
                    name_hint: "elem",
                });
            },

            Work::Node(node) if *wat::kind::MODULE_FIELD_FUNC == node.kind_id() => {
                work.push(Work::Data);
                data.push(Data {
                    node,
                    children_count: 0,
                    kind: lsp::SymbolKind::Function,
                    name_hint: "func",
                });
            },

            Work::Node(node) if *wat::kind::MODULE_FIELD_GLOBAL == node.kind_id() => {
                work.push(Work::Data);
                data.push(Data {
                    node,
                    children_count: 0,
                    kind: lsp::SymbolKind::Event,
                    name_hint: "global",
                });
            },

            Work::Node(node) if *wat::kind::MODULE_FIELD_MEMORY == node.kind_id() => {
                work.push(Work::Data);
                data.push(Data {
                    node,
                    children_count: 0,
                    kind: lsp::SymbolKind::Array,
                    name_hint: "memory",
                });
            },

            Work::Node(node) if *wat::kind::MODULE_FIELD_TABLE == node.kind_id() => {
                work.push(Work::Data);
                data.push(Data {
                    node,
                    children_count: 0,
                    kind: lsp::SymbolKind::Interface,
                    name_hint: "table",
                });
            },

            Work::Node(node) if *wat::kind::MODULE_FIELD_TYPE == node.kind_id() => {
                work.push(Work::Data);
                data.push(Data {
                    node,
                    children_count: 0,
                    kind: lsp::SymbolKind::TypeParameter,
                    name_hint: "type",
                });
            },

            _ => {},
        }
    }
    // Reverse the syms vec so that document symbols are returned in the correct order. Note that
    // children nodes are reversed _as the symbols are nested_.
    syms.reverse();

    Ok(Some(lsp::DocumentSymbolResponse::Nested(syms)))
}
