use crate::{message::Message, synchronizer::Synchronizer};
use failure::Fallible;
use lsp_types::*;
use std::sync::Arc;
use tokio::sync::watch;
use tower_lsp::Client;

pub struct Auditor {
    rx: watch::Receiver<Message>,
    synchronizer: Arc<Synchronizer>,
}

impl Auditor {
    pub fn new(rx: watch::Receiver<Message>, synchronizer: Arc<Synchronizer>) -> Fallible<Self> {
        Ok(Auditor { rx, synchronizer })
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
            if node.has_error() {
                log::info!("syntax error");
            }
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
            if node.has_error() {
                log::info!("syntax error");
            }
        }
        Ok(())
    }
}
