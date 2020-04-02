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

    pub fn range(node: &Node) -> Range {
        Range::new(position::start(node), position::end(node))
    }

    #[derive(Clone, Debug)]
    pub struct NameAndRanges<'a> {
        pub name: &'a str,
        pub range: Range,
        pub selection_range: Range,
    }

    pub fn name_and_ranges<'a>(source: &'a [u8], empty_name: &'a str, node: &Node, field_id: u16) -> NameAndRanges<'a> {
        let name;
        let range = crate::lsp::node::range(&node);
        let selection_range;
        if let Some(inner_node) = node.child_by_field_id(field_id) {
            name = inner_node.utf8_text(source).unwrap();
            selection_range = crate::lsp::node::range(&inner_node);
        } else {
            name = empty_name;
            selection_range = range;
        }

        NameAndRanges {
            name,
            range,
            selection_range,
        }
    }
}
