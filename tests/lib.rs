#[allow(unused_imports)]
#[macro_use]
extern crate wasm_language_server;

#[cfg(feature = "test")]
mod lsp {
    use serde_json::{json, Value};
    use std::task::Poll;
    use tower_lsp::{jsonrpc, lsp_types::*, ExitedError};
    use wasm_language_server::test;

    // FIXME: remove #[allow(dead_code)]
    #[allow(dead_code)]
    #[derive(Debug)]
    enum CorpusInclude {
        Glob(String),
        List(Vec<String>),
    }

    #[derive(Debug)]
    struct Corpus {
        pub ignore: &'static [&'static str],
        pub include: CorpusInclude,
    }

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

    #[allow(non_snake_case)]
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
        use super::{Corpus, CorpusInclude};
        use glob::glob;
        use serde_json::{json, Value};
        use std::task::Poll;
        use tower_lsp::lsp_types::*;
        use wasm_language_server::test;

        #[tokio::test]
        async fn did_open() -> anyhow::Result<()> {
            use futures::stream::StreamExt;

            let tests = vec![
                // FIXME: annotation parsing is currently broken; uncomment once we fix
                // {
                //     let dir = "vendor/corpus/vendor/WebAssembly/annotations/test/core";
                //     Corpus {
                //         ignore: &[],
                //         include: CorpusInclude::List(vec![format!("{}/annotations.wast", dir)]),
                //     }
                // },
                {
                    let dir = "vendor/corpus/vendor/WebAssembly/spec/test/core";
                    Corpus {
                        ignore: &["comments.wast"],
                        include: CorpusInclude::Glob(format!("{}/*.wast", dir)),
                    }
                },
            ];

            for corpus in tests {
                let entries = match corpus.include {
                    CorpusInclude::Glob(pattern) => glob(&pattern)?.map(|path| path.unwrap()).collect::<Vec<_>>(),
                    CorpusInclude::List(entries) => entries.iter().map(std::path::PathBuf::from).collect::<Vec<_>>(),
                };
                for entry in entries {
                    let path = entry.canonicalize()?;
                    let file_name = path.file_name().unwrap().to_str().unwrap();
                    if !corpus.ignore.contains(&file_name) {
                        let extension = path.extension().and_then(std::ffi::OsStr::to_str);
                        if let Some("wast") = extension {
                            let uri = Url::from_file_path(&path).unwrap();
                            let text = std::fs::read_to_string(path)?;

                            let (mut service, mut messages) = test::service::spawn()?;
                            let service = &mut service;

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
                            let actual = serde_json::from_str::<Value>(&message)?;
                            let expected = json!({
                                "jsonrpc": "2.0",
                                "method": "textDocument/publishDiagnostics",
                                "params": {
                                    "uri": uri,
                                    "diagnostics": [],
                                },
                            });
                            assert_eq!(actual, expected);
                        }
                    }
                }
            }

            Ok(())
        }
    }
}
