use crate::{elaborator::Elaborator, parser::Parser};
use failure::Fallible;
use std::sync::Arc;

/// Synchronizer parses documents into trees which are then passed to elaborator
/// for further processing. Document parsing is triggered by file watcher events
/// or by document modification events.
pub struct Synchronizer {
    elaborator: Arc<Elaborator>,
    parser: Arc<Parser>,
}

impl Synchronizer {
    pub fn new(elaborator: Arc<Elaborator>, parser: Arc<Parser>) -> Fallible<Self> {
        Ok(Synchronizer { elaborator, parser })
    }
}
