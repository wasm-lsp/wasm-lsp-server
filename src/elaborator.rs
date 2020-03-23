use crate::{database::Database, message::Message, synchronizer::Synchronizer};
use failure::Fallible;
use std::sync::Arc;
use tokio::sync::watch;

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
        while let Some(_value) = rx.recv().await {
            log::info!("{:?}", rx);
        }
        Ok(())
    }
}
