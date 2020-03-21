use crate::{message::Message, synchronizer::Synchronizer};
use failure::Fallible;
use log;
use std::sync::Arc;
use tokio::sync::watch;

// TODO: implement highlight adapter from tree-sitter to LSP SemanticTokenType

/// Computes highlights from elaborated syntax and metadata in
/// [`Database`](crate::database::Database).
pub struct Highlighter {
    rx: watch::Receiver<Message>,
    synchronizer: Arc<Synchronizer>,
}

impl Highlighter {
    pub fn new(rx: watch::Receiver<Message>, synchronizer: Arc<Synchronizer>) -> Fallible<Self> {
        Ok(Highlighter { rx, synchronizer })
    }

    pub async fn init(&self) {
        let mut rx = self.rx.clone();
        while let Some(_value) = rx.recv().await {
            log::info!("{:?}", rx);
        }
    }
}
