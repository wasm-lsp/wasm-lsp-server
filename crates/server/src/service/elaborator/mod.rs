//! Elaborates parse trees into structured data to be cached in the database.

/// Elaborator definitions specific to ".wast" files.
mod wast;
/// Elaborator definitions specific to ".wat" files.
mod wat;
/// Elaborator definitions specific to ".wit" files.
mod wit;
/// Elaborator definitions specific to ".witx" files.
mod witx;

/// Functions related to processing parse tree events for a document.
pub(crate) mod tree {
    use crate::core::session::Session;
    use std::sync::Arc;
    use tower_lsp::lsp_types::*;

    /// Handle a parse tree "change" event.
    pub(crate) async fn change(session: Arc<Session>, uri: Url) -> anyhow::Result<()> {
        let document = session.get_document(&uri).await?;
        let tree = document.tree.lock().await.clone();
        let node = tree.root_node();
        if !node.has_error() {
            log::info!("syntax well-formed");
        }
        Ok(())
    }

    /// Handle a parse tree "close" event.
    pub(crate) async fn close(_: Arc<Session>, _: Url) -> anyhow::Result<()> {
        Ok(())
    }

    /// Handle a parse tree "open" event.
    pub(crate) async fn open(session: Arc<Session>, uri: Url) -> anyhow::Result<()> {
        self::change(session, uri).await
    }
}

use crate::core::{language::Language, session::Session};
use std::sync::Arc;
use tower_lsp::lsp_types::*;

/// Functionality used for computing "textDocument/documentSymbols".
mod document_symbol {
    use crate::{
        core::document::Document,
        util::node::{symbol_range, SymbolRange},
    };
    use std::borrow::Cow;
    use tower_lsp::lsp_types::{Range, SymbolKind};

    /// Encodes data for constructing upcoming DocumentSymbols.
    #[derive(Clone, Debug)]
    pub(crate) struct Data<'a> {
        /// Number of children to be processed for given symbol.
        pub(crate) children_count: usize,
        /// The kind of document entity the symbol represents.
        pub(crate) kind: SymbolKind,
        /// The name (identifier) for the symbol.
        pub(crate) name: Cow<'a, str>,
        /// The (node-enclosing) range for the symbol.
        pub(crate) range: Range,
        /// The (identifier-enclosing) range for the symbol.
        pub(crate) selection_range: Range,
    }

    /// Encodes actions for loop iterations when processing tree-sitter nodes.
    #[derive(Debug)]
    pub(crate) enum Work<'a> {
        /// Construct a DocumentSymbol and pop the data stack.
        Data,
        /// Add a tree-sitter node to remaining nodes to process.
        Node(tree_sitter::Node<'a>),
    }

    /// Convenience function for processing document symbol nodes.
    #[allow(clippy::needless_lifetimes)]
    pub(crate) fn push<'a>(
        document: &'a Document,
        field_id: u16,
        data: &mut Vec<Data<'a>>,
        work: &mut Vec<Work>,
        node: &tree_sitter::Node,
        empty_name: &'static str,
        kind: SymbolKind,
    ) {
        let SymbolRange {
            name,
            range,
            selection_range,
        } = symbol_range(&document.text.as_bytes(), empty_name, &node, field_id);
        work.push(Work::Data);
        data.push(Data {
            children_count: 0,
            kind,
            name,
            range,
            selection_range,
        });
    }
}

// FIXME: reorganize this to where outline is pulled from database
/// Compute the symbols for a given document.
pub(crate) async fn document_symbol(
    session: Arc<Session>,
    params: DocumentSymbolParams,
) -> anyhow::Result<Option<DocumentSymbolResponse>> {
    let DocumentSymbolParams {
        text_document: TextDocumentIdentifier { uri },
        ..
    } = &params;
    let document = session.get_document(uri).await?;
    let result = match document.language {
        Language::Wast => self::wast::document_symbol(&document).await,
        Language::Wat => self::wat::document_symbol(&document).await,
        Language::Wit => self::wit::document_symbol(&document).await,
        Language::Witx => self::witx::document_symbol(&document).await,
    };
    Ok(result)
}
