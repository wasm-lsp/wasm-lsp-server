/// Utility trait for working with [`tree_sitter::Range`].
pub trait RangeExt {
    /// Predicate to determine if one range contains another range
    fn contains(&self, other: &tree_sitter::Range) -> bool;
}

impl RangeExt for tree_sitter::Range {
    fn contains(&self, other: &tree_sitter::Range) -> bool {
        self.start_byte() <= other.start_byte() && other.end_byte() <= self.end_byte()
    }
}
