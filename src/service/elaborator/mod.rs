//! Elaborates parse trees into structured data to be cached in the database.

/// Elaborator definitions specific to ".wast" files.
mod wast;
/// Elaborator definitions specific to ".wat" files.
mod wat;
/// Elaborator definitions specific to ".wit" files.
mod wit;
/// Elaborator definitions specific to ".witx" files.
mod witx;

/// Functions related to processing parse tree events for a document.
pub(crate) mod tree {
    use crate::core::session::Session;
    use failure::Fallible;
    use lsp_types::*;
    use std::sync::Arc;
    use tower_lsp::Client;

    /// Handle a parse tree "change" event.
    pub(crate) async fn change(session: Arc<Session>, _: &Client, uri: Url) -> Fallible<()> {
        if let Some(document) = session.documents.get(&uri) {
            let tree = document.tree.lock().await.clone();
            let node = tree.root_node();
            if !node.has_error() {
                log::info!("syntax well-formed");
            }
            // NOTE: else let auditor handle
            // TODO: allow partial elaboration in presence of syntax errors
        }
        Ok(())
    }

    /// Handle a parse tree "close" event.
    pub(crate) async fn close(_: Arc<Session>, _: &Client, _: Url) -> Fallible<()> {
        Ok(())
    }

    /// Handle a parse tree "open" event.
    pub(crate) async fn open(session: Arc<Session>, client: &Client, uri: Url) -> Fallible<()> {
        self::change(session, client, uri).await
    }
}

use crate::core::{language::Language, session::Session};
use lsp_types::*;
use std::sync::Arc;

// FIXME: reorganize this to where outline is pulled from database
/// Compute the symbols for a given document.
pub(crate) async fn document_symbol(
    session: Arc<Session>,
    params: DocumentSymbolParams,
) -> jsonrpc_core::Result<Option<DocumentSymbolResponse>> {
    let DocumentSymbolParams {
        text_document: TextDocumentIdentifier { uri },
    } = &params;
    if let Some(document) = session.documents.get(uri) {
        match document.language {
            Language::Wast => self::wast::document_symbol(session.clone(), params).await,
            Language::Wat => self::wat::document_symbol(session.clone(), params).await,
            Language::Wit => self::wit::document_symbol(session.clone(), params).await,
            Language::Witx => self::witx::document_symbol(session.clone(), params).await,
        }
    } else {
        // TODO: report
        log::warn!("documents.get failed for {}", uri);
        Ok(None)
    }
}