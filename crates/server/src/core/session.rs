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
use std::time::Duration;
use tokio::time::timeout;
use tower_lsp::{lsp_types::*, Client};
use zerocopy::AsBytes;

/// Represents the current state of the LSP service.
pub(crate) struct Session {
    /// The LSP client handle.
    pub(crate) client: Client,
    /// The document metadata database.
    database: Database,
    /// The store of currently open documents.
    documents: DashMap<Url, Document>,
}

impl Session {
    /// Create a new session.
    pub(crate) fn new(client: Client) -> Fallible<Self> {
        let database = Database::new()?;
        let documents = DashMap::new();
        Ok(Session {
            client,
            database,
            documents,
        })
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
    /// until that happens (up to 5 seconds, otherwise failing). This usually occurs by a call to
    /// Self::insert_document from another thread of control.
    pub(crate) async fn get_document(&self, uri: &Url) -> Fallible<Option<Ref<'_, Url, Document>>> {
        #![allow(clippy::needless_lifetimes)]
        async fn future<'a>(session: &'a Session, uri: &Url) -> Fallible<Option<Ref<'a, Url, Document>>> {
            let mut result = session.documents.get(uri);
            if result.is_none() {
                let subscriber = session.database.trees.documents.watch_prefix(vec![]);
                let mut stream = stream::iter(subscriber);
                // FIXME: use a timeout for this in case the event never arrives; should be an error
                while let Some(event) = stream.next().await {
                    if let sled::Event::Insert { key, value } = event {
                        if &*key == uri.as_ref().as_bytes() && value == DocumentStatus::opened().as_bytes() {
                            if let Some(document) = session.documents.get(uri) {
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
            Ok(result)
        }
        timeout(Duration::from_secs(5), future(self, uri)).await?
    }

    /// Get a mutable document from the session. If the document is not yet open, this function will
    /// await until that happens (up to 5 seconds, otherwise failing). This usually occurs by a call
    /// to Self::insert_document from another thread of control.
    pub(crate) async fn get_mut_document(&self, uri: &Url) -> Fallible<Option<RefMut<'_, Url, Document>>> {
        #![allow(clippy::needless_lifetimes)]
        async fn future<'a>(session: &'a Session, uri: &Url) -> Fallible<Option<RefMut<'a, Url, Document>>> {
            let mut result = session.documents.get_mut(uri);
            if result.is_none() {
                let subscriber = session.database.trees.documents.watch_prefix(vec![]);
                let mut stream = stream::iter(subscriber);
                // FIXME: use a timeout for this in case the event never arrives; should be an error
                while let Some(event) = stream.next().await {
                    if let sled::Event::Insert { key, value } = event {
                        if &*key == uri.as_ref().as_bytes() && value == DocumentStatus::opened().as_bytes() {
                            if let Some(document) = session.documents.get_mut(uri) {
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
            Ok(result)
        }
        timeout(Duration::from_secs(5), future(self, uri)).await?
    }
}
