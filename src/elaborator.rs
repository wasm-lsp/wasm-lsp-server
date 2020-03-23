use crate::{database::Database, message::Message, synchronizer::Synchronizer};
use failure::Fallible;
use lsp_types::*;
use std::sync::Arc;
use tokio::sync::watch;
use tower_lsp::Client;

/// Elaborates a given [`Tree`] into structured data to be cached in
/// [`Database`](crate::database::Database).
///
/// [`Tree`]: https://docs.rs/tree-sitter/latest/tree_sitter/struct.Tree.html
pub struct Elaborator {
    database: Arc<Database>,
    rx: watch::Receiver<Message>,
    synchronizer: Arc<Synchronizer>,
}

impl Elaborator {
    pub fn new(
        database: Arc<Database>,
        rx: watch::Receiver<Message>,
        synchronizer: Arc<Synchronizer>,
    ) -> Fallible<Self> {
        Ok(Elaborator {
            database,
            rx,
            synchronizer,
        })
    }

    pub async fn init(&self) -> Fallible<()> {
        let mut rx = self.rx.clone();
        while let Some(message) = rx.recv().await {
            match message {
                Message::TreeDidChange { client, uri, .. } => self.tree_did_change(client, uri).await?,
                Message::TreeDidClose { client, uri, .. } => self.tree_did_close(client, uri).await?,
                Message::TreeDidOpen { client, uri, .. } => self.tree_did_open(client, uri).await?,
                _ => {},
            }
        }
        Ok(())
    }

    async fn tree_did_change(&self, _: Client, uri: Url) -> Fallible<()> {
        if let Some(tree) = self.synchronizer.trees.get(&uri) {
            let tree = tree.lock().await.clone();
            let node = tree.root_node();
            if !node.has_error() {
                log::info!("syntax well-formed");
            }
            // NOTE: else let auditor handle
            // TODO: allow partial elaboration in presence of syntax errors
        }
        Ok(())
    }

    async fn tree_did_close(&self, _: Client, _: Url) -> Fallible<()> {
        Ok(())
    }

    async fn tree_did_open(&self, _: Client, uri: Url) -> Fallible<()> {
        if let Some(tree) = self.synchronizer.trees.get(&uri) {
            let tree = tree.lock().await.clone();
            let node = tree.root_node();
            if !node.has_error() {
                log::info!("syntax well-formed");
            }
            // NOTE: else let auditor handle
            // TODO: allow partial elaboration in presence of syntax errors
        }
        Ok(())
    }
}
