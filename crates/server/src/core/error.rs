//! Core definitions related to runtime errors.

use thiserror::Error;
use tower_lsp::lsp_types::*;

/// Runtime errors for the WebAssembly language server.
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Error, PartialEq)]
pub(crate) enum Error {
    /// Error that a given document could not be found.
    #[error("ClientNotInitialzed")]
    ClientNotInitialized,
    /// Error that a given document could not be found.
    #[error("core::DocumentNotFound: {0}")]
    DocumentNotFound(Url),
}

/// Convenience newtype wrapper for convertion to jsonrpc_core::Error.
pub(crate) struct IntoJsonRpcError(pub(crate) anyhow::Error);

impl From<IntoJsonRpcError> for tower_lsp::jsonrpc::Error {
    fn from(error: IntoJsonRpcError) -> Self {
        let value = serde_json::to_value(format!("{}", error.0)).unwrap();
        let mut rpc_error = tower_lsp::jsonrpc::Error::internal_error();
        rpc_error.data = Some(value);
        rpc_error
    }
}

#[cfg(test)]
mod tests {
    use super::{Error, IntoJsonRpcError};
    use tower_lsp::jsonrpc;

    #[test]
    fn from() {
        let error = Error::ClientNotInitialized;
        let error = error.into();

        let code = jsonrpc::ErrorCode::InternalError;
        let message = code.description().to_string();
        let value = serde_json::to_value(format!("{}", error)).unwrap();
        let data = Some(value);

        let error: tower_lsp::jsonrpc::Error = IntoJsonRpcError(error).into();

        assert_eq!(error.code, code);
        assert_eq!(error.message, message);
        assert_eq!(error.data, data)
    }
}
