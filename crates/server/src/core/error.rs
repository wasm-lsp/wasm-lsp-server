use crate::core;
use thiserror::Error;

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Error, PartialEq)]
pub enum Error {
    #[error("ClientNotInitialzed")]
    ClientNotInitialized,
    #[error("ColumnOutOfBounds: given={given:?}, max={max:?}")]
    ColumnOutOfBounds { given: usize, max: usize },
    #[error("core::SessionResourceNotFound: kind={kind:?}, uri={uri:?}")]
    SessionResourceNotFound {
        kind: core::session::SessionResourceKind,
        uri: lsp::Url,
    },
    #[error("LineOutOfBounds: given={given:?}, max={max:?}")]
    LineOutOfBounds { given: usize, max: usize },
}

pub struct IntoJsonRpcError(pub anyhow::Error);

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
