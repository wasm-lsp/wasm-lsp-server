use crate::{database::Database, synchronizer::Synchronizer};
use failure::Fallible;
use std::sync::Arc;

/// Analyzer computes queries from elaborated syntax and metadata in Database.
pub struct Analyzer {
    database: Arc<Database>,
    synchronizer: Arc<Synchronizer>,
}

impl Analyzer {
    pub fn new(database: Arc<Database>, synchronizer: Arc<Synchronizer>) -> Fallible<Self> {
        Ok(Analyzer { database, synchronizer })
    }
}
