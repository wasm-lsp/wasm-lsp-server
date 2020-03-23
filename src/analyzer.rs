use crate::session::SessionHandle;
use failure::Fallible;

/// Computes queries about documents.
pub struct Analyzer {
    session: SessionHandle,
}

impl Analyzer {
    pub fn new(session: SessionHandle) -> Fallible<Self> {
        Ok(Analyzer { session })
    }
}
