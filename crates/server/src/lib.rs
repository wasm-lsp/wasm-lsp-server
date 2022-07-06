//! The implementation of the WebAssembly language server.

#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]
#![allow(clippy::if_same_then_else)]
#![allow(clippy::needless_lifetimes)]

/// Core definitions for server functionality.
pub mod core;

/// LSP message handler functions.
pub mod handler;

/// Build-time metadata for the server.
pub mod metadata;

/// LSP feature provider functions.
pub mod provider;

use std::sync::Arc;
use tower_lsp::jsonrpc;

/// The WebAssembly language server structure.
pub struct Server {
    /// Reference to the LSP client.
    pub client: tower_lsp::Client,
    /// Reference to the LSP session.
    pub session: Arc<crate::core::Session>,
}

impl Server {
    /// Create a new [`Server`] instance.
    pub fn new(languages: crate::core::SessionLanguages, client: tower_lsp::Client) -> anyhow::Result<Self> {
        let session = Arc::new(crate::core::Session::new(languages, Some(client.clone()))?);
        Ok(Server { client, session })
    }

    /// Convenience function for building [`lsp::ServerCapabilities`] for [Server].
    pub fn capabilities() -> lsp::ServerCapabilities {
        let document_symbol_provider = Some(lsp::OneOf::Left(true));

        // let semantic_tokens_provider = {
        //     let token_types = vec![
        //         lsp::SemanticTokenType::COMMENT,
        //         lsp::SemanticTokenType::FUNCTION,
        //         lsp::SemanticTokenType::KEYWORD,
        //         lsp::SemanticTokenType::NAMESPACE,
        //         lsp::SemanticTokenType::OPERATOR,
        //         lsp::SemanticTokenType::PARAMETER,
        //         lsp::SemanticTokenType::STRING,
        //         lsp::SemanticTokenType::TYPE,
        //         lsp::SemanticTokenType::TYPE_PARAMETER,
        //         lsp::SemanticTokenType::VARIABLE,
        //     ];
        //     let token_modifiers = Default::default();

        //     let options = lsp::SemanticTokensOptions {
        //         legend: lsp::SemanticTokensLegend {
        //             token_types,
        //             token_modifiers,
        //         },
        //         range: Some(true),
        //         full: Some(lsp::SemanticTokensFullOptions::Bool(true)),
        //         ..Default::default()
        //     };
        //     Some(lsp::SemanticTokensServerCapabilities::SemanticTokensOptions(options))
        // };

        let text_document_sync = {
            let options = lsp::TextDocumentSyncOptions {
                open_close: Some(true),
                change: Some(lsp::TextDocumentSyncKind::INCREMENTAL),
                ..Default::default()
            };
            Some(lsp::TextDocumentSyncCapability::Options(options))
        };

        lsp::ServerCapabilities {
            text_document_sync,
            document_symbol_provider,
            // semantic_tokens_provider,
            ..Default::default()
        }
    }
}

#[tower_lsp::async_trait]
impl tower_lsp::LanguageServer for Server {
    async fn initialize(&self, params: lsp::InitializeParams) -> jsonrpc::Result<lsp::InitializeResult> {
        let session = self.session.clone();
        let result = crate::handler::initialize(session, params).await;
        Ok(result)
    }

    async fn initialized(&self, _: lsp::InitializedParams) {
        let typ = lsp::MessageType::INFO;
        let message = "WebAssembly language server initialized!";
        self.client.log_message(typ, message).await;
    }

    async fn shutdown(&self) -> jsonrpc::Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: lsp::DidOpenTextDocumentParams) {
        let session = self.session.clone();
        crate::handler::text_document::did_open(session, params).await.unwrap()
    }

    async fn did_change(&self, params: lsp::DidChangeTextDocumentParams) {
        let session = self.session.clone();
        crate::handler::text_document::did_change(session, params)
            .await
            .unwrap()
    }

    async fn did_close(&self, params: lsp::DidCloseTextDocumentParams) {
        let session = self.session.clone();
        crate::handler::text_document::did_close(session, params).await.unwrap()
    }

    async fn document_symbol(
        &self,
        params: lsp::DocumentSymbolParams,
    ) -> jsonrpc::Result<Option<lsp::DocumentSymbolResponse>> {
        let session = self.session.clone();
        let result = crate::handler::text_document::document_symbol(session, params).await;
        Ok(result.map_err(crate::core::IntoJsonRpcError)?)
    }

    async fn semantic_tokens_full(
        &self,
        params: lsp::SemanticTokensParams,
    ) -> jsonrpc::Result<Option<lsp::SemanticTokensResult>> {
        let session = self.session.clone();
        let result = crate::handler::text_document::semantic_tokens::full(session, params).await;
        Ok(result.map_err(crate::core::IntoJsonRpcError)?)
    }

    async fn semantic_tokens_range(
        &self,
        params: lsp::SemanticTokensRangeParams,
    ) -> jsonrpc::Result<Option<lsp::SemanticTokensRangeResult>> {
        let session = self.session.clone();
        let result = crate::handler::text_document::semantic_tokens::range(session, params).await;
        Ok(result.map_err(crate::core::IntoJsonRpcError)?)
    }
}
