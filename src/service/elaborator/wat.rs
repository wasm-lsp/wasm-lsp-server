/// Elaborates parse trees into structured data to be cached in the database.

use crate::core::session::Session;
use lsp_types::*;
use std::sync::Arc;

// FIXME: reorganize this to where outline is pulled from database
pub(crate) async fn document_symbol(
    _session: Arc<Session>,
    _params: DocumentSymbolParams,
) -> jsonrpc_core::Result<Option<DocumentSymbolResponse>> {
    #![allow(non_snake_case)]
    log::info!("unimplemented");
    Ok(None)
}
