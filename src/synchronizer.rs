use crate::parser::Parser;
use dashmap::DashMap;
use failure::Fallible;
use std::sync::Arc;
use tokio::sync::{watch, Mutex};
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
    parser: Arc<Parser>,
    trees: Arc<DashMap<Url, Mutex<Tree>>>,
    tx: watch::Sender<()>,
    pub rx: watch::Receiver<()>,
}

impl Synchronizer {
    pub fn new(parser: Arc<Parser>) -> Fallible<Self> {
        let trees = Arc::new(DashMap::new());
        let (tx, rx) = watch::channel(());
        Ok(Synchronizer { parser, rx, trees, tx })
    }

    pub async fn did_open(&self, _: &Client, params: DidOpenTextDocumentParams) {
        let mut parser = self.parser.wat.lock().await;
        let DidOpenTextDocumentParams {
            text_document: TextDocumentItem { uri, text, .. },
        } = &params;
        let old_tree = None;
        let tree = parser.parse(text, old_tree);
        log::info!("tree: {:?}", tree);
        if let Some(tree) = tree {
            let _ = self.trees.insert(uri.clone(), Mutex::new(tree));
            self.tx.broadcast(()).unwrap();
        } else {
            // TODO: report
        }
    }

    pub async fn did_change(&self, _: &Client, params: DidChangeTextDocumentParams) {
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
            self.tx.broadcast(()).unwrap();
        } else {
            // TODO: report
        }
    }

    pub async fn did_close(&self, _: &Client, params: DidCloseTextDocumentParams) {
        let DidCloseTextDocumentParams {
            text_document: TextDocumentIdentifier { uri },
        } = &params;
        self.trees.remove(uri);
    }
}
