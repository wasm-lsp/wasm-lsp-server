/// Synchronizes document changes between editor and server
use crate::core::document::Document;
use dashmap::DashMap;
use failure::Fallible;
use lsp_types::*;
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_lsp::Client;

pub(crate) async fn tasks_did_open(
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
    let mut parser = crate::core::parser::wast().expect("parser creation failed");
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

pub(crate) async fn tasks_did_change(
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
            let client = client; // FIXME
            let uri = uri.clone();
            crate::service::auditor::tree_did_change(documents, client, uri)
        };
        tokio::spawn(task).await??;

        // run the elaborator tasks
        let task = {
            let documents = documents.clone();
            let client = client; // FIXME
            let uri = uri.clone();
            crate::service::elaborator::tree_did_change(documents, client, uri)
        };
        tokio::spawn(task).await??;
    } else {
        // TODO: report
    }
    Ok(())
}

pub(crate) async fn tasks_did_close(
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
        let client = client; // FIXME
        let uri = uri.clone();
        crate::service::auditor::tree_did_close(documents, client, uri)
    };
    tokio::spawn(task).await??;

    let task = {
        let documents = documents.clone();
        let client = client; // FIXME
        let uri = uri.clone();
        crate::service::elaborator::tree_did_close(documents, client, uri)
    };
    tokio::spawn(task).await??;

    Ok(())
}
