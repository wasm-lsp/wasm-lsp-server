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
        assert_eq!(service.poll_ready(), Poll::Ready(Ok(())));
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
        assert_eq!(test::service::call(service, request).await?, response);

        // expect error response for second request
        assert_eq!(service.poll_ready(), Poll::Ready(Ok(())));
        let response = Some(json!({
            "jsonrpc": "2.0",
            "error": {
                "code": jsonrpc::ErrorCode::InvalidRequest.code(),
                "message": "Invalid request",
            },
            "id": 1,
        }));
        assert_eq!(test::service::call(service, request).await?, response);

        Ok(())
    }

    #[tokio::test]
    async fn initialize() -> anyhow::Result<()> {
        let service = &mut test::service::spawn()?;

        assert_eq!(service.poll_ready(), Poll::Ready(Ok(())));
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
        assert_eq!(test::service::call(service, request).await?, response);

        Ok(())
    }

    #[tokio::test]
    async fn exit() {
        let service = &mut test::service::spawn().unwrap();

        assert_eq!(service.poll_ready(), Poll::Ready(Ok(())));
        let request = &json!({ "jsonrpc": "2.0", "method": "initialized" });
        let response = None::<Value>;
        assert_eq!(test::service::call(service, request).await, Ok(response));

        assert_eq!(service.poll_ready(), Poll::Ready(Ok(())));
        let request = &json!({ "jsonrpc": "2.0", "method": "exit" });
        let response = None::<Value>;
        assert_eq!(test::service::call(service, request).await, Ok(response));

        assert_eq!(service.poll_ready(), Poll::Ready(Err(ExitedError)));
        let request = &json!({ "jsonrpc": "2.0", "method": "initialized" });
        let error = ExitedError;
        assert_eq!(test::service::call(service, request).await, Err(error));
    }
}
