
use crate::{database::Database, document::Document};
use dashmap::DashMap;
use failure::Fallible;
use lsp_types::*;
use std::sync::Arc;

/// Represents the current state of the LSP service.
pub struct Session {
    #[allow(dead_code)]
    pub database: Database,
    pub documents: Arc<DashMap<Url, Document>>,
}

impl Session {
    pub fn new() -> Fallible<Self> {
        let database = Database::new()?;
        let documents = Arc::new(DashMap::new());
        Ok(Session { database, documents })
    }
}
