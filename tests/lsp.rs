#[cfg(feature = "test")]
mod test {
    use serde_json::{from_value, json};
    use std::task::Poll;
    use tower_lsp::{jsonrpc, lsp_types::*, ExitedError, Incoming};
    use wasm_language_server::test;

    #[tokio::test]
    async fn initialize_once() -> anyhow::Result<()> {
        let service = &mut test::service::spawn()?;

        let request: &Incoming = &from_value(json!({
            "jsonrpc": "2.0",
            "method": "initialize",
            "params": {
                "capabilities":{},
            },
            "id": 1,
        }))?;

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
        let request = &from_value(json!({
            "jsonrpc": "2.0",
            "method": "initialize",
            "params": {
                "capabilities":{},
            },
            "id": 1,
        }))?;
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
        let request = &from_value(json!({
            "jsonrpc": "2.0",
            "method": "initialized",
        }))
        .unwrap();
        let response = None;
        assert_eq!(test::service::call(service, request).await, Ok(response));

        assert_eq!(service.poll_ready(), Poll::Ready(Ok(())));
        let request = &from_value(json!({
            "jsonrpc": "2.0",
            "method": "exit",
        }))
        .unwrap();
        let response = None;
        assert_eq!(test::service::call(service, request).await, Ok(response));

        assert_eq!(service.poll_ready(), Poll::Ready(Err(ExitedError)));
        let request = &from_value(json!({
            "jsonrpc": "2.0",
            "method": "initialized",
        }))
        .unwrap();
        let error = ExitedError;
        assert_eq!(test::service::call(service, request).await, Err(error));
    }
}
