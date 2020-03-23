use crate::{database::Database, message::Message, synchronizer::Synchronizer};
use failure::Fallible;
use std::sync::Arc;
use tokio::sync::watch::Receiver;

/// Computes queries about documents.
pub struct Analyzer {
    database: Arc<Database>,
    receiver: Receiver<Message>,
    synchronizer: Arc<Synchronizer>,
}

impl Analyzer {
    pub fn new(
        database: Arc<Database>,
        receiver: Receiver<Message>,
        synchronizer: Arc<Synchronizer>,
    ) -> Fallible<Self> {
        Ok(Analyzer {
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
