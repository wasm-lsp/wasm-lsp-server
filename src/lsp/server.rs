//! Definitions for the server instance.

use crate::core::session::Session;
use failure::Fallible;
use std::sync::Arc;

/// The WASM language server instance.
pub struct Server {
    /// The current state of the server.
    pub(crate) session: Arc<Session>,
}

impl Server {
    /// Create a new server.
    pub fn new() -> Fallible<Self> {
        let session = Arc::new(Session::new()?);
        Ok(Server { session })
    }
}
