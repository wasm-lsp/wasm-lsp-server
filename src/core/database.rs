use failure::Fallible;
use log;
use sled;
use std::{env, path::Path};
use uuid::Uuid;

/// Caches elaborated syntax and metadata of documents.
pub(crate) struct Database {
    #[allow(dead_code)]
    sled: sled::Db,
    #[allow(dead_code)]
    uuid: Uuid,
}

impl Database {
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
        Ok(Database { sled, uuid })
    }

    fn handle(uuid: &Uuid) -> Box<Path> {
        let mut buf = env::temp_dir();
        buf.push(uuid.to_string());
        buf.set_extension("sled");
        buf.into_boxed_path()
    }
}
