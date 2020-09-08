//! Elaborates parse trees into structured data to be cached in the database.

/// Elaborator definitions specific to ".wat" and ".wast" files.
mod wast;

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
            // TODO: elaborate
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
    use tower_lsp::lsp_types::SymbolKind;

    /// Encodes data for constructing upcoming DocumentSymbols.
    #[derive(Clone, Debug)]
    pub(crate) struct Data<'a> {
        /// The tree-sitter Node to be processed as a symbol.
        pub(crate) node: tree_sitter::Node<'a>,
        /// Number of (possibly filtered) children to be processed for the symbol.
        pub(crate) children_count: usize,
        /// The kind of document entity the symbol represents.
        pub(crate) kind: SymbolKind,
        /// The name hint for the symbol (used for anonymous entities).
        pub(crate) name_hint: &'static str,
    }

    /// Encodes actions for loop iterations when processing tree-sitter nodes.
    #[derive(Debug)]
    pub(crate) enum Work<'a> {
        /// Construct a DocumentSymbol and pop the data stack.
        Data,
        /// Add a tree-sitter node to remaining nodes to process.
        Node(tree_sitter::Node<'a>),
    }

    /// Convenience type for packaging a (symbol) name with an lsp range and selection range.
    #[derive(Clone, Debug)]
    pub(crate) struct SymbolRange {
        /// The name (identifier) of the symbol.
        pub(crate) name: String,
        /// The (node-enclosing) range of the symbol.
        pub(crate) range: tower_lsp::lsp_types::Range,
        /// The (identifier-enclosing) range of the symbol.
        pub(crate) selection_range: tower_lsp::lsp_types::Range,
    }

    /// Compute the name and ranges for a document symbol given tree-sitter node data.
    pub(crate) fn symbol_range(
        source: &[u8],
        node: tree_sitter::Node,
        name_hint: &'static str,
        field_id: u16,
    ) -> SymbolRange {
        let name;
        let range = crate::util::node::range(&node);
        let selection_range;
        if let Some(inner_node) = node.child_by_field_id(field_id) {
            name = inner_node.utf8_text(source).unwrap().into();
            selection_range = crate::util::node::range(&inner_node);
        } else {
            name = format!("<{}@{}:{}>", name_hint, range.start.line + 1, range.start.character + 1);
            selection_range = range;
        }

        SymbolRange {
            name,
            range,
            selection_range,
        }
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
        Language::Wat => self::wast::document_symbol(&document).await,
    };
    Ok(result)
}
