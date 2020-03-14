use crate::{analyzer::Analyzer, parser};
use failure::Fallible;
use std::sync::{Arc, Mutex};
use tree_sitter::Parser;

pub struct Session {
    analyzer: Arc<Analyzer>,
    parser: Arc<Mutex<Parser>>,
}

impl Session {
    pub fn new() -> Fallible<Session> {
        let parser = Arc::new(Mutex::new(parser::Factory::parser()?));
        let analyzer = Arc::new(Analyzer::new(parser.clone())?);
        Ok(Session { analyzer, parser })
    }
}
