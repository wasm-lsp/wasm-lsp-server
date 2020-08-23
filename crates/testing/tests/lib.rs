#[allow(unused_imports)]
#[macro_use]
extern crate wasm_language_server_testing;

#[cfg(feature = "test")]
mod lsp {
    use serde_json::{json, Value};
    use std::task::Poll;
    use tower_lsp::{jsonrpc, lsp_types::*, ExitedError};
    use wasm_language_server_testing::test;

    #[tokio::test]
    async fn initialize_once() -> anyhow::Result<()> {
        let service = &mut test::service::spawn()?.0;

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
        let service = &mut test::service::spawn()?.0;

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
    async fn exit() -> anyhow::Result<()> {
        let service = &mut test::service::spawn()?.0;

        assert_ready!(service, Ok(()));
        let request = &json!({ "jsonrpc": "2.0", "method": "initialized", "params": {} });
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

        Ok(())
    }

    mod text_document {
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
}
