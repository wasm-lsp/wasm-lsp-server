//! Tree-sitter languages for the WebAssembly language server.

#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

#[cfg(not(target_arch = "wasm32"))]
extern {
    #[allow(dead_code)]
    #[doc(hidden)]
    fn tree_sitter_wast() -> tree_sitter_sys::Language;

    #[allow(dead_code)]
    #[doc(hidden)]
    fn tree_sitter_wat() -> tree_sitter_sys::Language;
}

/// Tree-sitter language for the `.wast` grammar.
#[cfg(not(target_arch = "wasm32"))]
pub fn wast() -> tree_sitter::Language {
    #[allow(unsafe_code)]
    let inner = unsafe { crate::tree_sitter_wast() };
    inner.into()
}

/// Tree-sitter language for the `.wast` grammar.
#[cfg(target_arch = "wasm32")]
pub fn wast() -> tree_sitter::Language {
    use wasm_bindgen::JsCast;
    use wasm_bindgen_futures::JsFuture;
    let bytes: &[u8] = include_bytes!("../../../vendor/tree-sitter-wasm/wast/tree-sitter-wast.wasm");
    let promise = web_tree_sitter_sys::Language::load_bytes(&bytes.into());
    let future = JsFuture::from(promise);
    let result = futures::executor::block_on(future).unwrap();
    let inner = result.unchecked_into::<web_tree_sitter_sys::Language>();
    inner.into()
}

/// Tree-sitter language for the `.wat` grammar.
#[cfg(not(target_arch = "wasm32"))]
pub fn wat() -> tree_sitter::Language {
    #[allow(unsafe_code)]
    let inner = unsafe { crate::tree_sitter_wat() };
    inner.into()
}

/// Tree-sitter language for the `.wat` grammar.
#[cfg(target_arch = "wasm32")]
pub fn wat() -> tree_sitter::Language {
    use wasm_bindgen::JsCast;
    use wasm_bindgen_futures::JsFuture;
    let bytes: &[u8] = include_bytes!("../../../vendor/tree-sitter-wasm/wat/tree-sitter-wat.wasm");
    let promise = web_tree_sitter_sys::Language::load_bytes(&bytes.into());
    let future = JsFuture::from(promise);
    let result = futures::executor::block_on(future).unwrap();
    let inner = result.unchecked_into::<web_tree_sitter_sys::Language>();
    inner.into()
}
