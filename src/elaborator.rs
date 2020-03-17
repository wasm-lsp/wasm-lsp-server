use crate::database::Database;
use failure::Fallible;
use std::sync::Arc;

/// Writes elaborated syntax and metadata to [Database].
pub struct Elaborator {
    database: Arc<Database>,
}

impl Elaborator {
    pub fn new(database: Arc<Database>) -> Fallible<Self> {
        Ok(Elaborator { database })
    }
}
