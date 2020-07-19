#[allow(unused_imports)]
#[macro_use]
extern crate wasm_language_server;

#[cfg(feature = "test")]
mod test {
    use serde_json::{json, Value};
    use std::task::Poll;
    use tower_lsp::{jsonrpc, lsp_types::*, ExitedError};
    use wasm_language_server::test;

    #[tokio::test]
    async fn initialize_once() -> anyhow::Result<()> {
        let service = &mut test::service::spawn()?;

        let request = &json!({
            "jsonrpc": "2.0",
            "method": "initialize",
            "params": {
                "capabilities":{},
            },
            "id": 1,
        });

        // expect nominal response for first request
        assert_ready!(service, Ok(()));
        let response = Some(json!({
            "jsonrpc": "2.0",
            "result": {
                "capabilities": {
                    "documentSymbolProvider": true,
                    "textDocumentSync": {
                        "change": TextDocumentSyncKind::Full,
                        "openClose": true,
                    },
                },
            },
            "id": 1,
        }));
        assert_exchange!(service, request, Ok(response));

        // expect error response for second request
        assert_ready!(service, Ok(()));
        let response = Some(json!({
            "jsonrpc": "2.0",
            "error": {
                "code": jsonrpc::ErrorCode::InvalidRequest.code(),
                "message": "Invalid request",
            },
            "id": 1,
        }));
        assert_exchange!(service, request, Ok(response));

        Ok(())
    }

    #[tokio::test]
    async fn initialize() -> anyhow::Result<()> {
        let service = &mut test::service::spawn()?;

        assert_ready!(service, Ok(()));
        let request = &json!({
            "jsonrpc": "2.0",
            "method": "initialize",
            "params": {
                "capabilities":{},
            },
            "id": 1,
        });
        let response = Some(json!({
            "jsonrpc": "2.0",
            "result": {
                "capabilities": {
                    "documentSymbolProvider": true,
                    "textDocumentSync": {
                        "change": TextDocumentSyncKind::Full,
                        "openClose": true,
                    },
                },
            },
            "id": 1,
        }));
        assert_exchange!(service, request, Ok(response));

        Ok(())
    }

    #[tokio::test]
    async fn exit() {
        let service = &mut test::service::spawn().unwrap();

        assert_ready!(service, Ok(()));
        let request = &json!({ "jsonrpc": "2.0", "method": "initialized" });
        let response = None::<Value>;
        assert_exchange!(service, request, Ok(response));

        assert_ready!(service, Ok(()));
        let request = &json!({ "jsonrpc": "2.0", "method": "exit" });
        let response = None::<Value>;
        assert_exchange!(service, request, Ok(response));

        assert_ready!(service, Err(ExitedError));
        let request = &json!({ "jsonrpc": "2.0", "method": "initialized" });
        let error = ExitedError;
        assert_exchange!(service, request, Err(error));
    }
}
