use serde_json::Value;

#[futures_test::test]
async fn exit() -> anyhow::Result<()> {
    let service = &mut testing::service::spawn().await?.0;

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
    match service.poll_ready() {
        std::task::Poll::Ready(Err(err)) => {
            assert_eq!(err.to_string(), "language server has exited");
        },
        _ => {
            panic!()
        },
    }
    let notification = &{
        let uri = lsp::Url::parse("inmemory::///test")?;
        let language_id = "wasm.wat";
        let text = String::from("");
        testing::lsp::text_document::did_open::notification(&uri, language_id, 1, text)
    };
    match testing::service::send(service, notification).await {
        Err(err) => {
            assert_eq!(err.to_string(), "language server has exited");
        },
        _ => {
            panic!()
        },
    }

    Ok(())
}

#[futures_test::test]
async fn initialize() -> anyhow::Result<()> {
    let service = &mut testing::service::spawn().await?.0;

    // send "initialize" request
    testing::assert_status!(service, Ok(()));
    let request = &testing::lsp::initialize::request();
    let response = Some(testing::lsp::initialize::response());
    testing::assert_exchange!(service, request, Ok(response));

    Ok(())
}

#[futures_test::test]
async fn initialized() -> anyhow::Result<()> {
    let service = &mut testing::service::spawn().await?.0;

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

#[futures_test::test]
async fn initialize_once() -> anyhow::Result<()> {
    let service = &mut testing::service::spawn().await?.0;

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

#[futures_test::test]
async fn shutdown() -> anyhow::Result<()> {
    let service = &mut testing::service::spawn().await?.0;

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

    #[futures_test::test]
    async fn did_close() -> anyhow::Result<()> {
        let uri = lsp::Url::parse("inmemory:///test")?;
        let language_id = "wasm.wast";
        let text = String::from("(module)");

        let (ref mut service, ref mut messages) = testing::service::spawn().await?;

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

    #[futures_test::test]
    async fn did_open() -> anyhow::Result<()> {
        let uri = lsp::Url::parse("inmemory:///test")?;
        let language_id = "wasm.wast";
        let text = String::from("(module)");

        let (ref mut service, ref mut messages) = testing::service::spawn().await?;

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

        #[futures_test::test]
        async fn wat() -> anyhow::Result<()> {
            let uri = lsp::Url::parse("inmemory:///test")?;
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

            let (ref mut service, ref mut messages) = testing::service::spawn().await?;

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
                        "kind": lsp::SymbolKind::TYPE_PARAMETER,
                        "range": { "start": { "line": 0, "character": 0 }, "end": { "line": 0, "character": 16 } },
                        "selectionRange": { "start": { "line": 0, "character": 6 }, "end": { "line": 0, "character": 8 } },
                        "children": [],
                    },
                    {
                        "name": "$g",
                        "kind": lsp::SymbolKind::EVENT,
                        "range": { "start": { "line": 1, "character": 0 }, "end": { "line": 1, "character": 29 } },
                        "selectionRange": { "start": { "line": 1, "character": 8 }, "end": { "line": 1, "character": 10 } },
                        "children": [],
                    },
                    {
                        "name": "$m",
                        "kind": lsp::SymbolKind::ARRAY,
                        "range": { "start": { "line": 2, "character": 0 }, "end": { "line": 2, "character": 13 } },
                        "selectionRange": { "start": { "line": 2, "character": 8 }, "end": { "line": 2, "character": 10 } },
                        "children": [],
                    },
                    {
                        "name": "<data@4:1>",
                        "kind": lsp::SymbolKind::KEY,
                        "range": { "start": { "line": 3, "character": 0 }, "end": { "line": 3, "character": 20 } },
                        "selectionRange": { "start": { "line": 3, "character": 0 }, "end": { "line": 3, "character": 20 } },
                        "children": [],
                    },
                    {
                        "name": "$t",
                        "kind": lsp::SymbolKind::INTERFACE,
                        "range": { "start": { "line": 4, "character": 0 }, "end": { "line": 4, "character": 21 } },
                        "selectionRange": { "start": { "line": 4, "character": 7 }, "end": { "line": 4, "character": 9 } },
                        "children": [],
                    },
                    {
                        "name": "$f",
                        "kind": lsp::SymbolKind::FUNCTION,
                        "range": { "start": { "line": 5, "character": 0 }, "end": { "line": 5, "character": 9 } },
                        "selectionRange": { "start": { "line": 5, "character": 6 }, "end": { "line": 5, "character": 8 } },
                        "children": [],
                    },
                    {
                        "name": "<elem@7:1>",
                        "kind": lsp::SymbolKind::FIELD,
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

        #[futures_test::test]
        async fn wast() -> anyhow::Result<()> {
            let uri = lsp::Url::parse("inmemory:///test")?;
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

            let (ref mut service, ref mut messages) = testing::service::spawn().await?;

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
                        "kind": lsp::SymbolKind::MODULE,
                        "range": { "start": { "line": 0, "character": 0 }, "end": { "line": 7, "character": 23 } },
                        "selectionRange": { "start": { "line": 0, "character": 8 }, "end": { "line": 0, "character": 10 } },
                        "children": [
                            {
                                "name": "$a",
                                "kind": lsp::SymbolKind::TYPE_PARAMETER,
                                "range": { "start": { "line": 1, "character": 2 }, "end": { "line": 1, "character": 18 } },
                                "selectionRange": { "start": { "line": 1, "character": 8 }, "end": { "line": 1, "character": 10 } },
                                "children": [],
                            },
                            {
                                "name": "$g",
                                "kind": lsp::SymbolKind::EVENT,
                                "range": { "start": { "line": 2, "character": 2 }, "end": { "line": 2, "character": 31 } },
                                "selectionRange": { "start": { "line": 2, "character": 10 }, "end": { "line": 2, "character": 12 } },
                                "children": [],
                            },
                            {
                                "name": "$m",
                                "kind": lsp::SymbolKind::ARRAY,
                                "range": { "start": { "line": 3, "character": 2 }, "end": { "line": 3, "character": 15 } },
                                "selectionRange": { "start": { "line": 3, "character": 10 }, "end": { "line": 3, "character": 12 } },
                                "children": [],
                            },
                            {
                                "name": "<data@5:3>",
                                "kind": lsp::SymbolKind::KEY,
                                "range": { "start": { "line": 4, "character": 2 }, "end": { "line": 4, "character": 22 } },
                                "selectionRange": { "start": { "line": 4, "character": 2 }, "end": { "line": 4, "character": 22 } },
                                "children": [],
                            },
                            {
                                "name": "$t",
                                "kind": lsp::SymbolKind::INTERFACE,
                                "range": { "start": { "line": 5, "character": 2 }, "end": { "line": 5, "character": 23 } },
                                "selectionRange": { "start": { "line": 5, "character": 9 }, "end": { "line": 5, "character": 11 } },
                                "children": [],
                            },
                            {
                                "name": "$f",
                                "kind": lsp::SymbolKind::FUNCTION,
                                "range": { "start": { "line": 6, "character": 2 }, "end": { "line": 6, "character": 11 } },
                                "selectionRange": { "start": { "line": 6, "character": 8 }, "end": { "line": 6, "character": 10 } },
                                "children": [],
                            },
                            {
                                "name": "<elem@8:3>",
                                "kind": lsp::SymbolKind::FIELD,
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

        #[cfg(feature = "goldenfiles")]
        mod corpus {
            use wasm_language_server_macros::corpus_tests;

            fn handler(corpus: &str, path: &str) {
                use futures::stream::StreamExt;
                use serde_json::Value;
                use std::io::Write;
                use wasm_lsp_syntax::core::language::Language;

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

                    let uri = lsp::Url::from_file_path(path).unwrap();
                    let text = std::fs::read_to_string(path)?;
                    let language = Language::try_from(path)?;
                    let language_id = language.id();

                    let (ref mut service, ref mut messages) = testing::service::spawn().await?;

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
                futures::executor::block_on(handler(corpus, path)).unwrap();
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
                    // NOTE: fails because language id should be wasm.wast not wasm.wat
                    "bad-schema.wat",
                    // NOTE: fails because language id should be wasm.wast not wasm.wat
                    "bad-section.wat",
                    // NOTE: true positive; fails due to syntax error
                    "not-interface.wat",
                    // NOTE: fails because language id should be wasm.wast not wasm.wat
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
                include: "vendor/corpus/vendor/WebAssembly/spec/test/core/address.wast",
                handler: crate::lsp::text_document::document_symbol::corpus::handler,
            }

            corpus_tests! {
                corpus: threads,
                include: "vendor/corpus/vendor/WebAssembly/threads/test/core/*.wast",
                handler: crate::lsp::text_document::document_symbol::corpus::handler,
            }
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    mod did_open {
        use wasm_lsp_macros::corpus_tests;

        fn handler(_corpus: &str, path: &str) {
            use futures::stream::StreamExt;
            use serde_json::Value;
            use wasm_lsp_server::core::Language;

            async fn handler(path: &str) -> anyhow::Result<()> {
                let uri = lsp::Url::from_file_path(path).unwrap();
                let text = std::fs::read_to_string(path).unwrap();
                let language = Language::try_from(std::path::Path::new(path))?;
                let language_id = language.id();

                let (ref mut service, ref mut messages) = testing::service::spawn().await?;

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
            futures::executor::block_on(handler(path)).unwrap();
        }

        corpus_tests! {
            corpus: annotations,
            include: "vendor/corpus/vendor/WebAssembly/annotations/test/core/*.wast",
            exclude: [
                "annotations.wast",
                "br.wast",
                "comments.wast",
                "local_set.wast",
            ],
            handler: crate::lsp::text_document::did_open::handler,
        }

        corpus_tests! {
            corpus: bulk_memory_operations,
            include: "vendor/corpus/vendor/WebAssembly/bulk-memory-operations/test/core/*.wast",
            exclude: [
                "br.wast",
                "comments.wast",
                "local_set.wast",
            ],
            handler: crate::lsp::text_document::did_open::handler,
        }

        corpus_tests! {
            corpus: exception_handling,
            include: "vendor/corpus/vendor/WebAssembly/exception-handling/test/core/*.wast",
            exclude: [
                "br.wast",
                "comments.wast",
                "local_set.wast",
            ],
            handler: crate::lsp::text_document::did_open::handler,
        }

        corpus_tests! {
            corpus: function_references,
            include: "vendor/corpus/vendor/WebAssembly/function-references/test/core/*.wast",
            exclude: [
                "br.wast",
                "comments.wast",
                "let.wast",
                "local_set.wast",
            ],
            handler: crate::lsp::text_document::did_open::handler,
        }

        corpus_tests! {
            corpus: interface_types,
            include: "vendor/corpus/vendor/bytecodealliance/wasm-interface-types/tests/*.wat",
            exclude: [
                // NOTE: fails because language id should be wasm.wast not wasm.wat
                "bad-schema.wat",
                // NOTE: fails because language id should be wasm.wast not wasm.wat
                "bad-section.wat",
                // NOTE: true positive; fails due to syntax error
                "not-interface.wat",
                // NOTE: fails because language id should be wasm.wast not wasm.wat
                "two-sections.wat",
            ],
            handler: crate::lsp::text_document::did_open::handler,
        }

        corpus_tests! {
            corpus: multi_memory,
            include: "vendor/corpus/vendor/WebAssembly/multi-memory/test/core/*.wast",
            exclude: [
                "br.wast",
                "comments.wast",
                "local_set.wast",
                "memory_grow.wast",
            ],
            handler: crate::lsp::text_document::did_open::handler,
        }

        corpus_tests! {
            corpus: reference_types,
            include: "vendor/corpus/vendor/WebAssembly/reference-types/test/core/*.wast",
            exclude: [
                "br.wast",
                "comments.wast",
                "local_set.wast",
            ],
            handler: crate::lsp::text_document::did_open::handler,
        }

        corpus_tests! {
            corpus: simd,
            include: "vendor/corpus/vendor/WebAssembly/simd/test/core/**/*.wast",
            exclude: [
                "br.wast",
                "comments.wast",
                "local_set.wast",
            ],
            handler: crate::lsp::text_document::did_open::handler,
        }

        corpus_tests! {
            corpus: spec,
            include: "vendor/corpus/vendor/WebAssembly/spec/test/core/*.wast",
            exclude: [
                "br.wast",
                "comments.wast",
                "local_set.wast",
            ],
            handler: crate::lsp::text_document::did_open::handler,
        }

        corpus_tests! {
            corpus: threads,
            include: "vendor/corpus/vendor/WebAssembly/threads/test/core/*.wast",
            exclude: [
                "comments.wast",
                "local_set.wast",
            ],
            handler: crate::lsp::text_document::did_open::handler,
        }
    }
}
