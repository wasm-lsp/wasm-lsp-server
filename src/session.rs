use crate::{
    analyzer::Analyzer,
    database::Database,
    elaborator::Elaborator,
    highlighter::Highlighter,
    parser::Parser,
    synchronizer::Synchronizer,
};
use failure::Fallible;
use std::sync::Arc;

/// Represents the current state of the LSP service.
pub struct Session {
    analyzer: Arc<Analyzer>,
    database: Arc<Database>,
    elaborator: Arc<Elaborator>,
    highlighter: Arc<Highlighter>,
    parser: Arc<Parser>,
    pub synchronizer: Arc<Synchronizer>,
}

impl Session {
    pub fn new() -> Fallible<Self> {
        let database = Arc::new(Database::new()?);
        let parser = Arc::new(Parser::new()?);
        let elaborator = Arc::new(Elaborator::new()?);
        let synchronizer = Arc::new(Synchronizer::new(database.clone(), elaborator.clone(), parser.clone())?);
        let analyzer = Arc::new(Analyzer::new(synchronizer.clone())?);
        let highlighter = Arc::new(Highlighter::new(synchronizer.clone())?);
        Ok(Session {
            analyzer,
            database,
            elaborator,
            highlighter,
            parser,
            synchronizer,
        })
    }
}
