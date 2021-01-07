//! Synchronizes document changes between editor and server

/// Functions related to processing events for a document.
pub(crate) mod document {
    use crate::core;
    use ropey::Rope;
    use std::sync::Arc;

    /// Handle a document "change" event.
    pub(crate) async fn change(
        session: Arc<core::Session>,
        params: lsp::DidChangeTextDocumentParams,
    ) -> anyhow::Result<()> {
        {
            let mut document = session.get_mut_document(&params.text_document.uri).await?;

            for change in &params.content_changes {
                if change.range.is_some() {
                    let edit = document.build_edit(change)?;
                    document.apply_edit(&edit);
                } else {
                    document.content = Rope::from(change.text.as_str());
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
    pub(crate) async fn close(
        session: Arc<core::Session>,
        params: lsp::DidCloseTextDocumentParams,
    ) -> anyhow::Result<()> {
        let lsp::DidCloseTextDocumentParams {
            text_document: lsp::TextDocumentIdentifier { uri },
        } = &params;
        session.remove_document(uri)?;
        crate::provider::diagnostics::tree::close(session.clone(), uri.clone()).await?;
        Ok(())
    }

    /// Handle a document "open" event.
    pub(crate) async fn open(
        session: Arc<core::Session>,
        params: lsp::DidOpenTextDocumentParams,
    ) -> anyhow::Result<()> {
        let lsp::DidOpenTextDocumentParams {
            text_document: lsp::TextDocumentItem { uri, .. },
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
    use crate::core::{self, language, RopeExt};
    use ropey::Rope;
    use std::{convert::TryFrom, sync::Arc};
    use tokio::sync::Mutex;

    // TODO: implement parser cancellation
    /// Handle a parse tree "change" event.
    pub(super) async fn change(session: Arc<core::Session>, uri: lsp::Url) -> anyhow::Result<bool> {
        let mut document = session.get_mut_document(&uri).await?;

        let tree = {
            let mut parser = document.parser.lock().await;
            // FIXME: we reset the parser since we don't handle incremental changes yet
            parser.reset();
            let mut callback = document.content.chunk_walker(0).callback_adapter();
            // TODO: Fetch old_tree from cache and apply edits to prepare for incremental re-parsing.
            let old_tree = None;
            parser.parse_with(&mut callback, old_tree)
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
    pub(super) async fn open(
        session: Arc<core::Session>,
        params: lsp::DidOpenTextDocumentParams,
    ) -> anyhow::Result<bool> {
        let lsp::DidOpenTextDocumentParams {
            text_document: lsp::TextDocumentItem {
                language_id, text, uri, ..
            },
        } = params;

        let language = language::Language::try_from(language_id.as_str())?;
        let mut parser = tree_sitter::Parser::try_from(language)?;

        let rope = Rope::from(text);

        let tree = {
            let mut callback = rope.chunk_walker(0).callback_adapter();
            // TODO: Fetch old_tree from cache and apply edits to prepare for incremental re-parsing.
            let old_tree = None;
            parser.parse_with(&mut callback, old_tree)
        };

        let mut success = false;
        if let Some(tree) = tree {
            // let rope = Rope::from(text);
            let document = core::Document {
                language,
                content: rope,
                parser: Mutex::new(parser),
                tree: Mutex::new(tree),
            };
            session.insert_document(uri, document)?;
            success = true;
        }

        Ok(success)
    }
}
