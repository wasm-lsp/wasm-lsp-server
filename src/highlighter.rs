use crate::{message::Message, synchronizer::Synchronizer};
use failure::Fallible;
use log;
use std::sync::Arc;
use tokio::sync::watch;

// TODO: implement highlight adapter from tree-sitter to LSP SemanticTokenType

/// Computes highlights from elaborated syntax and metadata in
/// [`Database`](crate::database::Database).
pub struct Highlighter {
    receiver: watch::Receiver<Message>,
    synchronizer: Arc<Synchronizer>,
}

impl Highlighter {
    pub fn new(receiver: watch::Receiver<Message>, synchronizer: Arc<Synchronizer>) -> Fallible<Self> {
        Ok(Highlighter { receiver, synchronizer })
    }

    pub async fn init(&self) -> Fallible<()> {
        let mut receiver = self.receiver.clone();
        while let Some(message) = receiver.recv().await {
            log::info!("{:?}", message);
        }
        Ok(())
    }
}
