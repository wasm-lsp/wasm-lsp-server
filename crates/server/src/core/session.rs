//! Core functionality related to the LSP server session.

use crate::core::{
    database::{Database, DocumentStatus},
    document::Document,
    error::Error,
};
use dashmap::{
    mapref::one::{Ref, RefMut},
    DashMap,
};
use tower_lsp::{lsp_types::*, Client};
use zerocopy::AsBytes;

/// Represents the current state of the LSP service.
pub(crate) struct Session {
    /// The LSP client handle.
    client: Option<Client>,
    /// The document metadata database.
    database: Database,
    /// The store of currently open documents.
    documents: DashMap<Url, Document>,
}

impl Session {
    /// Create a new session.
    pub(crate) fn new(client: Option<Client>) -> anyhow::Result<Self> {
        let database = Database::new()?;
        let documents = DashMap::new();
        Ok(Session {
            client,
            database,
            documents,
        })
    }

    pub(crate) fn client(&self) -> anyhow::Result<&Client> {
        self.client.as_ref().ok_or_else(|| Error::ClientNotInitialized.into())
    }

    /// Insert an opened document into the session. Updates the documents hashmap and sets the
    /// document status in the database to "opened". Notifies subscribers to the document status.
    pub(crate) fn insert_document(&self, uri: Url, document: Document) -> anyhow::Result<Option<Document>> {
        let result = self.documents.insert(uri.clone(), document);
        self.database
            .trees
            .documents
            .insert(&uri[..], DocumentStatus::opened().as_bytes())?;
        Ok(result)
    }

    /// Remove a closed document from the session. Updates the documents hashmap and sets the
    /// document status in the database to "closed". Notifies subscribers to the document status.
    pub(crate) fn remove_document(&self, uri: &Url) -> anyhow::Result<Option<(Url, Document)>> {
        let result = self.documents.remove(uri);
        self.database
            .trees
            .documents
            .insert(&uri[..], DocumentStatus::closed().as_bytes())?;
        Ok(result)
    }

    pub(crate) async fn get_document(&self, uri: &Url) -> anyhow::Result<Ref<'_, Url, Document>> {
        self.documents
            .get(uri)
            .ok_or_else(|| Error::DocumentNotFound(uri.clone()).into())
    }

    pub(crate) async fn get_mut_document(&self, uri: &Url) -> anyhow::Result<RefMut<'_, Url, Document>> {
        self.documents
            .get_mut(uri)
            .ok_or_else(|| Error::DocumentNotFound(uri.clone()).into())
    }
}

#[cfg(test)]
mod tests {
    use super::Session;
    use crate::core::error::Error;
    use tower_lsp::lsp_types::*;

    #[tokio::test]
    async fn client_not_initialized() -> anyhow::Result<()> {
        let client = None;
        let session = Session::new(client)?;
        let result = session.client();

        assert!(result.is_err());
        if let Err(error) = result {
            let expected = &Error::ClientNotInitialized;
            let actual = error.root_cause().downcast_ref::<Error>();
            assert!(actual.is_some());
            if let Some(actual) = actual {
                assert_eq!(expected, actual);
            }
        }

        Ok(())
    }

    #[tokio::test]
    async fn document_not_found() -> anyhow::Result<()> {
        let client = None;
        let session = Session::new(client)?;
        let uri = Url::parse("inmemory:///test")?;
        let result = session.get_document(&uri).await;

        assert!(result.is_err());
        if let Err(error) = result {
            let expected = &Error::DocumentNotFound(uri.clone());
            let actual = error.root_cause().downcast_ref::<Error>();
            assert!(actual.is_some());
            if let Some(actual) = actual {
                assert_eq!(expected, actual);
            }
        }

        Ok(())
    }
}
