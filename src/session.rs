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

/// Session represents the current state of the LSP service.
pub struct Session {
    analyzer: Arc<Analyzer>,
    database: Arc<Database>,
    elaborator: Arc<Elaborator>,
    highlighter: Arc<Highlighter>,
    parser: Arc<Parser>,
    synchronizer: Arc<Synchronizer>,
}

impl Session {
    pub fn new() -> Fallible<Self> {
        let database = Arc::new(Database::new()?);
        let parser = Arc::new(Parser::new()?);
        let elaborator = Arc::new(Elaborator::new(database.clone())?);
        let synchronizer = Arc::new(Synchronizer::new(elaborator.clone(), parser.clone())?);
        let analyzer = Arc::new(Analyzer::new(database.clone(), synchronizer.clone())?);
        let highlighter = Arc::new(Highlighter::new(database.clone(), synchronizer.clone())?);
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
