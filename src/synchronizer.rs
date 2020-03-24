use crate::{parser::Parser, session::SessionHandle};
use dashmap::DashMap;
use failure::Fallible;
use lsp_types::*;
use tokio::sync::Mutex;
use tower_lsp::Client;
use tree_sitter::Tree;

pub struct Document {
    pub text: String,
    pub tree: Mutex<Tree>,
}

/// Synchronizes document edits and parse trees and notifies other server components of changes.
pub struct Synchronizer {
    parser: Parser,
    session: SessionHandle,
    pub documents: DashMap<Url, Document>,
}

impl Synchronizer {
    pub fn new(parser: Parser, session: SessionHandle) -> Fallible<Self> {
        let documents = DashMap::new();
        Ok(Synchronizer {
            parser,
            session,
            documents,
        })
    }

    pub async fn did_open(&self, client: &Client, params: DidOpenTextDocumentParams) -> Fallible<()> {
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
        self.did_change(client, params).await
    }

    pub async fn did_change(&self, client: &Client, params: DidChangeTextDocumentParams) -> Fallible<()> {
        let mut parser = self.parser.wat.lock().await;
        let DidChangeTextDocumentParams {
            text_document: VersionedTextDocumentIdentifier { uri, .. },
            content_changes,
        } = &params;
        let TextDocumentContentChangeEvent { text, .. } = &content_changes[0];
        let mut success = false;
        {
            // TODO: Fetch old_tree from cache and apply edits to prepare for incremental re-parsing.
            let old_tree = None;
            if let Some(tree) = parser.parse(text, old_tree) {
                let document = {
                    let text = text.clone();
                    let tree = Mutex::new(tree);
                    Document { text, tree }
                };
                self.documents.insert(uri.clone(), document);
                success = true;
            }
        }
        if success {
            self.session.get().await.auditor.tree_did_change(client, uri).await?;
            self.session.get().await.elaborator.tree_did_change(client, uri).await?;
        } else {
            // TODO: report
        }
        Ok(())
    }

    pub async fn did_close(&self, client: &Client, params: DidCloseTextDocumentParams) -> Fallible<()> {
        let DidCloseTextDocumentParams {
            text_document: TextDocumentIdentifier { uri },
        } = &params;
        self.documents.remove(uri);
        self.session.get().await.auditor.tree_did_close(client, uri).await?;
        self.session.get().await.elaborator.tree_did_close(client, uri).await?;
        Ok(())
    }
}
