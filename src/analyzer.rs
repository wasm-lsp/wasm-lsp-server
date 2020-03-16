use crate::{database::Database, parser::Parser};
use failure::Fallible;
use std::sync::Arc;

pub struct Analyzer {
    database: Arc<Database>,
    parser: Arc<Parser>,
}

impl Analyzer {
    pub fn new(database: Arc<Database>, parser: Arc<Parser>) -> Fallible<Self> {
        Ok(Analyzer { database, parser })
    }
}
