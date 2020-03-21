use crate::{message::Message, synchronizer::Synchronizer};
use failure::Fallible;
use std::sync::Arc;
use tokio::sync::watch;

pub struct Auditor {
    rx: watch::Receiver<Message>,
    synchronizer: Arc<Synchronizer>,
}

impl Auditor {
    pub fn new(rx: watch::Receiver<Message>, synchronizer: Arc<Synchronizer>) -> Fallible<Self> {
        Ok(Auditor { rx, synchronizer })
    }

    pub async fn init(&self) {
        let mut rx = self.rx.clone();
        while let Some(message) = rx.recv().await {
            match &message {
                Message::DidChangeTree { uri } => {
                    if let Some(tree) = self.synchronizer.trees.get(uri) {
                        let tree = tree.lock().await.clone();
                        let node = tree.root_node();
                        if node.has_error() {
                            log::info!("syntax error");
                        }
                    }
                },
                Message::DidCloseTree { .. } => {},
                Message::DidOpenTree { uri } => {
                    if let Some(tree) = self.synchronizer.trees.get(uri) {
                        let tree = tree.lock().await.clone();
                        let node = tree.root_node();
                        if node.has_error() {
                            log::info!("syntax error");
                        }
                    }
                },
                Message::Start => {},
            }
        }
    }
}
