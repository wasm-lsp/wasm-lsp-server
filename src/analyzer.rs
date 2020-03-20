use crate::{database::Database, synchronizer::Synchronizer};
use failure::Fallible;
use std::sync::Arc;
use tokio::sync::watch;

/// Computes queries from elaborated syntax and metadata in [`Database`](crate::database::Database).
pub struct Analyzer {
    database: Arc<Database>,
    rx: watch::Receiver<()>,
    synchronizer: Arc<Synchronizer>,
}

impl Analyzer {
    pub fn new(database: Arc<Database>, rx: watch::Receiver<()>, synchronizer: Arc<Synchronizer>) -> Fallible<Self> {
        Ok(Analyzer {
            database,
            rx,
            synchronizer,
        })
    }

    pub async fn init(&self) {
        let mut rx = self.rx.clone();
        while let Some(_value) = rx.recv().await {
            log::info!("rx");
        }
    }
}
