//! Definitions related to LSP documents.

use crate::core::{self};
use lsp_text::{RopeExt, TextEdit};
use std::sync::Arc;

#[cfg(feature = "runtime-agnostic")]
use async_lock::Mutex;
#[cfg(feature = "runtime-tokio")]
use tokio::sync::Mutex;

/// Documents for the LSP session.
pub struct Document {
    /// The language-id of the document.
    pub language: core::Language,
    /// The textual content of the document.
    pub content: ropey::Rope,
    /// The active parser for the document.
    pub parser: tree_sitter::Parser,
    /// The current syntax tree for the document.
    pub tree: tree_sitter::Tree,
}

impl Document {
    /// Create a new [`Document`] given [`lsp::DidOpenTextDocumentParams`].
    pub fn open(
        session: Arc<crate::core::Session>,
        params: lsp::DidOpenTextDocumentParams,
    ) -> anyhow::Result<Option<Self>> {
        let language = core::Language::try_from(params.text_document.language_id.as_str())?;
        let mut parser = tree_sitter::Parser::new()?;
        match language {
            core::Language::Wast => parser.set_language(&session.languages.wast)?,
            core::Language::Wat => parser.set_language(&session.languages.wat)?,
        };

        let content = ropey::Rope::from(params.text_document.text);
        let result = {
            let content = content.clone();
            let byte_idx = 0;
            let callback = content.chunk_walker(byte_idx).callback_adapter_for_tree_sitter();
            let old_tree = None;
            parser.parse_with(callback, old_tree)?
        };
        Ok(result.map(|tree| core::Document {
            language,
            content,
            parser,
            tree,
        }))
    }

    ///
    pub async fn change<'changes>(
        session: Arc<core::Session>,
        uri: &lsp::Url,
        content: &ropey::Rope,
        edits: &[TextEdit<'changes>],
    ) -> anyhow::Result<Option<tree_sitter::Tree>> {
        let result = {
            let parser = session.get_mut_parser(uri).await?;
            let mut parser = parser.lock().await;

            let callback = {
                let mut content = content.clone();
                content.shrink_to_fit();
                let byte_idx = 0;
                content.chunk_walker(byte_idx).callback_adapter_for_tree_sitter()
            };

            let old_tree = session.get_mut_tree(uri).await?;
            let mut old_tree = old_tree.lock().await;

            for edit in edits {
                old_tree.edit(&edit.input_edit);
            }

            parser.parse_with(callback, Some(&*old_tree))?
        };

        if let Some(tree) = result {
            {
                let tree = tree.clone();
                *session.get_mut_tree(uri).await?.value_mut() = Mutex::new(tree);
            }
            Ok(Some(tree))
        } else {
            Ok(None)
        }
    }

    /// Return the language-id and textual content portion of the [`Document`].
    pub fn text(&self) -> core::Text {
        core::Text {
            language: self.language,
            content: self.content.clone(),
        }
    }
}
