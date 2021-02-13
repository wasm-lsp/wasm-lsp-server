pub mod error;
pub mod language;
pub mod parser;

pub trait NodeExt {
    fn matches_subtypes(&self, supertype_id: u16, subtype_ids: &[u16]) -> bool;
}

impl<'tree> NodeExt for tree_sitter::Node<'tree> {
    fn matches_subtypes(&self, supertype_id: u16, subtype_ids: &[u16]) -> bool {
        supertype_id == self.kind_id() && subtype_ids.contains(&self.kind_id())
    }
}
