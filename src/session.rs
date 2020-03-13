use crate::parser;
use failure::Fallible;
use std::sync::{Arc, Mutex};
use tree_sitter::Parser;

pub struct Session {
    pub parser: Arc<Mutex<Parser>>,
}

impl Session {
    pub fn new() -> Fallible<Session> {
        let parser = Arc::new(Mutex::new(parser::Factory::parser()?));
        Ok(Session { parser })
    }
}
