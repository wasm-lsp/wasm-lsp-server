use crate::{parser::Parser, session::SessionHandle};
use dashmap::DashMap;
use failure::Fallible;
use lsp_types::*;
use tokio::sync::Mutex;
use tower_lsp::Client;
use tree_sitter::Tree;

/// Synchronizes document edits and parse trees and notifies other server components of changes.
pub struct Synchronizer {
    parser: Parser,
    session: SessionHandle,
    pub trees: DashMap<Url, Mutex<Tree>>,
}

impl Synchronizer {
    pub fn new(parser: Parser, session: SessionHandle) -> Fallible<Self> {
        let trees = DashMap::new();
        Ok(Synchronizer { parser, session, trees })
    }

    pub async fn did_open(&self, client: &Client, params: DidOpenTextDocumentParams) -> Fallible<()> {
        let mut parser = self.parser.wat.lock().await;
        let DidOpenTextDocumentParams {
            text_document: TextDocumentItem { uri, text, .. },
        } = &params;
        let tree;
        {
            // NOTE: Perhaps we should persist trees even on close. We could make this configurable.
            let old_tree = None;
            tree = parser.parse(text, old_tree);
        }
        if let Some(tree) = tree {
            let _ = self.trees.insert(uri.clone(), Mutex::new(tree));
            self.session.get().await.auditor.tree_did_open(client, uri).await?;
            self.session.get().await.elaborator.tree_did_open(client, uri).await?;
        } else {
            // TODO: report
        }
        Ok(())
    }

    pub async fn did_change(&self, client: &Client, params: DidChangeTextDocumentParams) -> Fallible<()> {
        let mut parser = self.parser.wat.lock().await;
        let DidChangeTextDocumentParams {
            text_document: VersionedTextDocumentIdentifier { uri, .. },
            content_changes,
        } = &params;
        let TextDocumentContentChangeEvent { ref text, .. } = content_changes[0];
        let mut success = false;
        {
            // TODO: Fetch old_tree from cache and apply edits to prepare for incremental re-parsing.
            let old_tree = None;
            if let Some(tree) = parser.parse(text, old_tree) {
                self.trees.insert(uri.clone(), Mutex::new(tree));
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
        self.trees.remove(&uri);
        self.session.get().await.auditor.tree_did_close(client, uri).await?;
        self.session.get().await.elaborator.tree_did_close(client, uri).await?;
        Ok(())
    }
}
