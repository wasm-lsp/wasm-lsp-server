//! Elaborates parse trees into structured data to be cached in the database.

mod wast;
mod wat;
mod wit;
mod witx;

use crate::core::{document::Document, language::Language};
use dashmap::DashMap;
use failure::Fallible;
use lsp_types::*;
use std::sync::Arc;
use tower_lsp::Client;

pub(crate) async fn tree_did_change(documents: Arc<DashMap<Url, Document>>, _: &Client, uri: Url) -> Fallible<()> {
    if let Some(document) = documents.get(&uri) {
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

pub(crate) async fn tree_did_close(_: Arc<DashMap<Url, Document>>, _: &Client, _: Url) -> Fallible<()> {
    Ok(())
}

pub(crate) async fn tree_did_open(documents: Arc<DashMap<Url, Document>>, client: &Client, uri: Url) -> Fallible<()> {
    self::tree_did_change(documents, client, uri).await
}

// FIXME: reorganize this to where outline is pulled from database
pub(crate) async fn document_symbol(
    documents: Arc<DashMap<Url, Document>>,
    params: DocumentSymbolParams,
) -> jsonrpc_core::Result<Option<DocumentSymbolResponse>> {
    let DocumentSymbolParams {
        text_document: TextDocumentIdentifier { uri },
    } = &params;
    if let Some(document) = documents.get(uri) {
        match document.language {
            Language::Wast => self::wast::document_symbol(documents.clone(), params).await,
            Language::Wat => self::wat::document_symbol(documents.clone(), params).await,
            Language::Wit => self::wit::document_symbol(documents.clone(), params).await,
            Language::Witx => self::witx::document_symbol(documents.clone(), params).await,
        }
    } else {
        // TODO: report
        log::warn!("documents.get failed for {}", uri);
        Ok(None)
    }
}
