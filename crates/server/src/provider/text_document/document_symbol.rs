#![allow(unused)]

use crate::core::{self};
use lsp_text::RopeExt;
use std::sync::Arc;

/// Provider definitions for LSP `textDocument/documentSymbol` for `.wast` documents.
pub mod wast;

/// Provider definitions for LSP `textDocument/documentSymbol` for `.wat` documents.
pub mod wat;

/// Encodes data for constructing upcoming DocumentSymbols.
#[derive(Clone, Debug)]
pub(self) struct Data<'tree> {
    /// The tree-sitter Node to be processed as a symbol.
    pub node: tree_sitter::Node<'tree>,
    /// Number of (possibly filtered) children to be processed for the symbol.
    pub children_count: usize,
    /// The kind of document entity the symbol represents.
    pub kind: lsp::SymbolKind,
    /// The name hint for the symbol (used for anonymous entities).
    pub name_hint: &'static str,
}

/// Encodes actions for loop iterations when processing tree-sitter nodes.
#[derive(Debug)]
pub(self) enum Work<'tree> {
    /// Construct a DocumentSymbol and pop the data stack.
    Data,
    /// Add a tree-sitter node to remaining nodes to process.
    Node(tree_sitter::Node<'tree>),
}

/// Convenience type for packaging a (symbol) name with an lsp range and selection range.
#[derive(Clone, Debug)]
pub(self) struct SymbolRange {
    /// The name (identifier) of the symbol.
    pub name: String,
    /// The (node-enclosing) range of the symbol.
    pub range: lsp::Range,
    /// The (identifier-enclosing) range of the symbol.
    pub selection_range: lsp::Range,
}

/// Compute the name and ranges for a document symbol given tree-sitter node data.
pub(self) fn symbol_range(
    content: &ropey::Rope,
    node: tree_sitter::Node,
    name_hint: &'static str,
    field_id: u16,
) -> SymbolRange {
    let name;
    let range = content.tree_sitter_range_to_lsp_range(node.range());
    let selection_range;
    if let Some(inner_node) = node.child_by_field_id(field_id) {
        name = content.utf8_text_for_tree_sitter_node(&inner_node).into();
        selection_range = content.tree_sitter_range_to_lsp_range(inner_node.range());
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

/// Provider function for LSP `textDocument/documentSymbol`.
pub async fn document_symbol(
    session: Arc<core::Session>,
    params: lsp::DocumentSymbolParams,
) -> anyhow::Result<Option<lsp::DocumentSymbolResponse>> {
    let uri = params.text_document.uri.clone();
    let text = session.get_text(&uri).await?;
    let session = session.clone();
    let response = match text.language {
        core::Language::Wast => self::wast::document_symbol(session, params, &text.content).await?,
        core::Language::Wat => self::wat::document_symbol(session, params, &text.content).await?,
    };
    Ok(response)
}
