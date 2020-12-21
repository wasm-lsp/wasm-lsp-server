//! Elaborates parse trees into structured data to be cached in the database.

use lspower::lsp_types::*;

// Elaborator definitions specific to ".wast" files.
pub mod wast;

// Elaborator definitions specific to ".wat" files.
pub mod wat;

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
