//! Core functionality related to the LSP server session.

use crate::core::{
    database::{Database, DocumentStatus},
    document::Document,
    error::Fallible,
};
use dashmap::{
    mapref::one::{Ref, RefMut},
    DashMap,
};
use futures::stream::{self, StreamExt};
use lsp_types::*;
use std::sync::Arc;
use zerocopy::AsBytes;

/// Represents the current state of the LSP service.
pub(crate) struct Session {
    /// The document metadata database.
    database: Database,
    /// The store of currently open documents.
    pub(crate) documents: Arc<DashMap<Url, Document>>,
}

impl Session {
    /// Create a new session.
    pub(crate) fn new() -> Fallible<Self> {
        let database = Database::new()?;
        let documents = Arc::new(DashMap::new());
        Ok(Session { database, documents })
    }

    /// Insert an opened document into the session. Updates the documents hashmap and sets the
    /// document status in the database to "opened". Notifies subscribers to the document status.
    pub(crate) fn insert_document(&self, uri: Url, document: Document) -> Fallible<Option<Document>> {
        let result = self.documents.insert(uri.clone(), document);
        self.database
            .trees
            .documents
            .insert(&uri[..], DocumentStatus::opened().as_bytes())?;
        Ok(result)
    }

    /// Remove a closed document from the session.
    pub(crate) fn remove_document(&self, uri: &Url) -> Fallible<Option<(Url, Document)>> {
        let result = self.documents.remove(uri);
        self.database
            .trees
            .documents
            .insert(&uri[..], DocumentStatus::closed().as_bytes())?;
        Ok(result)
    }

    /// Get a document from the session. If the document is not yet open, this function will await
    /// until that happens. This usually occurs by a call to Self::insert_document from another
    /// thread of control.
    pub(crate) async fn get_document(&'_ self, uri: &Url) -> Option<Ref<'_, Url, Document>> {
        let mut result = self.documents.get(uri);
        if result.is_none() {
            let subscriber = self.database.trees.documents.watch_prefix(vec![]);
            let mut stream = stream::iter(subscriber);
            // FIXME: use a timeout for this in case the event never arrives; should be an error
            while let Some(event) = stream.next().await {
                if let sled::Event::Insert(key, val) = event {
                    if &*key == uri.as_ref().as_bytes() && val == DocumentStatus::opened().as_bytes() {
                        if let Some(document) = self.documents.get(uri) {
                            result = Some(document);
                            break;
                        } else {
                            // FIXME
                            unreachable!()
                        }
                    }
                }
            }
        }
        result
    }

    /// Get a mutable document from the session. If the document is not yet open, this function will
    /// await until that happens. This usually occurs by a call to Self::insert_document from
    /// another thread of control.
    pub(crate) async fn get_mut_document(&'_ self, uri: &Url) -> Option<RefMut<'_, Url, Document>> {
        let mut result = self.documents.get_mut(uri);
        if result.is_none() {
            let subscriber = self.database.trees.documents.watch_prefix(vec![]);
            let mut stream = stream::iter(subscriber);
            // FIXME: use a timeout for this in case the event never arrives; should be an error
            while let Some(event) = stream.next().await {
                if let sled::Event::Insert(key, val) = event {
                    if &*key == uri.as_ref().as_bytes() && val == DocumentStatus::opened().as_bytes() {
                        if let Some(document) = self.documents.get_mut(uri) {
                            result = Some(document);
                            break;
                        } else {
                            // FIXME
                            unreachable!()
                        }
                    }
                }
            }
        }
        result
    }
}
