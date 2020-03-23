use crate::session::SessionHandle;
use failure::Fallible;

/// Synthesizes typing information for documents.
pub struct Synthesizer {
    session: SessionHandle,
}

impl Synthesizer {
    pub fn new(session: SessionHandle) -> Fallible<Self> {
        Ok(Synthesizer { session })
    }
}
