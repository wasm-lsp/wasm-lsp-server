use crate::{auditor, database::Database, document::Document, elaborator, parser, server};
use dashmap::DashMap;
use failure::Fallible;
use jsonrpc_core::Result;
use log;
use lsp_types::*;
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_lsp::{Client, LanguageServer};

/// Represents the current state of the LSP service.
pub struct Session {
    database: Database,
    documents: Arc<DashMap<Url, Document>>,
}

impl Session {
    pub fn new() -> Fallible<Self> {
        let database = Database::new()?;
        let documents = Arc::new(DashMap::new());
        Ok(Session { database, documents })
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Session {
    fn initialize(&self, _: &Client, params: InitializeParams) -> Result<InitializeResult> {
        log::info!("{:?}", params);
        let document_symbol_provider = Some(true);
        // let semantic_tokens_provider =
        // Some(SemanticTokensServerCapabilities::SemanticTokensRegistrationOptions(
        //     SemanticTokensRegistrationOptions {
        //         semantic_tokens_options: SemanticTokensOptions {
        //             document_provider: Some(SemanticTokensDocumentProvider::Bool(false)),
        //             legend: SemanticTokensLegend {
        //                 token_modifiers: vec![],
        //                 token_types: vec![],
        //             },
        //             range_provider: Some(false),
        //             work_done_progress_options: WorkDoneProgressOptions {
        //                 work_done_progress: Some(false),
        //             },
        //         },
        //         static_registration_options: StaticRegistrationOptions { id: None },
        //         text_document_registration_options: TextDocumentRegistrationOptions {
        //             document_selector: None,
        //         },
        //     },
        // ));
        let text_document_sync = Some(TextDocumentSyncCapability::Options(TextDocumentSyncOptions {
            open_close: Some(true),
            change: Some(TextDocumentSyncKind::Full),
            ..TextDocumentSyncOptions::default()
        }));
        // let workspace = Some(WorkspaceCapability {
        //     workspace_folders: Some(WorkspaceFolderCapability {
        //         supported: Some(false),
        //         change_notifications: Some(WorkspaceFolderCapabilityChangeNotifications::Bool(false)),
        //     }),
        // });
        // let workspace_symbol_provider = Some(false);
        let capabilities = ServerCapabilities {
            document_symbol_provider,
            // semantic_tokens_provider,
            text_document_sync,
            // workspace,
            // workspace_symbol_provider,
            ..ServerCapabilities::default()
        };
        Ok(InitializeResult {
            capabilities,
            ..InitializeResult::default()
        })
    }

    async fn initialized(&self, client: &Client, _: InitializedParams) {
        client.log_message(MessageType::Info, "server initialized!");
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, client: &Client, params: DidOpenTextDocumentParams) {
        let documents = self.documents.clone();
        // FIXME: remove on next tower-lsp release
        let client: &'static Client = unsafe { std::mem::transmute(client) };
        server::tasks_did_open(documents, client, params).await.unwrap()
    }

    async fn did_change(&self, client: &Client, params: DidChangeTextDocumentParams) {
        let documents = self.documents.clone();
        // FIXME: remove on next tower-lsp release
        let client: &'static Client = unsafe { std::mem::transmute(client) };
        server::tasks_did_change(documents, client, params).await.unwrap()
    }

    async fn did_close(&self, client: &Client, params: DidCloseTextDocumentParams) {
        let documents = self.documents.clone();
        // FIXME: remove on next tower-lsp release
        let client: &'static Client = unsafe { std::mem::transmute(client) };
        server::tasks_did_close(documents, client, params).await.unwrap()
    }

    async fn document_symbol(&self, params: DocumentSymbolParams) -> Result<Option<DocumentSymbolResponse>> {
        let documents = self.documents.clone();
        let task = elaborator::document_symbol(documents, params);
        tokio::spawn(task).await.unwrap()
    }
}

async fn tasks_did_open(
    documents: Arc<DashMap<Url, Document>>,
    client: &'static Client,
    params: DidOpenTextDocumentParams,
) -> Fallible<()> {
    let DidOpenTextDocumentParams {
        text_document: TextDocumentItem { uri, text, .. },
    } = params;
    let params = {
        let text_document = {
            let uri = uri;
            let version = None;
            VersionedTextDocumentIdentifier { uri, version }
        };
        let content_changes = vec![{
            let range = None;
            let range_length = None;
            TextDocumentContentChangeEvent {
                range,
                range_length,
                text,
            }
        }];
        DidChangeTextDocumentParams {
            text_document,
            content_changes,
        }
    };
    self::tasks_did_change(documents, client, params).await
}

// TODO: implement parser cancellation
async fn tasks_parse_tree(documents: Arc<DashMap<Url, Document>>, uri: Url, text: String) -> bool {
    let mut success = false;
    let mut parser = parser::wat().expect("parser creation failed");
    // TODO: Fetch old_tree from cache and apply edits to prepare for incremental re-parsing.
    let old_tree = None;
    if let Some(tree) = parser.parse(&text[..], old_tree) {
        documents.insert(uri, Document {
            text,
            tree: Mutex::new(tree),
        });
        success = true;
    }
    success
}

async fn tasks_did_change(
    documents: Arc<DashMap<Url, Document>>,
    client: &'static Client,
    params: DidChangeTextDocumentParams,
) -> Fallible<()> {
    let DidChangeTextDocumentParams {
        text_document: VersionedTextDocumentIdentifier { uri, .. },
        content_changes,
    } = params;
    let TextDocumentContentChangeEvent { text, .. } = content_changes[0].clone();

    // spawn a parser and try to generate a syntax tree
    let tree_was_generated = {
        let task = {
            let documents = documents.clone();
            let uri = uri.clone();
            tasks_parse_tree(documents, uri, text)
        };
        tokio::spawn(task).await?
    };

    // on successful generation of a parse tree (which may contain syntax errors)
    if tree_was_generated {
        // run the auditor tasks
        let task = {
            let documents = documents.clone();
            let client = client.clone();
            let uri = uri.clone();
            auditor::tree_did_change(documents, client, uri)
        };
        tokio::spawn(task).await??;

        // run the elaborator tasks
        let task = {
            let documents = documents.clone();
            let client = client.clone();
            let uri = uri.clone();
            elaborator::tree_did_change(documents, client, uri)
        };
        tokio::spawn(task).await??;
    } else {
        // TODO: report
    }
    Ok(())
}

async fn tasks_did_close(
    documents: Arc<DashMap<Url, Document>>,
    client: &'static Client,
    params: DidCloseTextDocumentParams,
) -> Fallible<()> {
    let DidCloseTextDocumentParams {
        text_document: TextDocumentIdentifier { uri },
    } = &params;
    documents.remove(uri);

    let task = {
        let documents = documents.clone();
        let client = client.clone();
        let uri = uri.clone();
        auditor::tree_did_close(documents, client, uri)
    };
    tokio::spawn(task).await??;

    let task = {
        let documents = documents.clone();
        let client = client.clone();
        let uri = uri.clone();
        elaborator::tree_did_close(documents, client, uri)
    };
    tokio::spawn(task).await??;

    Ok(())
}
