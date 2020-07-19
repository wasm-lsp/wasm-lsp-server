use serde_json::{from_value, json, Value};
use std::task::Poll;
use tower_lsp::{jsonrpc, lsp_types::*, Incoming, LspService};

#[tokio::test]
async fn initialize_once() -> anyhow::Result<()> {
    let server = wasm_language_server::lsp::server::Server::new()?;
    let (service, _) = LspService::new(server);
    let mut service = tower_test::mock::Spawn::new(service);

    let request: Incoming = from_value(json!({
        "jsonrpc": "2.0",
        "method": "initialize",
        "params": {
            "capabilities":{},
        },
        "id": 1,
    }))?;

    assert_eq!(service.poll_ready(), Poll::Ready(Ok(())));
    service.call(request.clone()).await?.unwrap().parse::<Value>()?;

    assert_eq!(service.poll_ready(), Poll::Ready(Ok(())));
    assert_eq!(
        service.call(request).await?.unwrap().parse::<Value>()?,
        json!({
            "jsonrpc": "2.0",
            "error": {
                "code": jsonrpc::ErrorCode::InvalidRequest.code(),
                "message": "Invalid request",
            },
            "id": 1,
        })
    );

    Ok(())
}

#[tokio::test]
async fn initialize_capabilities() -> anyhow::Result<()> {
    let server = wasm_language_server::lsp::server::Server::new()?;
    let (service, _) = LspService::new(server);
    let mut service = tower_test::mock::Spawn::new(service);

    let request: Incoming = from_value(json!({
        "jsonrpc": "2.0",
        "method": "initialize",
        "params": {
            "capabilities":{},
        },
        "id": 1,
    }))?;

    assert_eq!(service.poll_ready(), Poll::Ready(Ok(())));
    assert_eq!(
        service.call(request.clone()).await?.unwrap().parse::<Value>()?,
        json!({
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
        })
    );

    Ok(())
}
