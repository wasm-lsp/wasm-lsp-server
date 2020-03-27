pub mod node {
    use lsp_types::*;
    use tree_sitter::Node;

    pub mod position {
        use lsp_types::*;
        use tree_sitter::{Node, Point};

        pub fn start(node: &Node) -> Position {
            let Point { row, column } = node.start_position();
            Position::new(row as u64, column as u64)
        }

        pub fn end(node: &Node) -> Position {
            let Point { row, column } = node.end_position();
            Position::new(row as u64, column as u64)
        }
    }

    pub fn location(uri: Url, node: &Node) -> Location {
        let range = range(node);
        Location { uri, range }
    }

    pub fn range(node: &Node) -> Range {
        Range::new(position::start(node), position::end(node))
    }
}
