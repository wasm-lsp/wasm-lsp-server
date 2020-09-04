//! JSON-RPC-related definitions.

/// Definitions for constructing error messages.
pub mod error {
    use serde_json::{json, Value};
    use tower_lsp::jsonrpc;

    /// Construct an `invalid request` error.
    pub fn invalid_request() -> Value {
        json!({
            "jsonrpc": "2.0",
            "error": {
                "code": jsonrpc::ErrorCode::InvalidRequest.code(),
                "message": "Invalid request",
            },
            "id": 1,
        })
    }
}
