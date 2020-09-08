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
        let language_id = "wasm.wast";
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
        let language_id = "wasm.wast";

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

    #[tokio::test]
    async fn document_symbol() -> anyhow::Result<()> {
        let uri = Url::parse("inmemory:///test")?;
        let language_id = "wasm.wat";
        #[rustfmt::skip]
        let text = String::from(r#"
            (module $m
              (type $a (func))
              (global $g i32 (i32.const 0))
              (memory $m 1)
              (data (i32.const 0))
              (table $t 10 funcref)
              (func $f)
              (elem (i32.const 0)))
        "#);

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
                    "range": { "start": { "line": 1, "character": 12 }, "end": { "line": 8, "character": 35 } },
                    "selectionRange": { "start": { "line": 1, "character": 20 }, "end": { "line": 1, "character": 22 } },
                    "children": [
                        {
                            "name": "$a",
                            "kind": SymbolKind::TypeParameter,
                            "range": { "start": { "line": 2, "character": 14 }, "end": { "line": 2, "character": 30 } },
                            "selectionRange": { "start": { "line": 2, "character": 20 }, "end": { "line": 2, "character": 22 } },
                            "children": [],
                        },
                        {
                            "name": "$g",
                            "kind": SymbolKind::Event,
                            "range": { "start": { "line": 3, "character": 14 }, "end": { "line": 3, "character": 43 } },
                            "selectionRange": { "start": { "line": 3, "character": 22 }, "end": { "line": 3, "character": 24 } },
                            "children": [],
                        },
                        {
                            "name": "$m",
                            "kind": SymbolKind::Array,
                            "range": { "start": { "line": 4, "character": 14 }, "end": { "line": 4, "character": 27 } },
                            "selectionRange": { "start": { "line": 4, "character": 22 }, "end": { "line": 4, "character": 24 } },
                            "children": [],
                        },
                        {
                            "name": "<data@6:15>",
                            "kind": SymbolKind::Key,
                            "range": { "start": { "line": 5, "character": 14 }, "end": { "line": 5, "character": 34 } },
                            "selectionRange": { "start": { "line": 5, "character": 14 }, "end": { "line": 5, "character": 34 } },
                            "children": [],
                        },
                        {
                            "name": "$t",
                            "kind": SymbolKind::Interface,
                            "range": { "start": { "line": 6, "character": 14 }, "end": { "line": 6, "character": 35 } },
                            "selectionRange": { "start": { "line": 6, "character": 21 }, "end": { "line": 6, "character": 23 } },
                            "children": [],
                        },
                        {
                            "name": "$f",
                            "kind": SymbolKind::Function,
                            "range": { "start": { "line": 7, "character": 14 }, "end": { "line": 7, "character": 23 } },
                            "selectionRange": { "start": { "line": 7, "character": 20 }, "end": { "line": 7, "character": 22 } },
                            "children": [],
                        },
                        {
                            "name": "<elem@9:15>",
                            "kind": SymbolKind::Field,
                            "range": { "start": { "line": 8, "character": 14 }, "end": { "line": 8, "character": 34 } },
                            "selectionRange": { "start": { "line": 8, "character": 14 }, "end": { "line": 8, "character": 34 } },
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

    #[tokio::test]
    async fn hover() -> anyhow::Result<()> {
        let uri = Url::parse("inmemory:///test")?;
        let language_id = "wasm.wast";
        let text = String::from("(module $m (func $f))");

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
        #[rustfmt::skip]
        let response = Some(json!({
            "jsonrpc": "2.0",
            "result": {
                "contents": [ "(func $f)" ],
                "range": { "start": { "line": 0, "character": 11 }, "end": { "line": 0, "character":  20 } },
            },
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

        corpus_tests! {
            corpus: annotations,
            include: "vendor/corpus/vendor/WebAssembly/annotations/test/core/*.wast",
            exclude: [
            ],
        }

        corpus_tests! {
            corpus: bulk_memory_operations,
            include: "vendor/corpus/vendor/WebAssembly/bulk-memory-operations/test/core/*.wast",
            exclude: [
            ],
        }

        corpus_tests! {
            corpus: exception_handling,
            include: "vendor/corpus/vendor/WebAssembly/exception-handling/test/core/*.wast",
            exclude: [
            ],
        }

        corpus_tests! {
            corpus: function_references,
            include: "vendor/corpus/vendor/WebAssembly/function-references/test/core/*.wast",
            exclude: [
            ],
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
        }

        corpus_tests! {
            corpus: multi_memory,
            include: "vendor/corpus/vendor/WebAssembly/multi-memory/test/core/*.wast",
            exclude: [
            ],
        }

        corpus_tests! {
            corpus: reference_types,
            include: "vendor/corpus/vendor/WebAssembly/reference-types/test/core/*.wast",
            exclude: [
            ],
        }

        corpus_tests! {
            corpus: simd,
            include: "vendor/corpus/vendor/WebAssembly/simd/test/core/**/*.wast",
            exclude: [
            ],
        }

        corpus_tests! {
            corpus: spec,
            include: "vendor/corpus/vendor/WebAssembly/spec/test/core/*.wast",
            exclude: [
            ],
        }

        corpus_tests! {
            corpus: threads,
            include: "vendor/corpus/vendor/WebAssembly/threads/test/core/*.wast",
            exclude: [
            ],
        }
    }
}
