//! Definitions related to runtime errors.

use thiserror::Error;

/// Runtime errors for the LSP server.
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Eq, Error, PartialEq)]
pub enum Error {
    /// Error that occurs when [`core::Session.client`] is accessed and is `None`.
    #[error("ClientNotInitialzed")]
    ClientNotInitialized,
    /// Error that occurs when a session resource is requested and does not exist.
    #[error("core::SessionResourceNotFound: kind={kind:?}, uri={uri:?}")]
    SessionResourceNotFound {
        /// The kind of the requested session resource.
        kind: crate::core::session::SessionResourceKind,
        /// The URL of the requested session resource.
        uri: lsp::Url,
    },
}

/// Wrapper struct for converting [`anyhow::Error`] into [`tower_lsp::jsonrpc::Error`].
pub struct IntoJsonRpcError(pub anyhow::Error);

impl From<IntoJsonRpcError> for tower_lsp::jsonrpc::Error {
    fn from(error: IntoJsonRpcError) -> Self {
        let mut rpc_error = tower_lsp::jsonrpc::Error::internal_error();
        rpc_error.data = Some(serde_json::to_value(format!("{}", error.0)).unwrap());
        rpc_error
    }
}

#[cfg(test)]
mod tests {
    use super::{Error, IntoJsonRpcError};

    #[test]
    fn from() {
        let error = Error::ClientNotInitialized;
        let error = error.into();

        let mut expected = tower_lsp::jsonrpc::Error::internal_error();
        expected.data = Some(serde_json::to_value(format!("{}", error)).unwrap());

        let actual: tower_lsp::jsonrpc::Error = IntoJsonRpcError(error).into();

        assert_eq!(expected, actual);
    }
}
