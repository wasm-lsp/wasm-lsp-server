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

    pub async fn did_open(&self, client: &Client, params: DidOpenTextDocumentParams) {
        let _ = client;
        let mut parser = self.parser.wat.lock().await;
        let DidOpenTextDocumentParams {
            text_document: TextDocumentItem { uri, text, .. },
        } = &params;
        let old_tree = None;
        let tree = parser.parse(text, old_tree);
        log::info!("tree: {:?}", tree);
        if let Some(tree) = tree {
            let _ = self.trees.insert(uri.clone(), Mutex::new(tree));
        } else {
            // TODO: report
        }
    }

    pub async fn did_change(&self, client: &Client, params: DidChangeTextDocumentParams) {
        let _ = client;
        let mut parser = self.parser.wat.lock().await;
        let DidChangeTextDocumentParams {
            text_document: VersionedTextDocumentIdentifier { uri, .. },
            content_changes,
        } = &params;
        let TextDocumentContentChangeEvent { ref text, .. } = content_changes[0];
        let old_tree = None;
        let tree = parser.parse(text, old_tree);
        log::info!("tree: {:?}", tree);
        if let Some(tree) = tree {
            let _ = self.trees.insert(uri.clone(), Mutex::new(tree));
        } else {
            // TODO: report
        }
    }

    pub async fn did_close(&self, client: &Client, params: DidCloseTextDocumentParams) {
        let _ = client;
        let DidCloseTextDocumentParams {
            text_document: TextDocumentIdentifier { uri },
        } = &params;
        self.trees.remove(uri);
    }
}
