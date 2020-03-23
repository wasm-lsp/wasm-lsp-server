use crate::{database::Database, message::Message, synchronizer::Synchronizer};
use failure::Fallible;
use std::sync::Arc;
use tokio::sync::watch;

/// Computes queries from elaborated syntax and metadata in [`Database`](crate::database::Database).
pub struct Analyzer {
    database: Arc<Database>,
    receiver: watch::Receiver<Message>,
    synchronizer: Arc<Synchronizer>,
}

impl Analyzer {
    pub fn new(
        database: Arc<Database>,
        receiver: watch::Receiver<Message>,
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
