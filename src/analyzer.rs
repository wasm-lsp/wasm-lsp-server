use failure::Fallible;
use std::sync::{Arc, Mutex};
use tree_sitter::Parser;

pub struct Analyzer {
    parser: Arc<Mutex<Parser>>,
}

impl Analyzer {
    pub fn new(parser: Arc<Mutex<Parser>>) -> Fallible<Analyzer> {
        Ok(Analyzer { parser })
    }
}
