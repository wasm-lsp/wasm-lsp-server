//! Elaborator definitions specific to ".wat" files.

use crate::core::session::Session;
use lsp_types::*;
use std::sync::Arc;

// FIXME: reorganize this to where outline is pulled from database
/// Compute the symbols for a given document.
pub(crate) async fn document_symbol(
    _session: Arc<Session>,
    _params: DocumentSymbolParams,
) -> jsonrpc_core::Result<Option<DocumentSymbolResponse>> {
    #![allow(non_snake_case)]
    log::info!("unimplemented");
    Ok(None)
}
