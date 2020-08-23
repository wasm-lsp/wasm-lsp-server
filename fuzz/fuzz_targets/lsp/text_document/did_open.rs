#![allow(non_snake_case)]
#![no_main]

#[cfg(feature = "fuzz")]
#[macro_use]
extern crate wasm_language_server_testing;

#[cfg(feature = "fuzz")]
#[doc(hidden)]
pub mod test {
    use futures::stream::StreamExt;
    use libfuzzer_sys::fuzz_target;
    use serde_json::{json, Value};
    use std::task::Poll;
    use tokio::runtime::Runtime;
    use tower_lsp::lsp_types::*;
    use wasm_language_server_testing::test;
    use wasm_smith::Module;

    fuzz_target!(|module: Module| {
        let future = async {
            let wasm = module.to_bytes();
            let text = wabt::wasm2wat_with_features(wasm, {
                let mut features = wabt::Features::new();
                features.enable_all();
                features
            })
            .unwrap();

            println!("{}", text);

            let (mut service, mut messages) = test::service::spawn().unwrap();
            let service = &mut service;

            let uri = "inmemory:///test";

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

            assert_ready!(service, Ok(()));
            let request = &json!({
                "jsonrpc": "2.0",
                "method": "textDocument/didOpen",
                "params": {
                    "textDocument": {
                        "uri": uri,
                        "languageId": "wasm.wast",
                        "version": 1,
                        "text": text,
                    },
                },
            });
            let response = None::<Value>;
            assert_exchange!(service, request, Ok(response));

            let message = messages.next().await.unwrap();
            let actual = serde_json::to_value(&message).unwrap();
            let expected = json!({
                "jsonrpc": "2.0",
                "method": "textDocument/publishDiagnostics",
                "params": {
                    "uri": uri,
                    "diagnostics": [],
                },
            });
            assert_eq!(actual, expected);
        };
        let mut runtime = Runtime::new().unwrap();
        runtime.block_on(future);
    });
}
