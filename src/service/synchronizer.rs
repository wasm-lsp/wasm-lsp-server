/// Synchronizes document changes between editor and server
use crate::core::document::Document;
use dashmap::DashMap;
use failure::Fallible;
use lsp_types::*;
use std::{convert::TryFrom, sync::Arc};
use tokio::sync::Mutex;
use tower_lsp::Client;

pub(crate) async fn tasks_did_open(
    documents: Arc<DashMap<Url, Document>>,
    client: &'static Client,
    params: DidOpenTextDocumentParams,
) -> Fallible<()> {
    let DidOpenTextDocumentParams {
        text_document: TextDocumentItem { uri, .. },
    } = &params;

    // spawn a parser and try to generate a syntax tree
    let tree_was_generated = {
        let task = {
            let documents = documents.clone();
            tasks_open_tree(documents, params.clone())
        };
        tokio::spawn(task).await??
    };

    // on successful generation of a parse tree (which may contain syntax errors)
    if tree_was_generated {
        // run the auditor tasks
        let task = {
            let documents = documents.clone();
            let client = client; // FIXME
            let uri = uri.clone();
            crate::service::auditor::tree_did_open(documents, client, uri)
        };
        tokio::spawn(task).await??;

        // run the elaborator tasks
        let task = {
            let documents = documents.clone();
            let client = client; // FIXME
            let uri = uri.clone();
            crate::service::elaborator::tree_did_open(documents, client, uri)
        };
        tokio::spawn(task).await??;
    } else {
        // TODO: report
    }
    Ok(())
}

// TODO: implement parser cancellation
async fn tasks_open_tree(documents: Arc<DashMap<Url, Document>>, params: DidOpenTextDocumentParams) -> Fallible<bool> {
    let DidOpenTextDocumentParams {
        text_document: TextDocumentItem {
            language_id, text, uri, ..
        },
    } = params;

    let language = crate::core::language::Language::try_from(language_id)?;
    let mut parser = crate::core::parser::try_from(language)?;

    // TODO: Fetch old_tree from cache and apply edits to prepare for incremental re-parsing.
    let old_tree = None;

    let mut success = false;
    if let Some(tree) = parser.parse(&text[..], old_tree) {
        documents.insert(uri.clone(), Document {
            language,
            parser: Mutex::new(parser),
            text: text.clone(),
            tree: Mutex::new(tree),
        });
        success = true;
    }
    Ok(success)
}

// TODO: implement parser cancellation
async fn tasks_change_tree(documents: Arc<DashMap<Url, Document>>, uri: Url, text: String) -> bool {
    let mut success = false;
    if let Some(mut document) = documents.get_mut(&uri) {
        let tree;
        {
            let mut parser = document.parser.lock().await;
            // FIXME: we reset the parser since we don't handle incremental changes yet
            parser.reset();
            // TODO: Fetch old_tree from cache and apply edits to prepare for incremental re-parsing.
            let old_tree = None;
            tree = parser.parse(&text[..], old_tree);
        }
        if let Some(tree) = tree {
            document.text = text;
            document.tree = Mutex::new(tree);
            success = true;
        }
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

    let tree_was_generated = {
        let task = {
            let documents = documents.clone();
            tasks_change_tree(documents, uri.clone(), text)
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
