pub mod exit {
    use serde_json::{json, Value};

    pub fn notification() -> Value {
        json!({
            "jsonrpc": "2.0",
            "method": "exit",
        })
    }
}

pub mod initialize {
    use serde_json::{json, Value};

    pub fn request() -> Value {
        json!({
            "jsonrpc": "2.0",
            "method": "initialize",
            "params": {
                "capabilities":{},
            },
            "id": 1,
        })
    }

    pub fn response() -> Value {
        json!({
            "jsonrpc": "2.0",
            "result": {
                "capabilities": wasm_lsp_server::capabilities(),
            },
            "id": 1,
        })
    }
}

pub mod initialized {
    use serde_json::{json, Value};

    pub fn notification() -> Value {
        json!({
            "jsonrpc": "2.0",
            "method": "initialized",
            "params": {},
        })
    }
}

pub mod shutdown {
    use serde_json::{json, Value};

    pub fn request() -> Value {
        json!({
            "jsonrpc": "2.0",
            "method": "shutdown",
            "id": 1,
        })
    }

    pub fn response() -> Value {
        json!({
            "jsonrpc": "2.0",
            "result": null,
            "id": 1,
        })
    }
}

pub mod text_document {
    pub mod did_change {

        pub mod notification {
            use lspower::lsp::*;
            use serde_json::{json, Value};

            pub fn entire<S: AsRef<str>>(uri: &Url, text: S) -> Value {
                json!({
                    "jsonrpc": "2.0",
                    "method": "textDocument/didChange",
                    "params": {
                        "textDocument": {
                            "uri": uri,
                        },
                        "contentChanges": [
                            {
                                "text": text.as_ref(),
                            }
                        ],
                    },
                })
            }
        }
    }

    pub mod did_close {
        use lspower::lsp::*;
        use serde_json::{json, Value};

        pub fn notification(uri: &Url) -> Value {
            json!({
                "jsonrpc": "2.0",
                "method": "textDocument/didClose",
                "params": {
                    "textDocument": {
                        "uri": uri,
                    },
                },
            })
        }
    }

    pub mod did_open {
        use lspower::lsp::*;
        use serde_json::{json, Value};

        pub fn notification<S: AsRef<str>, T: AsRef<str>>(uri: &Url, language_id: S, version: i64, text: T) -> Value {
            json!({
                "jsonrpc": "2.0",
                "method": "textDocument/didOpen",
                "params": {
                    "textDocument": {
                        "uri": uri,
                        "languageId": language_id.as_ref(),
                        "version": version,
                        "text": text.as_ref(),
                    },
                },
            })
        }
    }

    pub mod document_symbol {
        use lspower::lsp::*;
        use serde_json::{json, Value};

        pub fn request(uri: &Url) -> Value {
            json!({
                "jsonrpc": "2.0",
                "method": "textDocument/documentSymbol",
                "params": {
                    "textDocument": {
                        "uri": uri,
                    },
                },
                "id": 1,
            })
        }

        pub fn response(response: DocumentSymbolResponse) -> Value {
            json!({
                "jsonrpc": "2.0",
                "result": response,
                "id": 1,
            })
        }
    }

    pub mod hover {
        use lspower::lsp::*;
        use serde_json::{json, Value};

        pub fn request(uri: &Url, position: Position) -> Value {
            json!({
                "jsonrpc": "2.0",
                "method": "textDocument/hover",
                "params": {
                    "textDocument": {
                        "uri": uri,
                    },
                    "position": position,
                },
                "id": 1,
            })
        }

        pub fn response() -> Value {
            json!({
                "jsonrpc": "2.0",
                "result": {
                },
                "id": 1,
            })
        }
    }

    pub mod publish_diagnostics {
        use lspower::lsp::*;
        use serde_json::{json, Value};

        pub fn notification(uri: &Url, diagnostics: &[Diagnostic]) -> Value {
            json!({
                "jsonrpc": "2.0",
                "method": "textDocument/publishDiagnostics",
                "params": {
                    "uri": uri,
                    "diagnostics": diagnostics,
                },
            })
        }
    }
}
