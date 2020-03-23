use crate::{message::Message, parser::Parser};
use dashmap::DashMap;
use failure::Fallible;
use lsp_types::*;
use std::sync::Arc;
use tokio::sync::{
    watch::{self, Receiver, Sender},
    Mutex,
};
use tower_lsp::Client;
use tree_sitter::Tree;

/// Synchronizes document edits and parse trees and notifies other server components of changes.
pub struct Synchronizer {
    parser: Arc<Parser>,
    sender: Sender<Message>,
    pub receiver: Receiver<Message>,
    pub trees: Arc<DashMap<Url, Mutex<Tree>>>,
}

impl Synchronizer {
    pub fn new(parser: Arc<Parser>) -> Fallible<Self> {
        let trees = Arc::new(DashMap::new());
        let (sender, receiver) = watch::channel(Message::Start);
        Ok(Synchronizer {
            parser,
            receiver,
            sender,
            trees,
        })
    }

    pub async fn did_open(&self, client: &Client, params: DidOpenTextDocumentParams) -> Fallible<()> {
        let mut parser = self.parser.wat.lock().await;
        let DidOpenTextDocumentParams {
            text_document: TextDocumentItem { uri, text, .. },
        } = &params;
        // NOTE: Perhaps we should persist trees even on close. We could make this configurable.
        let old_tree = None;
        let tree = parser.parse(text, old_tree);
        log::info!("tree: {:?}", tree);
        if let Some(tree) = tree {
            let _ = self.trees.insert(uri.clone(), Mutex::new(tree));
            self.sender.broadcast(Message::TreeDidOpen {
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
        // TODO: Fetch old_tree from cache and apply edits to prepare for incremental re-parsing.
        let old_tree = None;
        let tree = parser.parse(text, old_tree);
        log::info!("tree: {:?}", tree);
        if let Some(tree) = tree {
            let _ = self.trees.insert(uri.clone(), Mutex::new(tree));
            self.sender.broadcast(Message::TreeDidChange {
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
        } = params;
        self.trees.remove(&uri);
        self.sender.broadcast(Message::TreeDidClose {
            client: client.clone(),
            uri,
        })?;
        Ok(())
    }
}
