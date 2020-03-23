use crate::{message::Message, parser::Parser};
use dashmap::DashMap;
use failure::Fallible;
use lsp_types::*;
use std::sync::Arc;
use tokio::sync::{watch, Mutex};
use tower_lsp::Client;
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
    tx: watch::Sender<Message>,
    pub rx: watch::Receiver<Message>,
    pub trees: Arc<DashMap<Url, Mutex<Tree>>>,
}

impl Synchronizer {
    pub fn new(parser: Arc<Parser>) -> Fallible<Self> {
        let trees = Arc::new(DashMap::new());
        let (tx, rx) = watch::channel(Message::Start);
        Ok(Synchronizer { parser, rx, trees, tx })
    }

    pub async fn did_open(&self, client: &Client, params: DidOpenTextDocumentParams) -> Fallible<()> {
        let mut parser = self.parser.wat.lock().await;
        let DidOpenTextDocumentParams {
            text_document: TextDocumentItem { uri, text, .. },
        } = &params;
        let old_tree = None;
        let tree = parser.parse(text, old_tree);
        log::info!("tree: {:?}", tree);
        if let Some(tree) = tree {
            let _ = self.trees.insert(uri.clone(), Mutex::new(tree));
            self.tx.broadcast(Message::DidOpenTree {
                client: client.clone(),
                uri: uri.clone(),
            })?;
        } else {
            // TODO: report
        }
        Ok(())
    }

    pub async fn did_change(&self, client: &Client, params: DidChangeTextDocumentParams) -> Fallible<()> {
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
            self.tx.broadcast(Message::DidChangeTree {
                client: client.clone(),
                uri: uri.clone(),
            })?;
        } else {
            // TODO: report
        }
        Ok(())
    }

    pub async fn did_close(&self, client: &Client, params: DidCloseTextDocumentParams) -> Fallible<()> {
        let DidCloseTextDocumentParams {
            text_document: TextDocumentIdentifier { uri },
        } = &params;
        self.trees.remove(uri);
        self.tx.broadcast(Message::DidCloseTree {
            client: client.clone(),
            uri: uri.clone(),
        })?;
        Ok(())
    }
}
