pub mod error {
    use lspower::jsonrpc;
    use serde_json::{json, Value};

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
