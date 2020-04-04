/// Elaborates parse trees into structured data to be cached in the database.
use crate::core::document::Document;
use dashmap::DashMap;
use lsp_types::*;
use std::sync::Arc;

// FIXME: reorganize this to where outline is pulled from database
pub(crate) async fn document_symbol(
    _documents: Arc<DashMap<Url, Document>>,
    _params: DocumentSymbolParams,
) -> jsonrpc_core::Result<Option<DocumentSymbolResponse>> {
    #![allow(non_snake_case)]
    log::info!("unimplemented");
    Ok(None)
}
