//! Definitions for the server instance.

use crate::core::{error::Fallible, session::Session};
use std::sync::Arc;
use tower_lsp::Client;

/// The WASM language server instance.
pub struct Server {
    /// The LSP client handle.
    pub(crate) client: Client,
    /// The current state of the server.
    pub(crate) session: Arc<Session>,
}

impl Server {
    /// Create a new server.
    pub fn new(client: Client) -> Fallible<Self> {
        let session = Arc::new(Session::new(client.clone())?);
        Ok(Server { client, session })
    }
}
