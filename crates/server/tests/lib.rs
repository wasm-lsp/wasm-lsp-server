#[allow(unused_imports)]
#[macro_use]
extern crate wasm_language_server;

#[cfg(feature = "test")]
mod lsp {
    use serde_json::{json, Value};
    use std::task::Poll;
    use tower_lsp::{jsonrpc, lsp_types::*, ExitedError};
    use wasm_language_server::test;

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
                // FIXME
                exclude: [
                    "bulk.wast",
                    "data.wast",
                    "elem.wast",
                    "imports.wast",
                    "memory_init.wast",
                    "table_copy.wast",
                    "table_init.wast",
                ],
            }

            corpus_tests! {
                corpus: exception_handling,
                include: "vendor/corpus/vendor/WebAssembly/exception-handling/test/core/*.wast",
                // FIXME
                exclude: [
                    "br_table.wast",
                    "bulk.wast",
                    "call_indirect.wast",
                    "data.wast",
                    "elem.wast",
                    "global.wast",
                    "imports.wast",
                    "linking.wast",
                    "memory_copy.wast",
                    "memory_fill.wast",
                    "memory_init.wast",
                    "ref_func.wast",
                    "ref_is_null.wast",
                    "ref_null.wast",
                    "select.wast",
                    "table_copy.wast",
                    "table_fill.wast",
                    "table_get.wast",
                    "table_grow.wast",
                    "table_init.wast",
                    "table_set.wast",
                    "table_size.wast",
                    "table-sub.wast",
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
                // FIXME
                exclude: [
                    "simd_address.wast",
                    "simd_align.wast",
                    "simd_bit_shift.wast",
                    "simd_bitwise.wast",
                    "simd_boolean.wast",
                    "simd_const.wast",
                    "simd_conversions.wast",
                    "simd_f32x4.wast",
                    "simd_f32x4_arith.wast",
                    "simd_f32x4_cmp.wast",
                    "simd_f64x2.wast",
                    "simd_f64x2_arith.wast",
                    "simd_f64x2_cmp.wast",
                    "simd_i8x16_arith.wast",
                    "simd_i8x16_arith2.wast",
                    "simd_i8x16_cmp.wast",
                    "simd_i8x16_sat_arith.wast",
                    "simd_i16x8_arith.wast",
                    "simd_i16x8_arith2.wast",
                    "simd_i16x8_cmp.wast",
                    "simd_i16x8_sat_arith.wast",
                    "simd_i32x4_arith.wast",
                    "simd_i32x4_arith2.wast",
                    "simd_i32x4_cmp.wast",
                    "simd_i64x2_arith.wast",
                    "simd_lane.wast",
                    "simd_load.wast",
                    "simd_load_extend.wast",
                    "simd_load_splat.wast",
                    "simd_splat.wast",
                    "simd_store.wast",
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
