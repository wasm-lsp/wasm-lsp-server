mod walker;

pub use walker::*;

#[allow(missing_docs)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct NodeError;

impl<'tree> From<tree_sitter::Node<'tree>> for NodeError {
    fn from(_: tree_sitter::Node<'tree>) -> Self {
        Self
    }
}

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

#[allow(missing_docs)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct SyntaxError {
    expected: Vec<u16>,
    found: NodeError,
}

impl SyntaxError {
    #[allow(missing_docs)]
    pub fn expected(&self) -> &[u16] {
        &self.expected
    }

    #[allow(missing_docs)]
    pub fn found(&self) -> &NodeError {
        &self.found
    }
}

impl std::fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SyntaxError")
            .field("expected", &self.expected)
            .field("found", &self.found)
            .finish()
    }
}

#[allow(missing_docs)]
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct SyntaxErrors {
    errors: Vec<SyntaxError>,
}

#[allow(missing_docs)]
impl SyntaxErrors {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn append(&mut self, other: &mut SyntaxErrors) {
        self.errors.append(&mut other.errors);
    }

    pub fn errors(&self) -> &[SyntaxError] {
        &self.errors
    }

    pub fn push(&mut self, error: SyntaxError) {
        self.errors.push(error);
    }
}

impl From<SyntaxError> for SyntaxErrors {
    fn from(error: SyntaxError) -> Self {
        let mut errors = SyntaxErrors::new();
        errors.push(error);
        errors
    }
}
