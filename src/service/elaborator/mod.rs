//! Elaborates parse trees into structured data to be cached in the database.

mod wast;
mod wat;
mod wit;
mod witx;

use crate::core::{language::Language, session::Session};
use failure::Fallible;
use lsp_types::*;
use std::sync::Arc;
use tower_lsp::Client;

pub(crate) async fn tree_did_change(session: Arc<Session>, _: &Client, uri: Url) -> Fallible<()> {
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

pub(crate) async fn tree_did_close(_: Arc<Session>, _: &Client, _: Url) -> Fallible<()> {
    Ok(())
}

pub(crate) async fn tree_did_open(session: Arc<Session>, client: &Client, uri: Url) -> Fallible<()> {
    self::tree_did_change(session, client, uri).await
}

// FIXME: reorganize this to where outline is pulled from database
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
