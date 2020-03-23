use failure::Fallible;
use log;
use sled;
use std::{env, path::Path};
use uuid::Uuid;

/// Caches elaborated syntax and metadata of documents.
pub struct Database {
    sled: sled::Db,
    uuid: Uuid,
}

impl Database {
    pub fn new() -> Fallible<Self> {
        let uuid = Uuid::new_v4();
        let handle = Database::handle(&uuid);
        log::info!("{:?}", handle.clone());
        let sled = sled::Config::default()
            .create_new(true)
            .temporary(true)
            .use_compression(true)
            .path(handle)
            .open()?;
        Ok(Database { sled, uuid })
    }

    pub fn handle(uuid: &Uuid) -> Box<Path> {
        let mut buf = env::temp_dir();
        buf.push(uuid.to_string());
        buf.set_extension("sled");
        buf.into_boxed_path()
    }
}
