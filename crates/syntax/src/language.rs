//! Functionality related to [`tree_sitter::Language`].

/// Functions for working with the `.wast` grammar.
pub mod wast;

/// Functions for working with the `.wat` grammar.
pub mod wat;

/// Comment nodes for both the `.wast` and `.wat` grammar.
pub const COMMENT_NODES: &[u16] = &[
    wast::kind::COMMENT_BLOCK_ANNOT,
    wast::kind::COMMENT_BLOCK,
    wast::kind::COMMENT_LINE_ANNOT,
    wast::kind::COMMENT_LINE,
    wat::kind::COMMENT_BLOCK_ANNOT,
    wat::kind::COMMENT_BLOCK,
    wat::kind::COMMENT_LINE_ANNOT,
    wat::kind::COMMENT_LINE,
];
