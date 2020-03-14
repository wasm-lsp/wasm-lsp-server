use crate::{analyzer::Analyzer, database::Database, parser};
use failure::Fallible;
use std::sync::{Arc, Mutex};
use tree_sitter::Parser;

pub struct Session {
    analyzer: Arc<Analyzer>,
    database: Arc<Database>,
    parser: Arc<Mutex<Parser>>,
}

impl Session {
    pub fn new() -> Fallible<Session> {
        let database = Arc::new(Database::new()?);
        let parser = Arc::new(Mutex::new(parser::Factory::parser()?));
        let analyzer = Arc::new(Analyzer::new(database.clone(), parser.clone())?);
        Ok(Session {
            analyzer,
            database,
            parser,
        })
    }
}
