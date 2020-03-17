use crate::{database::Database, synchronizer::Synchronizer};
use failure::Fallible;
use std::sync::Arc;

/// Analyzer computes highlights from elaborated syntax and metadata in Database.
pub struct Highlighter {
    database: Arc<Database>,
    synchronizer: Arc<Synchronizer>,
}

impl Highlighter {
    pub fn new(database: Arc<Database>, synchronizer: Arc<Synchronizer>) -> Fallible<Self> {
        Ok(Highlighter { database, synchronizer })
    }
}
