use crate::{analyzer::Analyzer, database::Database, parser::Parser};
use failure::Fallible;
use std::sync::Arc;

pub struct Session {
    analyzer: Arc<Analyzer>,
    database: Arc<Database>,
    parser: Arc<Parser>,
}

impl Session {
    pub fn new() -> Fallible<Session> {
        let database = Arc::new(Database::new()?);
        let parser = Arc::new(Parser::new()?);
        let analyzer = Arc::new(Analyzer::new(database.clone(), parser.clone())?);
        Ok(Session {
            analyzer,
            database,
            parser,
        })
    }
}
