//! Elaborates parse trees into structured data to be cached in the database.

use lspower::lsp_types::*;

/// Encodes data for constructing upcoming DocumentSymbols.
#[derive(Clone, Debug)]
pub(crate) struct Data<'a> {
    /// The tree-sitter Node to be processed as a symbol.
    pub(crate) node: tree_sitter::Node<'a>,
    /// Number of (possibly filtered) children to be processed for the symbol.
    pub(crate) children_count: usize,
    /// The kind of document entity the symbol represents.
    pub(crate) kind: SymbolKind,
    /// The name hint for the symbol (used for anonymous entities).
    pub(crate) name_hint: &'static str,
}

/// Encodes actions for loop iterations when processing tree-sitter nodes.
#[derive(Debug)]
pub(crate) enum Work<'a> {
    /// Construct a DocumentSymbol and pop the data stack.
    Data,
    /// Add a tree-sitter node to remaining nodes to process.
    Node(tree_sitter::Node<'a>),
}

/// Convenience type for packaging a (symbol) name with an lsp range and selection range.
#[derive(Clone, Debug)]
pub(crate) struct SymbolRange {
    /// The name (identifier) of the symbol.
    pub(crate) name: String,
    /// The (node-enclosing) range of the symbol.
    pub(crate) range: lspower::lsp_types::Range,
    /// The (identifier-enclosing) range of the symbol.
    pub(crate) selection_range: lspower::lsp_types::Range,
}

/// Compute the name and ranges for a document symbol given tree-sitter node data.
pub(crate) fn symbol_range(
    source: &[u8],
    node: tree_sitter::Node,
    name_hint: &'static str,
    field_id: u16,
) -> SymbolRange {
    let name;
    let range = crate::util::node::range(&node);
    let selection_range;
    if let Some(inner_node) = node.child_by_field_id(field_id) {
        name = inner_node.utf8_text(source).unwrap().into();
        selection_range = crate::util::node::range(&inner_node);
    } else {
        name = format!("<{}@{}:{}>", name_hint, range.start.line + 1, range.start.character + 1);
        selection_range = range;
    }

    SymbolRange {
        name,
        range,
        selection_range,
    }
}

// Document symbol provider definitions for ".wast" files.
pub mod wast {
    //! Document symbol definitions for ".wast" files.

    use crate::{
        core::{document::Document, language::wast},
        provider::document_symbol::{symbol_range, Data, SymbolRange, Work},
    };
    use lspower::lsp_types::*;

    /// Compute "textDocument/documentSymbols" for a given document.
    pub async fn response(document: &Document) -> Option<DocumentSymbolResponse> {
        // Vector to collect document symbols into as they are constructed.
        let mut syms: Vec<DocumentSymbol> = vec![];

        // Prepare the syntax tree.
        let tree = document.tree.lock().await.clone();
        let node = tree.root_node();

        // Prepare the stack machine:
        //   data: contains data for constructing upcoming DocumentSymbols
        //   work: contains remaining tree_sitter nodes to process
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
                            tags: Default::default(),
                        };
                        syms.push(this);
                    }
                },

                Work::Node(node) if *wast::kind::ROOT == node.kind_id() => {
                    let mut cursor = node.walk();
                    let commands = node
                        .children(&mut cursor)
                        .filter(|it| {
                            [&[*wast::kind::COMMAND], wast::MODULE_FIELDS.as_slice()]
                                .concat()
                                .contains(&it.kind_id())
                        })
                        .map(Work::Node);
                    work.extend(commands);
                },

                Work::Node(node) if *wast::kind::COMMAND == node.kind_id() => {
                    debug_assert!(node.child_count() == 1);
                    if let Some(command) = node.named_child(0) {
                        work.push(Work::Node(command));
                    }
                },

                Work::Node(node) if *wast::kind::MODULE == node.kind_id() => {
                    work.push(Work::Data);

                    let mut children_count = 0;
                    for child in node.children(&mut node.walk()) {
                        if wast::MODULE_FIELDS.contains(&child.kind_id()) {
                            work.push(Work::Node(child));
                            children_count += 1;
                        }
                    }

                    data.push(Data {
                        node,
                        children_count,
                        kind: SymbolKind::Module,
                        name_hint: "module",
                    });
                },

                Work::Node(node) if *wast::kind::MODULE_FIELD_DATA == node.kind_id() => {
                    work.push(Work::Data);
                    data.push(Data {
                        node,
                        children_count: 0,
                        kind: SymbolKind::Key,
                        name_hint: "data",
                    });
                },

                Work::Node(node) if *wast::kind::MODULE_FIELD_ELEM == node.kind_id() => {
                    work.push(Work::Data);
                    data.push(Data {
                        node,
                        children_count: 0,
                        kind: SymbolKind::Field,
                        name_hint: "elem",
                    });
                },

                Work::Node(node) if *wast::kind::MODULE_FIELD_FUNC == node.kind_id() => {
                    work.push(Work::Data);
                    data.push(Data {
                        node,
                        children_count: 0,
                        kind: SymbolKind::Function,
                        name_hint: "func",
                    });
                },

                Work::Node(node) if *wast::kind::MODULE_FIELD_GLOBAL == node.kind_id() => {
                    work.push(Work::Data);
                    data.push(Data {
                        node,
                        children_count: 0,
                        kind: SymbolKind::Event,
                        name_hint: "global",
                    });
                },

                Work::Node(node) if *wast::kind::MODULE_FIELD_MEMORY == node.kind_id() => {
                    work.push(Work::Data);
                    data.push(Data {
                        node,
                        children_count: 0,
                        kind: SymbolKind::Array,
                        name_hint: "memory",
                    });
                },

                Work::Node(node) if *wast::kind::MODULE_FIELD_TABLE == node.kind_id() => {
                    work.push(Work::Data);
                    data.push(Data {
                        node,
                        children_count: 0,
                        kind: SymbolKind::Interface,
                        name_hint: "table",
                    });
                },

                Work::Node(node) if *wast::kind::MODULE_FIELD_TYPE == node.kind_id() => {
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
}

// Document symbol definitions for ".wat" files.
pub mod wat {
    //! Document symbol definitions for ".wat" files.

    use crate::{
        core::{document::Document, language::wat},
        provider::document_symbol::{symbol_range, Data, SymbolRange, Work},
    };
    use lspower::lsp_types::*;

    /// Compute "textDocument/documentSymbols" for a given document.
    pub async fn response(document: &Document) -> Option<DocumentSymbolResponse> {
        // Vector to collect document symbols into as they are constructed.
        let mut syms: Vec<DocumentSymbol> = vec![];

        // Prepare the syntax tree.
        let tree = document.tree.lock().await.clone();
        let node = tree.root_node();

        // Prepare the stack machine:
        //   data: contains data for constructing upcoming DocumentSymbols
        //   work: contains remaining tree_sitter nodes to process
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
                        } = symbol_range(&document.text.as_bytes(), node, name_hint, *wat::field::IDENTIFIER);

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
                            tags: Default::default(),
                        };
                        syms.push(this);
                    }
                },

                Work::Node(node) if *wat::kind::ROOT == node.kind_id() => {
                    let mut cursor = node.walk();
                    let commands = node
                        .children(&mut cursor)
                        .filter(|it| {
                            [&[*wat::kind::MODULE], wat::MODULE_FIELDS.as_slice()]
                                .concat()
                                .contains(&it.kind_id())
                        })
                        .map(Work::Node);
                    work.extend(commands);
                },

                Work::Node(node) if *wat::kind::MODULE == node.kind_id() => {
                    work.push(Work::Data);

                    let mut children_count = 0;
                    for child in node.children(&mut node.walk()) {
                        if wat::MODULE_FIELDS.contains(&child.kind_id()) {
                            work.push(Work::Node(child));
                            children_count += 1;
                        }
                    }

                    data.push(Data {
                        node,
                        children_count,
                        kind: SymbolKind::Module,
                        name_hint: "module",
                    });
                },

                Work::Node(node) if *wat::kind::MODULE_FIELD_DATA == node.kind_id() => {
                    work.push(Work::Data);
                    data.push(Data {
                        node,
                        children_count: 0,
                        kind: SymbolKind::Key,
                        name_hint: "data",
                    });
                },

                Work::Node(node) if *wat::kind::MODULE_FIELD_ELEM == node.kind_id() => {
                    work.push(Work::Data);
                    data.push(Data {
                        node,
                        children_count: 0,
                        kind: SymbolKind::Field,
                        name_hint: "elem",
                    });
                },

                Work::Node(node) if *wat::kind::MODULE_FIELD_FUNC == node.kind_id() => {
                    work.push(Work::Data);
                    data.push(Data {
                        node,
                        children_count: 0,
                        kind: SymbolKind::Function,
                        name_hint: "func",
                    });
                },

                Work::Node(node) if *wat::kind::MODULE_FIELD_GLOBAL == node.kind_id() => {
                    work.push(Work::Data);
                    data.push(Data {
                        node,
                        children_count: 0,
                        kind: SymbolKind::Event,
                        name_hint: "global",
                    });
                },

                Work::Node(node) if *wat::kind::MODULE_FIELD_MEMORY == node.kind_id() => {
                    work.push(Work::Data);
                    data.push(Data {
                        node,
                        children_count: 0,
                        kind: SymbolKind::Array,
                        name_hint: "memory",
                    });
                },

                Work::Node(node) if *wat::kind::MODULE_FIELD_TABLE == node.kind_id() => {
                    work.push(Work::Data);
                    data.push(Data {
                        node,
                        children_count: 0,
                        kind: SymbolKind::Interface,
                        name_hint: "table",
                    });
                },

                Work::Node(node) if *wat::kind::MODULE_FIELD_TYPE == node.kind_id() => {
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
}
