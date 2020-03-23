use crate::session::SessionHandle;
use failure::Fallible;

// TODO: implement highlight adapter from tree-sitter to LSP SemanticTokenType

/// Computes highlights for documents.
pub struct Highlighter {
    session: SessionHandle,
}

impl Highlighter {
    pub fn new(session: SessionHandle) -> Fallible<Self> {
        Ok(Highlighter { session })
    }
}
