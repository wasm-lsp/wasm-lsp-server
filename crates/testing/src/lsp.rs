//! LSP-related definitions.

/// Definitions for constructing `exit` messages.
pub mod exit {
    use serde_json::{json, Value};

    /// Construct an `exit` notification.
    pub fn notification() -> Value {
        json!({
            "jsonrpc": "2.0",
            "method": "exit",
        })
    }
}

/// Definitions for constructing `initialize` messages.
pub mod initialize {
    use serde_json::{json, Value};
    use wasm_language_server as server;

    /// Construct an `initialize` request.
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

    /// Construct an `initialize` response.
    pub fn response() -> Value {
        json!({
            "jsonrpc": "2.0",
            "result": {
                "capabilities": server::lsp::server::capabilities(),
            },
            "id": 1,
        })
    }
}

/// Definitions for constructing `initialized` messages.
pub mod initialized {
    use serde_json::{json, Value};

    /// Construct an `initialized` notification.
    pub fn notification() -> Value {
        json!({
            "jsonrpc": "2.0",
            "method": "initialized",
            "params": {},
        })
    }
}

/// Definitions for constructing `shutdown` messages.
pub mod shutdown {
    use serde_json::{json, Value};

    /// Construct an `shutdown` request.
    pub fn request() -> Value {
        json!({
            "jsonrpc": "2.0",
            "method": "shutdown",
            "id": 1,
        })
    }

    /// Construct an `shutdown` response.
    pub fn response() -> Value {
        json!({
            "jsonrpc": "2.0",
            "result": null,
            "id": 1,
        })
    }
}

/// Definitions for constructing `textDocument/*` messages.
pub mod text_document {
    /// Definitions for constructing `textDocument/didChange` messages.
    pub mod did_change {

        /// Definitions for constructing a `textDocument/didChange` notification.
        pub mod notification {
            use serde_json::{json, Value};
            use lspower::lsp_types::*;

            /// Construct a `textDocument/didChange` notification with entire new text.
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

    /// Definitions for constructing `textDocument/didClose` messages.
    pub mod did_close {
        use serde_json::{json, Value};
        use lspower::lsp_types::*;

        /// Construct a `textDocument/didClose` notification.
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

    /// Definitions for constructing `textDocument/didOpen` messages.
    pub mod did_open {
        use serde_json::{json, Value};
        use lspower::lsp_types::*;

        /// Construct a `textDocument/didOpen` notification.
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

    /// Definitions for constructing `textDocument/documentSymbol` messages.
    pub mod document_symbol {
        use serde_json::{json, Value};
        use lspower::lsp_types::*;

        /// Construct a `textDocument/documentSymbol` request.
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

        /// Construct a `textDocument/documentSymbol` request.
        pub fn response(response: DocumentSymbolResponse) -> Value {
            json!({
                "jsonrpc": "2.0",
                "result": response,
                "id": 1,
            })
        }
    }

    /// Definitions for constructing `textDocument/hover` messages.
    pub mod hover {
        use serde_json::{json, Value};
        use lspower::lsp_types::*;

        /// Construct a `textDocument/hover` request.
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

        /// Construct a `textDocument/hover` response.
        pub fn response() -> Value {
            json!({
                "jsonrpc": "2.0",
                "result": {
                },
                "id": 1,
            })
        }
    }

    /// Definitions for constructing `textDocument/publishDiagnostics` messages.
    pub mod publish_diagnostics {
        use serde_json::{json, Value};
        use lspower::lsp_types::*;

        /// Construct a `textDocument/publishDiagnostics` notification.
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
