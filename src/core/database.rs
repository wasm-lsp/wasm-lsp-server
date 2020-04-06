//! Core functionality related to the document metadata database.

use crate::core::error::Fallible;
use log;
use sled;
use std::{env, path::Path};
use uuid::Uuid;
use zerocopy::{AsBytes, FromBytes};

/// Enum encoding the individual document status kinds.
#[derive(Clone, Debug)]
pub(crate) enum DocumentStatusKind {
    Closed = 0,
    Opened = 1,
}

/// Structure encoding the current document status.
#[derive(AsBytes, Clone, Copy, Debug, FromBytes)]
#[repr(C)]
pub(crate) struct DocumentStatus(u8);

impl DocumentStatus {
    pub(crate) fn closed() -> Self {
        DocumentStatus(DocumentStatusKind::Closed as u8)
    }

    pub(crate) fn opened() -> Self {
        DocumentStatus(DocumentStatusKind::Opened as u8)
    }
}

/// Trees active in the sled database.
pub(crate) struct Trees {
    pub(crate) documents: sled::Tree,
}

impl Trees {
    fn new(sled: &sled::Db) -> Fallible<Trees> {
        let documents = sled.open_tree("documents")?;
        Ok(Trees { documents })
    }
}

/// Caches elaborated syntax and metadata of documents.
pub(crate) struct Database {
    #[allow(dead_code)]
    sled: sled::Db,
    pub(crate) trees: Trees,
    #[allow(dead_code)]
    uuid: Uuid,
}

impl Database {
    /// Create a new database.
    pub(crate) fn new() -> Fallible<Self> {
        let uuid = Uuid::new_v4();
        let handle = Database::handle(&uuid);
        log::info!("{:?}", handle.clone());
        let sled = sled::Config::default()
            .create_new(true)
            .temporary(true)
            .use_compression(true)
            .path(handle)
            .open()?;
        let trees = Trees::new(&sled)?;
        Ok(Database { sled, trees, uuid })
    }

    /// Compute a handle for the database given a uuid.
    fn handle(uuid: &Uuid) -> Box<Path> {
        let mut buf = env::temp_dir();
        buf.push(uuid.to_string());
        buf.set_extension("sled");
        buf.into_boxed_path()
    }
}
