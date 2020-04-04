pub(crate) mod node {
    mod position {
        pub(crate) fn start(node: &tree_sitter::Node) -> lsp_types::Position {
            let tree_sitter::Point { row, column } = node.start_position();
            lsp_types::Position::new(row as u64, column as u64)
        }

        pub(crate) fn end(node: &tree_sitter::Node) -> lsp_types::Position {
            let tree_sitter::Point { row, column } = node.end_position();
            lsp_types::Position::new(row as u64, column as u64)
        }
    }

    fn range(node: &tree_sitter::Node) -> lsp_types::Range {
        lsp_types::Range::new(position::start(node), position::end(node))
    }

    #[derive(Clone, Debug)]
    pub(crate) struct NameAndRanges<'a> {
        pub(crate) name: &'a str,
        pub(crate) range: lsp_types::Range,
        pub(crate) selection_range: lsp_types::Range,
    }

    pub(crate) fn name_and_ranges<'a>(
        source: &'a [u8],
        empty_name: &'a str,
        node: &tree_sitter::Node,
        field_id: u16,
    ) -> NameAndRanges<'a> {
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

        NameAndRanges {
            name,
            range,
            selection_range,
        }
    }
}
