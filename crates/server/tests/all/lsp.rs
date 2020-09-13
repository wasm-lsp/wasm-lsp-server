use serde_json::Value;
use tower_lsp::{lsp_types::*, ExitedError};
use wasm_language_server_testing as testing;

#[tokio::test]
async fn exit() -> anyhow::Result<()> {
    let service = &mut testing::service::spawn()?.0;

    // send "initialize" request
    testing::assert_status!(service, Ok(()));
    let request = &testing::lsp::initialize::request();
    let response = Some(testing::lsp::initialize::response());
    testing::assert_exchange!(service, request, Ok(response));

    // send "initialized" notification
    testing::assert_status!(service, Ok(()));
    let notification = &testing::lsp::initialized::notification();
    let status = None::<Value>;
    testing::assert_exchange!(service, notification, Ok(status));

    // send "exit" notification
    testing::assert_status!(service, Ok(()));
    let notification = &testing::lsp::exit::notification();
    let status = None::<Value>;
    testing::assert_exchange!(service, notification, Ok(status));

    // send "textDocument/didOpen" notification; should error
    testing::assert_status!(service, Err(ExitedError));
    let notification = &{
        let uri = Url::parse("inmemory::///test")?;
        let language_id = "wasm.wat";
        let text = String::from("");
        testing::lsp::text_document::did_open::notification(&uri, language_id, 1, text)
    };
    let status = ExitedError;
    testing::assert_exchange!(service, notification, Err(status));

    Ok(())
}

#[tokio::test]
async fn initialize() -> anyhow::Result<()> {
    let service = &mut testing::service::spawn()?.0;

    // send "initialize" request
    testing::assert_status!(service, Ok(()));
    let request = &testing::lsp::initialize::request();
    let response = Some(testing::lsp::initialize::response());
    testing::assert_exchange!(service, request, Ok(response));

    Ok(())
}

#[tokio::test]
async fn initialized() -> anyhow::Result<()> {
    let service = &mut testing::service::spawn()?.0;

    // send "initialize" request
    testing::assert_status!(service, Ok(()));
    let request = &testing::lsp::initialize::request();
    let response = Some(testing::lsp::initialize::response());
    testing::assert_exchange!(service, request, Ok(response));

    // send "initialized" notification
    testing::assert_status!(service, Ok(()));
    let notification = &testing::lsp::initialized::notification();
    let status = None;
    testing::assert_exchange!(service, notification, Ok(status));

    Ok(())
}

#[tokio::test]
async fn initialize_once() -> anyhow::Result<()> {
    let service = &mut testing::service::spawn()?.0;

    // send "initialize" request
    testing::assert_status!(service, Ok(()));
    let request = &testing::lsp::initialize::request();
    let response = Some(testing::lsp::initialize::response());
    testing::assert_exchange!(service, request, Ok(response));

    // send "initialized" notification
    testing::assert_status!(service, Ok(()));
    let notification = &testing::lsp::initialized::notification();
    let status = None::<Value>;
    testing::assert_exchange!(service, notification, Ok(status));

    // send "initialize" request (again); should error
    testing::assert_status!(service, Ok(()));
    let request = &testing::lsp::initialize::request();
    let response = Some(testing::jsonrpc::error::invalid_request());
    testing::assert_exchange!(service, request, Ok(response));

    Ok(())
}

#[tokio::test]
async fn shutdown() -> anyhow::Result<()> {
    let service = &mut testing::service::spawn()?.0;

    // send "initialize" request
    testing::assert_status!(service, Ok(()));
    let request = &testing::lsp::initialize::request();
    let response = Some(testing::lsp::initialize::response());
    testing::assert_exchange!(service, request, Ok(response));

    // send "initialized" notification
    testing::assert_status!(service, Ok(()));
    let notification = &testing::lsp::initialized::notification();
    let status = None::<Value>;
    testing::assert_exchange!(service, notification, Ok(status));

    // send "shutdown" request
    testing::assert_status!(service, Ok(()));
    let request = &testing::lsp::shutdown::request();
    let response = Some(testing::lsp::shutdown::response());
    testing::assert_exchange!(service, request, Ok(response));

    // send "shutdown" request (again); should error
    testing::assert_status!(service, Ok(()));
    let request = &testing::lsp::shutdown::request();
    let response = Some(testing::jsonrpc::error::invalid_request());
    testing::assert_exchange!(service, request, Ok(response));

    // send "exit" notification
    testing::assert_status!(service, Ok(()));
    let notification = &testing::lsp::exit::notification();
    let status = None::<Value>;
    testing::assert_exchange!(service, notification, Ok(status));

    Ok(())
}

mod text_document {
    use futures::stream::StreamExt;
    use serde_json::{json, Value};
    use tower_lsp::lsp_types::*;
    use wasm_language_server_testing as testing;

    #[tokio::test]
    async fn did_change() -> anyhow::Result<()> {
        let uri = Url::parse("inmemory:///test")?;
        let language_id = "wasm.wat";

        let old_text = String::from("");
        let new_text = String::from("(module $m (func $f))");

        let (ref mut service, ref mut messages) = testing::service::spawn()?;

        // send "initialize" request
        testing::assert_status!(service, Ok(()));
        let request = &testing::lsp::initialize::request();
        let response = Some(testing::lsp::initialize::response());
        testing::assert_exchange!(service, request, Ok(response));

        // send "initialized" notification
        testing::assert_status!(service, Ok(()));
        let notification = &testing::lsp::initialized::notification();
        let status = None::<Value>;
        testing::assert_exchange!(service, notification, Ok(status));
        // ignore the "window/logMessage" notification: "WebAssembly language server initialized!"
        messages.next().await.unwrap();

        // send "textDocument/didOpen" notification for `uri`
        testing::assert_status!(service, Ok(()));
        let notification = &testing::lsp::text_document::did_open::notification(&uri, language_id, 1, old_text);
        let status = None::<Value>;
        testing::assert_exchange!(service, notification, Ok(status));

        // receive "textDocument/publishDiagnostics" notification for `uri`
        let message = messages.next().await.unwrap();
        let actual = serde_json::to_value(&message)?;
        let expected = testing::lsp::text_document::publish_diagnostics::notification(&uri, &[]);
        assert_eq!(actual, expected);

        // send "textDocument/didChange" notification for `uri`
        testing::assert_status!(service, Ok(()));
        let notification = &testing::lsp::text_document::did_change::notification::entire(&uri, new_text);
        let status = None::<Value>;
        testing::assert_exchange!(service, notification, Ok(status));

        // receive "textDocument/publishDiagnostics" notification for `uri`
        let message = messages.next().await.unwrap();
        let actual = serde_json::to_value(&message)?;
        let expected = testing::lsp::text_document::publish_diagnostics::notification(&uri, &[]);
        assert_eq!(actual, expected);

        // send "shutdown" request
        testing::assert_status!(service, Ok(()));
        let request = &testing::lsp::shutdown::request();
        let response = Some(testing::lsp::shutdown::response());
        testing::assert_exchange!(service, request, Ok(response));

        // send "exit" notification
        testing::assert_status!(service, Ok(()));
        let notification = &testing::lsp::exit::notification();
        let status = None::<Value>;
        testing::assert_exchange!(service, notification, Ok(status));

        Ok(())
    }

    mod did_change {
        use futures::stream::StreamExt;
        use serde_json::{json, Value};
        use tower_lsp::lsp_types::*;
        use wasm_language_server_testing as testing;

        #[tokio::test]
        async fn lexer_error() -> anyhow::Result<()> {
            let uri = Url::parse("inmemory:///test")?;
            let language_id = "wasm.wat";

            let old_text = String::from("");
            let new_text = String::from("(module (func (export \"\\x\"))");

            let (ref mut service, ref mut messages) = testing::service::spawn()?;

            // send "initialize" request
            testing::assert_status!(service, Ok(()));
            let request = &testing::lsp::initialize::request();
            let response = Some(testing::lsp::initialize::response());
            testing::assert_exchange!(service, request, Ok(response));

            // send "initialized" notification
            testing::assert_status!(service, Ok(()));
            let notification = &testing::lsp::initialized::notification();
            let status = None::<Value>;
            testing::assert_exchange!(service, notification, Ok(status));
            // ignore the "window/logMessage" notification: "WebAssembly language server initialized!"
            messages.next().await.unwrap();

            // send "textDocument/didOpen" notification for `uri`
            testing::assert_status!(service, Ok(()));
            let notification = &testing::lsp::text_document::did_open::notification(&uri, language_id, 1, old_text);
            let status = None::<Value>;
            testing::assert_exchange!(service, notification, Ok(status));

            // receive "textDocument/publishDiagnostics" notification for `uri`
            let message = messages.next().await.unwrap();
            let actual = serde_json::to_value(&message)?;
            let expected = testing::lsp::text_document::publish_diagnostics::notification(&uri, &[]);
            assert_eq!(actual, expected);

            // send "textDocument/didChange" notification for `uri`
            testing::assert_status!(service, Ok(()));
            let notification = &testing::lsp::text_document::did_change::notification::entire(&uri, new_text);
            let status = None::<Value>;
            testing::assert_exchange!(service, notification, Ok(status));

            // receive "textDocument/publishDiagnostics" notification for `uri`
            let message = messages.next().await.unwrap();
            let actual = serde_json::to_value(&message)?;
            let expected = json!({
                "jsonrpc": "2.0",
                "method": "textDocument/publishDiagnostics",
                "params": {
                    "uri": uri,
                    "diagnostics": [
                        {
                            "range": { "start": { "line": 0, "character": 24 }, "end": { "line": 0, "character": 24 } },
                            "severity": 1,
                            "source": "wast",
                            "message": "invalid string escape \'x\'",
                        },
                    ],
                },
            });
            assert_eq!(actual, expected);

            // send "shutdown" request
            testing::assert_status!(service, Ok(()));
            let request = &testing::lsp::shutdown::request();
            let response = Some(testing::lsp::shutdown::response());
            testing::assert_exchange!(service, request, Ok(response));

            // send "exit" notification
            testing::assert_status!(service, Ok(()));
            let notification = &testing::lsp::exit::notification();
            let status = None::<Value>;
            testing::assert_exchange!(service, notification, Ok(status));

            Ok(())
        }

        #[tokio::test]
        async fn parser_error() -> anyhow::Result<()> {
            let uri = Url::parse("inmemory:///test")?;
            let language_id = "wasm.wat";

            let old_text = String::from("");
            let new_text = String::from("(modu)");

            let (ref mut service, ref mut messages) = testing::service::spawn()?;

            // send "initialize" request
            testing::assert_status!(service, Ok(()));
            let request = &testing::lsp::initialize::request();
            let response = Some(testing::lsp::initialize::response());
            testing::assert_exchange!(service, request, Ok(response));

            // send "initialized" notification
            testing::assert_status!(service, Ok(()));
            let notification = &testing::lsp::initialized::notification();
            let status = None::<Value>;
            testing::assert_exchange!(service, notification, Ok(status));
            // ignore the "window/logMessage" notification: "WebAssembly language server initialized!"
            messages.next().await.unwrap();

            // send "textDocument/didOpen" notification for `uri`
            testing::assert_status!(service, Ok(()));
            let notification = &testing::lsp::text_document::did_open::notification(&uri, language_id, 1, old_text);
            let status = None::<Value>;
            testing::assert_exchange!(service, notification, Ok(status));

            // receive "textDocument/publishDiagnostics" notification for `uri`
            let message = messages.next().await.unwrap();
            let actual = serde_json::to_value(&message)?;
            let expected = testing::lsp::text_document::publish_diagnostics::notification(&uri, &[]);
            assert_eq!(actual, expected);

            // send "textDocument/didChange" notification for `uri`
            testing::assert_status!(service, Ok(()));
            let notification = &testing::lsp::text_document::did_change::notification::entire(&uri, new_text);
            let status = None::<Value>;
            testing::assert_exchange!(service, notification, Ok(status));

            // receive "textDocument/publishDiagnostics" notification for `uri`
            let message = messages.next().await.unwrap();
            let actual = serde_json::to_value(&message)?;
            let expected = json!({
                "jsonrpc": "2.0",
                "method": "textDocument/publishDiagnostics",
                "params": {
                    "uri": uri,
                    "diagnostics": [
                        {
                            "range": { "start": { "line": 0, "character": 1 }, "end": { "line": 0, "character": 1 } },
                            "severity": 1,
                            "source": "wast",
                            "message": "expected valid module field",
                        },
                    ],
                },
            });
            assert_eq!(actual, expected);

            // send "shutdown" request
            testing::assert_status!(service, Ok(()));
            let request = &testing::lsp::shutdown::request();
            let response = Some(testing::lsp::shutdown::response());
            testing::assert_exchange!(service, request, Ok(response));

            // send "exit" notification
            testing::assert_status!(service, Ok(()));
            let notification = &testing::lsp::exit::notification();
            let status = None::<Value>;
            testing::assert_exchange!(service, notification, Ok(status));

            Ok(())
        }
    }

    #[tokio::test]
    async fn did_close() -> anyhow::Result<()> {
        let uri = Url::parse("inmemory:///test")?;
        let language_id = "wasm.wast";
        let text = String::new();

        let (ref mut service, ref mut messages) = testing::service::spawn()?;

        // send "initialize" request
        testing::assert_status!(service, Ok(()));
        let request = &testing::lsp::initialize::request();
        let response = Some(testing::lsp::initialize::response());
        testing::assert_exchange!(service, request, Ok(response));

        // send "initialized" notification
        testing::assert_status!(service, Ok(()));
        let notification = &testing::lsp::initialized::notification();
        let status = None::<Value>;
        testing::assert_exchange!(service, notification, Ok(status));
        // ignore the "window/logMessage" notification: "WebAssembly language server initialized!"
        messages.next().await.unwrap();

        // send "textDocument/didOpen" notification for `uri`
        testing::assert_status!(service, Ok(()));
        let notification = &testing::lsp::text_document::did_open::notification(&uri, language_id, 1, text);
        let status = None::<Value>;
        testing::assert_exchange!(service, notification, Ok(status));

        // receive "textDocument/publishDiagnostics" notification for `uri`
        let message = messages.next().await.unwrap();
        let actual = serde_json::to_value(&message)?;
        let expected = testing::lsp::text_document::publish_diagnostics::notification(&uri, &[]);
        assert_eq!(actual, expected);

        // send "textDocument/didClose" notification for `uri`
        testing::assert_status!(service, Ok(()));
        let notification = &testing::lsp::text_document::did_close::notification(&uri);
        let status = None::<Value>;
        testing::assert_exchange!(service, notification, Ok(status));

        // send "shutdown" request
        testing::assert_status!(service, Ok(()));
        let request = &testing::lsp::shutdown::request();
        let response = Some(testing::lsp::shutdown::response());
        testing::assert_exchange!(service, request, Ok(response));

        // send "exit" notification
        testing::assert_status!(service, Ok(()));
        let notification = &testing::lsp::exit::notification();
        let status = None::<Value>;
        testing::assert_exchange!(service, notification, Ok(status));

        Ok(())
    }

    #[tokio::test]
    async fn did_open() -> anyhow::Result<()> {
        let uri = Url::parse("inmemory:///test")?;
        let language_id = "wasm.wast";
        let text = String::new();

        let (ref mut service, ref mut messages) = testing::service::spawn()?;

        // send "initialize" request
        testing::assert_status!(service, Ok(()));
        let request = &testing::lsp::initialize::request();
        let response = Some(testing::lsp::initialize::response());
        testing::assert_exchange!(service, request, Ok(response));

        // send "initialized" notification
        testing::assert_status!(service, Ok(()));
        let notification = &testing::lsp::initialized::notification();
        let status = None::<Value>;
        testing::assert_exchange!(service, notification, Ok(status));
        // ignore the "window/logMessage" notification: "WebAssembly language server initialized!"
        messages.next().await.unwrap();

        // send "textDocument/didOpen" notification for `uri`
        testing::assert_status!(service, Ok(()));
        let notification = &testing::lsp::text_document::did_open::notification(&uri, language_id, 1, text);
        let status = None::<Value>;
        testing::assert_exchange!(service, notification, Ok(status));

        // receive "textDocument/publishDiagnostics" notification for `uri`
        let message = messages.next().await.unwrap();
        let actual = serde_json::to_value(&message)?;
        let expected = testing::lsp::text_document::publish_diagnostics::notification(&uri, &[]);
        assert_eq!(actual, expected);

        // send "shutdown" request
        testing::assert_status!(service, Ok(()));
        let request = &testing::lsp::shutdown::request();
        let response = Some(testing::lsp::shutdown::response());
        testing::assert_exchange!(service, request, Ok(response));

        // send "exit" notification
        testing::assert_status!(service, Ok(()));
        let notification = &testing::lsp::exit::notification();
        let status = None::<Value>;
        testing::assert_exchange!(service, notification, Ok(status));

        Ok(())
    }

    mod document_symbol {
        use futures::stream::StreamExt;
        use serde_json::{json, Value};
        use tower_lsp::lsp_types::*;
        use wasm_language_server_testing as testing;

        #[tokio::test]
        async fn wat() -> anyhow::Result<()> {
            let uri = Url::parse("inmemory:///test")?;
            let language_id = "wasm.wat";
            #[rustfmt::skip]
            let text = String::from(indoc::indoc! {r#"
                (type $a (func))
                (global $g i32 (i32.const 0))
                (memory $m 1)
                (data (i32.const 0))
                (table $t 10 funcref)
                (func $f)
                (elem (i32.const 0))
            "#});

            let (ref mut service, ref mut messages) = testing::service::spawn()?;

            // send "initialize" request
            testing::assert_status!(service, Ok(()));
            let request = &testing::lsp::initialize::request();
            let response = Some(testing::lsp::initialize::response());
            testing::assert_exchange!(service, request, Ok(response));

            // send "initialized" notification
            testing::assert_status!(service, Ok(()));
            let notification = &testing::lsp::initialized::notification();
            let status = None::<Value>;
            testing::assert_exchange!(service, notification, Ok(status));
            // ignore the "window/logMessage" notification: "WebAssembly language server initialized!"
            messages.next().await.unwrap();

            // send "textDocument/didOpen" notification for `uri`
            testing::assert_status!(service, Ok(()));
            let notification = &testing::lsp::text_document::did_open::notification(&uri, language_id, 1, text);
            let status = None::<Value>;
            testing::assert_exchange!(service, notification, Ok(status));

            // receive "textDocument/publishDiagnostics" notification for `uri`
            let message = messages.next().await.unwrap();
            let actual = serde_json::to_value(&message)?;
            let expected = testing::lsp::text_document::publish_diagnostics::notification(&uri, &[]);
            assert_eq!(actual, expected);

            // send "textDocument/documentSymbol" request for `uri`
            testing::assert_status!(service, Ok(()));
            let request = &testing::lsp::text_document::document_symbol::request(&uri);
            #[rustfmt::skip]
            let response = Some(json!({
                "jsonrpc": "2.0",
                "result": [
                    {
                        "name": "$a",
                        "kind": SymbolKind::TypeParameter,
                        "range": { "start": { "line": 0, "character": 0 }, "end": { "line": 0, "character": 16 } },
                        "selectionRange": { "start": { "line": 0, "character": 6 }, "end": { "line": 0, "character": 8 } },
                        "children": [],
                    },
                    {
                        "name": "$g",
                        "kind": SymbolKind::Event,
                        "range": { "start": { "line": 1, "character": 0 }, "end": { "line": 1, "character": 29 } },
                        "selectionRange": { "start": { "line": 1, "character": 8 }, "end": { "line": 1, "character": 10 } },
                        "children": [],
                    },
                    {
                        "name": "$m",
                        "kind": SymbolKind::Array,
                        "range": { "start": { "line": 2, "character": 0 }, "end": { "line": 2, "character": 13 } },
                        "selectionRange": { "start": { "line": 2, "character": 8 }, "end": { "line": 2, "character": 10 } },
                        "children": [],
                    },
                    {
                        "name": "<data@4:1>",
                        "kind": SymbolKind::Key,
                        "range": { "start": { "line": 3, "character": 0 }, "end": { "line": 3, "character": 20 } },
                        "selectionRange": { "start": { "line": 3, "character": 0 }, "end": { "line": 3, "character": 20 } },
                        "children": [],
                    },
                    {
                        "name": "$t",
                        "kind": SymbolKind::Interface,
                        "range": { "start": { "line": 4, "character": 0 }, "end": { "line": 4, "character": 21 } },
                        "selectionRange": { "start": { "line": 4, "character": 7 }, "end": { "line": 4, "character": 9 } },
                        "children": [],
                    },
                    {
                        "name": "$f",
                        "kind": SymbolKind::Function,
                        "range": { "start": { "line": 5, "character": 0 }, "end": { "line": 5, "character": 9 } },
                        "selectionRange": { "start": { "line": 5, "character": 6 }, "end": { "line": 5, "character": 8 } },
                        "children": [],
                    },
                    {
                        "name": "<elem@7:1>",
                        "kind": SymbolKind::Field,
                        "range": { "start": { "line": 6, "character": 0 }, "end": { "line": 6, "character": 20 } },
                        "selectionRange": { "start": { "line": 6, "character": 0 }, "end": { "line": 6, "character": 20 } },
                    },
                ],
                "id": 1,
            }));
            testing::assert_exchange!(service, request, Ok(response));

            // send "shutdown" request
            testing::assert_status!(service, Ok(()));
            let request = &testing::lsp::shutdown::request();
            let response = Some(testing::lsp::shutdown::response());
            testing::assert_exchange!(service, request, Ok(response));

            // send "exit" notification
            testing::assert_status!(service, Ok(()));
            let notification = &testing::lsp::exit::notification();
            let status = None::<Value>;
            testing::assert_exchange!(service, notification, Ok(status));

            Ok(())
        }

        #[tokio::test]
        async fn wast() -> anyhow::Result<()> {
            let uri = Url::parse("inmemory:///test")?;
            let language_id = "wasm.wast";
            #[rustfmt::skip]
            let text = String::from(indoc::indoc! {r#"
                (module $m
                  (type $a (func))
                  (global $g i32 (i32.const 0))
                  (memory $m 1)
                  (data (i32.const 0))
                  (table $t 10 funcref)
                  (func $f)
                  (elem (i32.const 0)))
                (assert_return (invoke "empty"))
            "#});

            let (ref mut service, ref mut messages) = testing::service::spawn()?;

            // send "initialize" request
            testing::assert_status!(service, Ok(()));
            let request = &testing::lsp::initialize::request();
            let response = Some(testing::lsp::initialize::response());
            testing::assert_exchange!(service, request, Ok(response));

            // send "initialized" notification
            testing::assert_status!(service, Ok(()));
            let notification = &testing::lsp::initialized::notification();
            let status = None::<Value>;
            testing::assert_exchange!(service, notification, Ok(status));
            // ignore the "window/logMessage" notification: "WebAssembly language server initialized!"
            messages.next().await.unwrap();

            // send "textDocument/didOpen" notification for `uri`
            testing::assert_status!(service, Ok(()));
            let notification = &testing::lsp::text_document::did_open::notification(&uri, language_id, 1, text);
            let status = None::<Value>;
            testing::assert_exchange!(service, notification, Ok(status));

            // receive "textDocument/publishDiagnostics" notification for `uri`
            let message = messages.next().await.unwrap();
            let actual = serde_json::to_value(&message)?;
            let expected = testing::lsp::text_document::publish_diagnostics::notification(&uri, &[]);
            assert_eq!(actual, expected);

            // send "textDocument/documentSymbol" request for `uri`
            testing::assert_status!(service, Ok(()));
            let request = &testing::lsp::text_document::document_symbol::request(&uri);
            #[rustfmt::skip]
            let response = Some(json!({
                "jsonrpc": "2.0",
                "result": [
                    {
                        "name": "$m",
                        "kind": SymbolKind::Module,
                        "range": { "start": { "line": 0, "character": 0 }, "end": { "line": 7, "character": 23 } },
                        "selectionRange": { "start": { "line": 0, "character": 8 }, "end": { "line": 0, "character": 10 } },
                        "children": [
                            {
                                "name": "$a",
                                "kind": SymbolKind::TypeParameter,
                                "range": { "start": { "line": 1, "character": 2 }, "end": { "line": 1, "character": 18 } },
                                "selectionRange": { "start": { "line": 1, "character": 8 }, "end": { "line": 1, "character": 10 } },
                                "children": [],
                            },
                            {
                                "name": "$g",
                                "kind": SymbolKind::Event,
                                "range": { "start": { "line": 2, "character": 2 }, "end": { "line": 2, "character": 31 } },
                                "selectionRange": { "start": { "line": 2, "character": 10 }, "end": { "line": 2, "character": 12 } },
                                "children": [],
                            },
                            {
                                "name": "$m",
                                "kind": SymbolKind::Array,
                                "range": { "start": { "line": 3, "character": 2 }, "end": { "line": 3, "character": 15 } },
                                "selectionRange": { "start": { "line": 3, "character": 10 }, "end": { "line": 3, "character": 12 } },
                                "children": [],
                            },
                            {
                                "name": "<data@5:3>",
                                "kind": SymbolKind::Key,
                                "range": { "start": { "line": 4, "character": 2 }, "end": { "line": 4, "character": 22 } },
                                "selectionRange": { "start": { "line": 4, "character": 2 }, "end": { "line": 4, "character": 22 } },
                                "children": [],
                            },
                            {
                                "name": "$t",
                                "kind": SymbolKind::Interface,
                                "range": { "start": { "line": 5, "character": 2 }, "end": { "line": 5, "character": 23 } },
                                "selectionRange": { "start": { "line": 5, "character": 9 }, "end": { "line": 5, "character": 11 } },
                                "children": [],
                            },
                            {
                                "name": "$f",
                                "kind": SymbolKind::Function,
                                "range": { "start": { "line": 6, "character": 2 }, "end": { "line": 6, "character": 11 } },
                                "selectionRange": { "start": { "line": 6, "character": 8 }, "end": { "line": 6, "character": 10 } },
                                "children": [],
                            },
                            {
                                "name": "<elem@8:3>",
                                "kind": SymbolKind::Field,
                                "range": { "start": { "line": 7, "character": 2 }, "end": { "line": 7, "character": 22 } },
                                "selectionRange": { "start": { "line": 7, "character": 2 }, "end": { "line": 7, "character": 22 } },
                            },
                        ],
                    },
                ],
                "id": 1,
            }));
            testing::assert_exchange!(service, request, Ok(response));

            // send "shutdown" request
            testing::assert_status!(service, Ok(()));
            let request = &testing::lsp::shutdown::request();
            let response = Some(testing::lsp::shutdown::response());
            testing::assert_exchange!(service, request, Ok(response));

            // send "exit" notification
            testing::assert_status!(service, Ok(()));
            let notification = &testing::lsp::exit::notification();
            let status = None::<Value>;
            testing::assert_exchange!(service, notification, Ok(status));

            Ok(())
        }

        #[cfg(feature = "corpus")]
        mod corpus {
            use wasm_language_server_macros::corpus_tests;

            fn handler(corpus: &str, path: &str) {
                use futures::stream::StreamExt;
                use serde_json::Value;
                use std::{convert::TryFrom, io::Write};
                use tower_lsp::lsp_types::*;
                use wasm_language_server_parsers::core::language::Language;
                use wasm_language_server_testing as testing;

                async fn handler(corpus: &str, path: &str) -> anyhow::Result<()> {
                    let path = std::path::Path::new(path);

                    let mut mint = {
                        let file_dir = format!("tests/goldenfiles/lsp/textDocument/documentSymbol/corpus/{}", corpus);
                        goldenfile::Mint::new(file_dir)
                    };
                    let mut goldenfile = {
                        let path = path.with_extension("json");
                        let file_name = path.file_name().unwrap().to_str().unwrap();
                        mint.new_goldenfile(file_name)?
                    };

                    let uri = Url::from_file_path(path).unwrap();
                    let text = std::fs::read_to_string(path)?;
                    let language = Language::try_from(path)?;
                    let language_id = language.id();

                    let (ref mut service, ref mut messages) = testing::service::spawn()?;

                    // send "initialize" request
                    testing::assert_status!(service, Ok(()));
                    let request = &testing::lsp::initialize::request();
                    let response = Some(testing::lsp::initialize::response());
                    testing::assert_exchange!(service, request, Ok(response));

                    // send "initialized" notification
                    testing::assert_status!(service, Ok(()));
                    let notification = &testing::lsp::initialized::notification();
                    let status = None::<Value>;
                    testing::assert_exchange!(service, notification, Ok(status));
                    // ignore the "window/logMessage" notification: "WebAssembly language server initialized!"
                    messages.next().await.unwrap();

                    // send "textDocument/didOpen" notification for `uri`
                    testing::assert_status!(service, Ok(()));
                    let notification = &testing::lsp::text_document::did_open::notification(&uri, language_id, 1, text);
                    let status = None::<Value>;
                    testing::assert_exchange!(service, notification, Ok(status));

                    // receive "textDocument/publishDiagnostics" notification for `uri`
                    let message = messages.next().await.unwrap();
                    let actual = serde_json::to_value(&message)?;
                    let expected = testing::lsp::text_document::publish_diagnostics::notification(&uri, &[]);
                    assert_eq!(actual, expected);

                    // send "textDocument/documentSymbol" request for `uri`
                    testing::assert_status!(service, Ok(()));
                    let request = &testing::lsp::text_document::document_symbol::request(&uri);
                    let response = wasm_language_server_testing::service::send(service, request).await?;
                    // convert the response (wrapped in an outer `Option`) to a JSON value
                    let value = serde_json::to_value(response)?;
                    // write the value to the goldenfile
                    write!(goldenfile, "{}", value)?;

                    // send "shutdown" request
                    testing::assert_status!(service, Ok(()));
                    let request = &testing::lsp::shutdown::request();
                    let response = Some(testing::lsp::shutdown::response());
                    testing::assert_exchange!(service, request, Ok(response));

                    // send "exit" notification
                    testing::assert_status!(service, Ok(()));
                    let notification = &testing::lsp::exit::notification();
                    let status = None::<Value>;
                    testing::assert_exchange!(service, notification, Ok(status));

                    Ok(())
                }
                let mut runtime = tokio::runtime::Builder::new().basic_scheduler().build().unwrap();
                runtime.block_on(handler(corpus, path)).unwrap();
            }

            corpus_tests! {
                corpus: annotations,
                include: "vendor/corpus/vendor/WebAssembly/annotations/test/core/*.wast",
                handler: crate::lsp::text_document::document_symbol::corpus::handler,
            }

            corpus_tests! {
                corpus: bulk_memory_operations,
                include: "vendor/corpus/vendor/WebAssembly/bulk-memory-operations/test/core/*.wast",
                handler: crate::lsp::text_document::document_symbol::corpus::handler,
            }

            corpus_tests! {
                corpus: exception_handling,
                include: "vendor/corpus/vendor/WebAssembly/exception-handling/test/core/*.wast",
                handler: crate::lsp::text_document::document_symbol::corpus::handler,
            }

            corpus_tests! {
                corpus: function_references,
                include: "vendor/corpus/vendor/WebAssembly/function-references/test/core/*.wast",
                handler: crate::lsp::text_document::document_symbol::corpus::handler,
            }

            corpus_tests! {
                corpus: interface_types,
                include: "vendor/corpus/vendor/bytecodealliance/wasm-interface-types/tests/*.wat",
                exclude: [
                    // FIXME: fails because language id should be wasm.wast not wasm.wat
                    "bad-schema.wat",
                    // FIXME: fails because language id should be wasm.wast not wasm.wat
                    "bad-section.wat",
                    // NOTE: true positive; fails due to syntax error
                    "not-interface.wat",
                    // FIXME: fails because language id should be wasm.wast not wasm.wat
                    "two-sections.wat",
                ],
                handler: crate::lsp::text_document::document_symbol::corpus::handler,
            }

            corpus_tests! {
                corpus: multi_memory,
                include: "vendor/corpus/vendor/WebAssembly/multi-memory/test/core/*.wast",
                handler: crate::lsp::text_document::document_symbol::corpus::handler,
            }

            corpus_tests! {
                corpus: reference_types,
                include: "vendor/corpus/vendor/WebAssembly/reference-types/test/core/*.wast",
                handler: crate::lsp::text_document::document_symbol::corpus::handler,
            }

            corpus_tests! {
                corpus: simd,
                include: "vendor/corpus/vendor/WebAssembly/simd/test/core/**/*.wast",
                handler: crate::lsp::text_document::document_symbol::corpus::handler,
            }

            corpus_tests! {
                corpus: spec,
                include: "vendor/corpus/vendor/WebAssembly/spec/test/core/*.wast",
                handler: crate::lsp::text_document::document_symbol::corpus::handler,
            }

            corpus_tests! {
                corpus: threads,
                include: "vendor/corpus/vendor/WebAssembly/threads/test/core/*.wast",
                handler: crate::lsp::text_document::document_symbol::corpus::handler,
            }
        }
    }

    #[tokio::test]
    async fn hover() -> anyhow::Result<()> {
        let uri = Url::parse("inmemory:///test")?;
        let language_id = "wasm.wast";
        let text = String::from("(module $m (func $f (call_indirect (i32.const 0))))");

        let (ref mut service, ref mut messages) = testing::service::spawn()?;

        // send "initialize" request
        testing::assert_status!(service, Ok(()));
        let request = &testing::lsp::initialize::request();
        let response = Some(testing::lsp::initialize::response());
        testing::assert_exchange!(service, request, Ok(response));

        // send "initialized" notification
        testing::assert_status!(service, Ok(()));
        let notification = &testing::lsp::initialized::notification();
        let status = None::<Value>;
        testing::assert_exchange!(service, notification, Ok(status));
        // ignore the "window/logMessage" notification: "WebAssembly language server initialized!"
        messages.next().await.unwrap();

        // send "textDocument/didOpen" notification for `uri`
        testing::assert_status!(service, Ok(()));
        let notification = &testing::lsp::text_document::did_open::notification(&uri, language_id, 1, text);
        let status = None::<Value>;
        testing::assert_exchange!(service, notification, Ok(status));

        // receive "textDocument/publishDiagnostics" notification for `uri`
        let message = messages.next().await.unwrap();
        let actual = serde_json::to_value(&message)?;
        let expected = testing::lsp::text_document::publish_diagnostics::notification(&uri, &[]);
        assert_eq!(actual, expected);

        // send "textDocument/hover" request for `uri`
        testing::assert_status!(service, Ok(()));
        let request = &{
            let position = Position { line: 0, character: 15 };
            testing::lsp::text_document::hover::request(&uri, position)
        };
        let response = Some(json!({
            "jsonrpc": "2.0",
            "result": {
                "contents": [ "(func $f (call_indirect (i32.const 0)))" ],
                "range": { "start": { "line": 0, "character": 11 }, "end": { "line": 0, "character":  50 } },
            },
            "id": 1,
        }));
        testing::assert_exchange!(service, request, Ok(response));

        // send "textDocument/hover" request for `uri`
        testing::assert_status!(service, Ok(()));
        let request = &{
            let position = Position { line: 0, character: 23 };
            testing::lsp::text_document::hover::request(&uri, position)
        };
        let response = Some(json!({
            "jsonrpc": "2.0",
            "result": {
                "contents": [ "(call_indirect (i32.const 0))" ],
                "range": { "start": { "line": 0, "character": 20 }, "end": { "line": 0, "character":  49 } },
            },
            "id": 1,
        }));
        testing::assert_exchange!(service, request, Ok(response));

        // send "textDocument/hover" request for `uri`
        testing::assert_status!(service, Ok(()));
        let request = &{
            let position = Position { line: 0, character: 39 };
            testing::lsp::text_document::hover::request(&uri, position)
        };
        let response = Some(json!({
            "jsonrpc": "2.0",
            "result": {
                "contents": [ "i32.const 0" ],
                "range": { "start": { "line": 0, "character": 36 }, "end": { "line": 0, "character":  47 } },
            },
            "id": 1,
        }));
        testing::assert_exchange!(service, request, Ok(response));

        // send "textDocument/hover" request for `uri`
        testing::assert_status!(service, Ok(()));
        let request = &{
            let position = Position { line: 0, character: 50 };
            testing::lsp::text_document::hover::request(&uri, position)
        };
        let response = Some(json!({
            "jsonrpc": "2.0",
            "result": null,
            "id": 1,
        }));
        testing::assert_exchange!(service, request, Ok(response));

        // send "shutdown" request
        testing::assert_status!(service, Ok(()));
        let request = &testing::lsp::shutdown::request();
        let response = Some(testing::lsp::shutdown::response());
        testing::assert_exchange!(service, request, Ok(response));

        // send "exit" notification
        testing::assert_status!(service, Ok(()));
        let notification = &testing::lsp::exit::notification();
        let status = None::<Value>;
        testing::assert_exchange!(service, notification, Ok(status));

        Ok(())
    }

    #[cfg(feature = "corpus")]
    mod did_open {
        use wasm_language_server_macros::corpus_tests;

        fn handler(_corpus: &str, path: &str) {
            use futures::stream::StreamExt;
            use serde_json::Value;
            use std::convert::TryFrom;
            use tower_lsp::lsp_types::*;
            use wasm_language_server_parsers::core::language::Language;
            use wasm_language_server_testing as testing;

            async fn handler(path: &str) -> anyhow::Result<()> {
                let uri = Url::from_file_path(path).unwrap();
                let text = std::fs::read_to_string(path).unwrap();
                let language = Language::try_from(std::path::Path::new(path))?;
                let language_id = language.id();

                let (ref mut service, ref mut messages) = testing::service::spawn()?;

                // send "initialize" request
                testing::assert_status!(service, Ok(()));
                let request = &testing::lsp::initialize::request();
                let response = Some(testing::lsp::initialize::response());
                testing::assert_exchange!(service, request, Ok(response));

                // send "initialized" notification
                testing::assert_status!(service, Ok(()));
                let notification = &testing::lsp::initialized::notification();
                let status = None::<Value>;
                testing::assert_exchange!(service, notification, Ok(status));
                // ignore the "window/logMessage" notification: "WebAssembly language server initialized!"
                messages.next().await.unwrap();

                // send "textDocument/didOpen" notification for `uri`
                testing::assert_status!(service, Ok(()));
                let notification = &testing::lsp::text_document::did_open::notification(&uri, language_id, 1, text);
                let status = None::<Value>;
                testing::assert_exchange!(service, notification, Ok(status));

                // receive "textDocument/publishDiagnostics" notification for `uri`
                let message = messages.next().await.unwrap();
                let actual = serde_json::to_value(&message)?;
                let expected = testing::lsp::text_document::publish_diagnostics::notification(&uri, &[]);
                assert_eq!(actual, expected);

                // send "shutdown" request
                testing::assert_status!(service, Ok(()));
                let request = &testing::lsp::shutdown::request();
                let response = Some(testing::lsp::shutdown::response());
                testing::assert_exchange!(service, request, Ok(response));

                // send "exit" notification
                testing::assert_status!(service, Ok(()));
                let notification = &testing::lsp::exit::notification();
                let status = None::<Value>;
                testing::assert_exchange!(service, notification, Ok(status));

                Ok(())
            }
            let mut runtime = tokio::runtime::Builder::new().basic_scheduler().build().unwrap();
            runtime.block_on(handler(path)).unwrap();
        }

        corpus_tests! {
            corpus: annotations,
            include: "vendor/corpus/vendor/WebAssembly/annotations/test/core/*.wast",
            handler: crate::lsp::text_document::did_open::handler,
        }

        corpus_tests! {
            corpus: bulk_memory_operations,
            include: "vendor/corpus/vendor/WebAssembly/bulk-memory-operations/test/core/*.wast",
            handler: crate::lsp::text_document::did_open::handler,
        }

        corpus_tests! {
            corpus: exception_handling,
            include: "vendor/corpus/vendor/WebAssembly/exception-handling/test/core/*.wast",
            handler: crate::lsp::text_document::did_open::handler,
        }

        corpus_tests! {
            corpus: function_references,
            include: "vendor/corpus/vendor/WebAssembly/function-references/test/core/*.wast",
            handler: crate::lsp::text_document::did_open::handler,
        }

        corpus_tests! {
            corpus: interface_types,
            include: "vendor/corpus/vendor/bytecodealliance/wasm-interface-types/tests/*.wat",
            exclude: [
                // FIXME: fails because language id should be wasm.wast not wasm.wat
                "bad-schema.wat",
                // FIXME: fails because language id should be wasm.wast not wasm.wat
                "bad-section.wat",
                // NOTE: true positive; fails due to syntax error
                "not-interface.wat",
                // FIXME: fails because language id should be wasm.wast not wasm.wat
                "two-sections.wat",
            ],
            handler: crate::lsp::text_document::did_open::handler,
        }

        corpus_tests! {
            corpus: multi_memory,
            include: "vendor/corpus/vendor/WebAssembly/multi-memory/test/core/*.wast",
            handler: crate::lsp::text_document::did_open::handler,
        }

        corpus_tests! {
            corpus: reference_types,
            include: "vendor/corpus/vendor/WebAssembly/reference-types/test/core/*.wast",
            handler: crate::lsp::text_document::did_open::handler,
        }

        corpus_tests! {
            corpus: simd,
            include: "vendor/corpus/vendor/WebAssembly/simd/test/core/**/*.wast",
            handler: crate::lsp::text_document::did_open::handler,
        }

        corpus_tests! {
            corpus: spec,
            include: "vendor/corpus/vendor/WebAssembly/spec/test/core/*.wast",
            handler: crate::lsp::text_document::did_open::handler,
        }

        corpus_tests! {
            corpus: threads,
            include: "vendor/corpus/vendor/WebAssembly/threads/test/core/*.wast",
            handler: crate::lsp::text_document::did_open::handler,
        }
    }
}
