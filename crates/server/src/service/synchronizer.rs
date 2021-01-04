//! Synchronizes document changes between editor and server

/// Functions related to processing events for a document.
pub(crate) mod document {
    use crate::core::session::Session;
    use lspower::lsp_types::*;
    use std::sync::Arc;

    /// Handle a document "change" event.
    pub(crate) async fn change(session: Arc<Session>, params: DidChangeTextDocumentParams) -> anyhow::Result<()> {
        {
            let mut document = session.get_mut_document(&params.text_document.uri).await?;

            for change in &params.content_changes {
                let mut finished = false;
                let (start, end) = if let Some(range) = change.range {
                    let start = crate::util::position::byte_index(&document, &range.start)?;
                    let end = crate::util::position::byte_index(&document, &range.end)?;
                    (start, end)
                } else {
                    finished = true;
                    let start = 0;
                    let end = change.text.len();
                    (start, end)
                };

                let replace_with = &change.text;
                document.text.replace_range(start .. end, replace_with);

                if finished {
                    // FIXME: For now just assume there really are no more edits, per spec.
                    break;
                }
            }
        }

        let tree_was_generated = super::tree::change(session.clone(), params.text_document.uri.clone()).await?;

        // on successful generation of a parse tree (which may contain syntax errors)
        if tree_was_generated {
            // run the auditor tasks
            crate::provider::diagnostics::tree::change(session.clone(), params.text_document.uri).await?;
        } else {
            // TODO: report
        }

        Ok(())
    }

    /// Handle a document "close" event.
    pub(crate) async fn close(session: Arc<Session>, params: DidCloseTextDocumentParams) -> anyhow::Result<()> {
        let DidCloseTextDocumentParams {
            text_document: TextDocumentIdentifier { uri },
        } = &params;
        session.remove_document(uri)?;
        crate::provider::diagnostics::tree::close(session.clone(), uri.clone()).await?;
        Ok(())
    }

    /// Handle a document "open" event.
    pub(crate) async fn open(session: Arc<Session>, params: DidOpenTextDocumentParams) -> anyhow::Result<()> {
        let DidOpenTextDocumentParams {
            text_document: TextDocumentItem { uri, .. },
        } = &params;

        // spawn a parser and try to generate a syntax tree
        let tree_was_generated = super::tree::open(session.clone(), params.clone()).await?;

        // on successful generation of a parse tree (which may contain syntax errors)
        if tree_was_generated {
            crate::provider::diagnostics::tree::open(session.clone(), uri.clone()).await?;
        } else {
            // TODO: report
        }

        Ok(())
    }
}

/// Functions related to processing parse tree events for a document.
mod tree {
    use crate::core::{document::Document, language, session::Session};
    use lspower::lsp_types::*;
    use std::{convert::TryFrom, sync::Arc};
    use tokio::sync::Mutex;

    // TODO: implement parser cancellation
    /// Handle a parse tree "change" event.
    pub(super) async fn change(session: Arc<Session>, uri: Url) -> anyhow::Result<bool> {
        let mut document = session.get_mut_document(&uri).await?;

        let tree = {
            let mut parser = document.parser.lock().await;
            // FIXME: we reset the parser since we don't handle incremental changes yet
            parser.reset();
            // TODO: Fetch old_tree from cache and apply edits to prepare for incremental re-parsing.
            let old_tree = None;
            parser.parse(&document.text[..], old_tree)
        };

        let mut success = false;
        if let Some(tree) = tree {
            document.tree = Mutex::new(tree);
            success = true;
        }

        Ok(success)
    }

    // TODO: implement parser cancellation
    /// Handle a parse tree "open" event.
    pub(super) async fn open(session: Arc<Session>, params: DidOpenTextDocumentParams) -> anyhow::Result<bool> {
        let DidOpenTextDocumentParams {
            text_document: TextDocumentItem {
                language_id, text, uri, ..
            },
        } = params;

        let language = language::Language::try_from(language_id.as_str())?;
        let mut parser = tree_sitter::Parser::try_from(language)?;

        let tree = {
            // TODO: Fetch old_tree from cache and apply edits to prepare for incremental re-parsing.
            let old_tree = None;
            parser.parse(&text[..], old_tree)
        };

        let mut success = false;
        if let Some(tree) = tree {
            let document = Document {
                language,
                parser: Mutex::new(parser),
                text,
                tree: Mutex::new(tree),
            };
            session.insert_document(uri, document)?;
            success = true;
        }

        Ok(success)
    }
}
