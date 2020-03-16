use crate::{database::Database, parser::Parser};
use failure::Fallible;
use std::sync::Arc;

pub struct Synchronizer {
    database: Arc<Database>,
    parser: Arc<Parser>,
}

impl Synchronizer {
    pub fn new(database: Arc<Database>, parser: Arc<Parser>) -> Fallible<Self> {
        Ok(Synchronizer { database, parser })
    }
}
