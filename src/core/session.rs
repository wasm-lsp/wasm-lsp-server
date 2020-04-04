//! Core functionality related to the LSP server session.

use crate::core::{database::Database, document::Document};
use dashmap::DashMap;
use failure::Fallible;
use lsp_types::*;
use std::sync::Arc;

/// Represents the current state of the LSP service.
pub(crate) struct Session {
    #[allow(dead_code)]
    database: Database,
    /// The store of currently open documents.
    pub(crate) documents: Arc<DashMap<Url, Document>>,
}

impl Session {
    /// Create a new session.
    pub(crate) fn new() -> Fallible<Self> {
        let database = Database::new()?;
        let documents = Arc::new(DashMap::new());
        Ok(Session { database, documents })
    }
}
