#![deny(clippy::all)]
#![deny(unsafe_code)]

pub mod core;
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
