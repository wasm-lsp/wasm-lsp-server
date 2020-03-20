use crate::{database::Database, elaborator::Elaborator, parser::Parser};
use dashmap::DashMap;
use failure::Fallible;
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_lsp::{lsp_types::*, Client};
use tree_sitter::Tree;

/// Parses a given document into a [`Tree`] with [`Parser`]. Then [`Elaborator`]
/// processes the resulting tree into structured data which is finally cached in
/// [`Database`]. The parsed tree is also stored in hashmap to allow for
/// incremental edits and re-parsing. Document parsing is triggered by file
/// watcher events or by document modification events.
///
/// [`Parser`]: https://docs.rs/tree-sitter/latest/tree_sitter/struct.Parser.html
/// [`Tree`]: https://docs.rs/tree-sitter/latest/tree_sitter/struct.Tree.html
pub struct Synchronizer {
    database: Arc<Database>,
    elaborator: Arc<Elaborator>,
    pub parser: Arc<Parser>,
    trees: Arc<DashMap<Url, Mutex<Tree>>>,
}

impl Synchronizer {
    pub fn new(database: Arc<Database>, elaborator: Arc<Elaborator>, parser: Arc<Parser>) -> Fallible<Self> {
        let trees = Arc::new(DashMap::new());
        Ok(Synchronizer {
            database,
            elaborator,
            parser,
            trees,
        })
    }
}
