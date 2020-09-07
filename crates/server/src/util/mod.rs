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
}
