use crate::synchronizer::Synchronizer;
use failure::Fallible;
use std::sync::Arc;

/// Represents the current state of the LSP service.
pub struct Session {
    pub synchronizer: Arc<Synchronizer>,
}

impl Session {
    pub fn new(synchronizer: Arc<Synchronizer>) -> Fallible<Self> {
        Ok(Session { synchronizer })
    }
}
