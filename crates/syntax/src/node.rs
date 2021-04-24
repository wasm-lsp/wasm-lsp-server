mod walker;

pub use walker::*;

#[allow(missing_docs)]
#[derive(Clone, PartialEq)]
pub struct NodeErrorData {
    language: tree_sitter::Language,
    kind_id: u16,
    range: tree_sitter::Range,
    error_state: Vec<u16>,
}

impl std::fmt::Debug for NodeErrorData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let kind_id = self
            .language
            .node_kind_for_id(self.kind_id)
            .unwrap_or_else(|| "<unknown>".into());
        let error_state = self
            .error_state
            .iter()
            .map(|&id| self.language.node_kind_for_id(id).unwrap_or_else(|| "<unknown>".into()))
            .collect::<Vec<_>>();
        f.debug_struct("NodeErrorData")
            .field("kind_id", &kind_id)
            .field("range", &self.range)
            .field("error_state", &error_state)
            .finish()
    }
}

impl NodeErrorData {
    #[allow(missing_docs)]
    pub fn new(node: tree_sitter::Node, error_state: Vec<u16>) -> Self {
        let language = node.language();
        let kind_id = node.kind_id();
        let range = node.range();
        NodeErrorData {
            language,
            kind_id,
            range,
            error_state,
        }
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
#[derive(Clone, PartialEq)]
pub struct NodeError {
    language: tree_sitter::Language,
    expected: Vec<u16>,
    found: Option<NodeErrorData>,
}

impl NodeError {
    #[allow(missing_docs)]
    pub fn new(language: tree_sitter::Language, expected: Vec<u16>, found: Option<NodeErrorData>) -> Self {
        Self {
            language,
            expected,
            found,
        }
    }

    #[allow(missing_docs)]
    pub fn expected(&self) -> &[u16] {
        &self.expected
    }

    #[allow(missing_docs)]
    pub fn found(&self) -> &Option<NodeErrorData> {
        &self.found
    }
}

impl From<NodeError> for SyntaxError {
    fn from(value: NodeError) -> Self {
        SyntaxError::NodeError(value)
    }
}

impl std::fmt::Debug for NodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let expected = self
            .expected
            .iter()
            .map(|&id| self.language.node_kind_for_id(id).unwrap_or_else(|| "<unknown>".into()))
            .collect::<Vec<_>>();
        f.debug_struct("SyntaxError")
            .field("expected", &expected)
            .field("found", &self.found)
            .finish()
    }
}

impl std::fmt::Display for NodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let expected = self
            .expected
            .iter()
            .map(|&id| self.language.node_kind_for_id(id).unwrap_or_else(|| "<unknown>".into()))
            .collect::<Vec<_>>();
        f.debug_struct("SyntaxError")
            .field("expected", &expected)
            .field("found", &self.found)
            .finish()
    }
}

#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub enum SyntaxError {
    DoneEarly,
    MissingNode(NodeErrorData),
    MoreNodes,
    NodeError(NodeError),
}

#[allow(missing_docs)]
#[derive(Clone, Debug, Default, PartialEq)]
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
