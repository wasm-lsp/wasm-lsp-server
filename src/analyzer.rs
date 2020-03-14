use crate::database::Database;
use failure::Fallible;
use std::sync::{Arc, Mutex};
use tree_sitter::Parser;

pub struct Analyzer {
    database: Arc<Database>,
    parser: Arc<Mutex<Parser>>,
}

impl Analyzer {
    pub fn new(database: Arc<Database>, parser: Arc<Mutex<Parser>>) -> Fallible<Analyzer> {
        Ok(Analyzer { database, parser })
    }
}
