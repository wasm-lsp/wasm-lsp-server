use crate::synchronizer::Synchronizer;
use failure::Fallible;
use std::sync::Arc;

/// Computes queries from elaborated syntax and metadata in [`Database`](crate::database::Database).
pub struct Analyzer {
    synchronizer: Arc<Synchronizer>,
}

impl Analyzer {
    pub fn new(synchronizer: Arc<Synchronizer>) -> Fallible<Self> {
        Ok(Analyzer { synchronizer })
    }
}
