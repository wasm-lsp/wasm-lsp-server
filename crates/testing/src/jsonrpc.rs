pub mod error {
    use serde_json::{json, Value};

    pub fn invalid_request() -> Value {
        json!({
            "jsonrpc": "2.0",
            "error": {
                "code": tower_lsp::jsonrpc::ErrorCode::InvalidRequest.code(),
                "message": "Invalid request",
            },
            "id": 1,
        })
    }
}
