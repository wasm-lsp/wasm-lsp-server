//! Parsers for the WebAssembly language server.

#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]
#![recursion_limit = "512"]

/// Functionality related to runtime errors.
pub mod error;

/// Functionality related to [`tree-sitter::Language`].
pub mod language;

/// Functionality related to [`tree-sitter::Parser`].
pub mod parser;

#[cfg(not(target_arch = "wasm32"))]
extern {
    #[allow(dead_code)]
    #[doc(hidden)]
    fn tree_sitter_wast() -> tree_sitter_sys::Language;

    #[allow(dead_code)]
    #[doc(hidden)]
    fn tree_sitter_wat() -> tree_sitter_sys::Language;

    #[allow(dead_code)]
    #[doc(hidden)]
    fn tree_sitter_wit() -> tree_sitter_sys::Language;

    #[allow(dead_code)]
    #[doc(hidden)]
    fn tree_sitter_witx() -> tree_sitter_sys::Language;
}

/// Utility trait for working with [`tree_sitter::Node`].
pub trait NodeExt {
    /// Predicate to determine if a supertype node matches a given subtype node kind.
    fn matches_subtypes(&self, supertype_id: u16, subtype_ids: &[u16]) -> bool;
}

impl<'tree> NodeExt for tree_sitter::Node<'tree> {
    fn matches_subtypes(&self, supertype_id: u16, subtype_ids: &[u16]) -> bool {
        if let Some(child) = self.named_child(0) {
            supertype_id == self.kind_id() && subtype_ids.contains(&child.kind_id())
        } else {
            false
        }
    }
}
