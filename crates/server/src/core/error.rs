//! Core definitions related to runtime errors.

use thiserror::Error;
use tower_lsp::lsp_types::*;

/// Runtime errors for the WebAssembly language server.
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Error)]
pub(crate) enum Error {
    /// Error that a given document could not be found.
    #[error("ClientNotInitialzed")]
    ClientNotInitialized,
    /// Error that a given document could not be found.
    #[error("core::DocumentNotFound: {0}")]
    DocumentNotFound(Url),
}

/// Convenience newtype wrapper for convertion to jsonrpc_core::Error.
pub(crate) struct IntoJsonRpcError(pub anyhow::Error);

impl From<IntoJsonRpcError> for tower_lsp::jsonrpc::Error {
    fn from(error: IntoJsonRpcError) -> Self {
        let mut rpc_error = tower_lsp::jsonrpc::Error::internal_error();
        rpc_error.message = format!("{}", error.0);
        rpc_error
    }
}
