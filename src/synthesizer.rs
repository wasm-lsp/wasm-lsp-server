use crate::{database::Database, message::Message, synchronizer::Synchronizer};
use failure::Fallible;
use std::sync::Arc;
use tokio::sync::watch::Receiver;

/// Synthesizes typing information for documents.
pub struct Synthesizer {
    database: Arc<Database>,
    receiver: Receiver<Message>,
    synchronizer: Arc<Synchronizer>,
}

impl Synthesizer {
    pub fn new(
        database: Arc<Database>,
        receiver: Receiver<Message>,
        synchronizer: Arc<Synchronizer>,
    ) -> Fallible<Self> {
        Ok(Synthesizer {
            database,
            receiver,
            synchronizer,
        })
    }

    pub async fn init(&self) -> Fallible<()> {
        let mut receiver = self.receiver.clone();
        while let Some(message) = receiver.recv().await {
            log::info!("{:?}", message);
        }
        Ok(())
    }
}
