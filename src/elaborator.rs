use crate::session::SessionHandle;
use failure::Fallible;
use lsp_types::*;
use tower_lsp::Client;

/// Elaborates parse trees into structured data to be cached in the database.
pub struct Elaborator {
    session: SessionHandle,
}

impl Elaborator {
    pub fn new(session: SessionHandle) -> Fallible<Self> {
        Ok(Elaborator { session })
    }

    pub async fn tree_did_change(&self, _: &Client, uri: &Url) -> Fallible<()> {
        if let Some(tree) = self.session.get().await.synchronizer.trees.get(&uri) {
            let tree = tree.lock().await.clone();
            let node = tree.root_node();
            if !node.has_error() {
                log::info!("syntax well-formed");
            }
            // NOTE: else let auditor handle
            // TODO: allow partial elaboration in presence of syntax errors
        }
        Ok(())
    }

    pub async fn tree_did_close(&self, _: &Client, _: &Url) -> Fallible<()> {
        Ok(())
    }

    pub async fn tree_did_open(&self, client: &Client, uri: &Url) -> Fallible<()> {
        self.tree_did_change(client, uri).await
    }
}
