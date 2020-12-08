//! Core definitions related to runtime errors.

use lspower::lsp_types::*;
use thiserror::Error;

/// Runtime errors for the WebAssembly language server.
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Error, PartialEq)]
pub(crate) enum Error {
    /// Error that a given document could not be found.
    #[error("ClientNotInitialzed")]
    ClientNotInitialized,
    /// Error that a given column index is out of bounds for a line of text in a document.
    #[error("ColumnOutOfBounds: given={given:?}, max={max:?}")]
    ColumnOutOfBounds { given: usize, max: usize },
    /// Error that a given document could not be found.
    #[error("core::DocumentNotFound: {0}")]
    DocumentNotFound(Url),
    /// Error that a given line index is out of bounds for a document.
    #[error("LineOutOfBounds: given={given:?}, max={max:?}")]
    LineOutOfBounds { given: usize, max: usize },
}

/// Convenience newtype wrapper for convertion to jsonrpc_core::Error.
pub(crate) struct IntoJsonRpcError(pub(crate) anyhow::Error);

impl From<IntoJsonRpcError> for lspower::jsonrpc::Error {
    fn from(error: IntoJsonRpcError) -> Self {
        let mut rpc_error = lspower::jsonrpc::Error::internal_error();
        rpc_error.data = Some(serde_json::to_value(format!("{}", error.0)).unwrap());
        rpc_error
    }
}

#[cfg(test)]
mod tests {
    use super::{Error, IntoJsonRpcError};
    use lspower::jsonrpc;

    #[test]
    fn from() {
        let error = Error::ClientNotInitialized;
        let error = error.into();

        let mut expected = jsonrpc::Error::internal_error();
        expected.data = Some(serde_json::to_value(format!("{}", error)).unwrap());

        let actual: lspower::jsonrpc::Error = IntoJsonRpcError(error).into();

        assert_eq!(expected, actual);
    }
}
