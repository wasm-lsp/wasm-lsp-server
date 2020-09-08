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
    use serde_json::Value;
    use tower_lsp::lsp_types::*;
    use wasm_language_server_testing as testing;

    #[tokio::test]
    async fn did_close() -> anyhow::Result<()> {
        let uri = Url::parse("inmemory:///test")?;
        let language_id = "wasm.wast";
        let text = String::new();

        let (mut service, mut messages) = testing::service::spawn()?;
        let service = &mut service;

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

        // send "textDocument/didOpen" notification for `uri`
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

        let (mut service, mut messages) = testing::service::spawn()?;
        let service = &mut service;

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
