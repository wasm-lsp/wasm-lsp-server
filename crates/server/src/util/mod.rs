//! Various utility functionality, e.g., for handling lsp or tree-sitter data.

/// Functions for creation of lsp data from tree-sitter nodes.
pub(crate) mod node {
    /// Functions for creation of lsp position data from tree-sitter nodes.
    mod position {
        /// Creates an lsp position from the starting position of a tree-sitter node.
        pub(crate) fn start(node: &tree_sitter::Node) -> tower_lsp::lsp_types::Position {
            let tree_sitter::Point { row, column } = node.start_position();
            tower_lsp::lsp_types::Position::new(row as u64, column as u64)
        }

        /// Creates an lsp position from the ending position of a tree-sitter node.
        pub(crate) fn end(node: &tree_sitter::Node) -> tower_lsp::lsp_types::Position {
            let tree_sitter::Point { row, column } = node.end_position();
            tower_lsp::lsp_types::Position::new(row as u64, column as u64)
        }
    }

    /// Creates an lsp range from the range of a tree-sitter node.
    pub(crate) fn range(node: &tree_sitter::Node) -> tower_lsp::lsp_types::Range {
        tower_lsp::lsp_types::Range::new(position::start(node), position::end(node))
    }

    /// Convenience type for packaging a (symbol) name with an lsp range and selection range.
    #[derive(Clone, Debug)]
    pub(crate) struct SymbolRange<'a> {
        /// The name (identifier) of the symbol.
        pub(crate) name: &'a str,
        /// The (node-enclosing) range of the symbol.
        pub(crate) range: tower_lsp::lsp_types::Range,
        /// The (identifier-enclosing) range of the symbol.
        pub(crate) selection_range: tower_lsp::lsp_types::Range,
    }

    /// Compute the name and ranges for a document symbol given tree-sitter node data.
    pub(crate) fn symbol_range<'a>(
        source: &'a [u8],
        empty_name: &'a str,
        node: &tree_sitter::Node,
        field_id: u16,
    ) -> SymbolRange<'a> {
        let name;
        let range = crate::util::node::range(&node);
        let selection_range;
        if let Some(inner_node) = node.child_by_field_id(field_id) {
            name = inner_node.utf8_text(source).unwrap();
            selection_range = crate::util::node::range(&inner_node);
        } else {
            name = empty_name;
            selection_range = range;
        }

        SymbolRange {
            name,
            range,
            selection_range,
        }
    }
}
