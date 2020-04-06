//! Elaborator definitions specific to ".witx" files.

use crate::core::document::Document;
use lsp_types::*;

/// Compute the symbols for a given document.
pub(crate) async fn document_symbol(_document: &Document) -> Option<DocumentSymbolResponse> {
    #![allow(non_snake_case)]
    log::info!("unimplemented");
    None
}
