mod walker;

pub use walker::*;

/// Utility trait for working with [`tree_sitter::Node`].
pub trait NodeExt {
    /// Predicate to determine if a supertype node matches a given subtype node kind.
    fn matches_subtypes(&self, supertype_id: u16, subtype_ids: &[u16]) -> bool;
}

impl<'tree> NodeExt for tree_sitter::Node<'tree> {
    fn matches_subtypes(&self, supertype_id: u16, subtype_ids: &[u16]) -> bool {
        if let Some(child) = self.named_child(0) {
            supertype_id == self.kind_id() && subtype_ids.contains(&child.kind_id())
        } else {
            false
        }
    }
}
