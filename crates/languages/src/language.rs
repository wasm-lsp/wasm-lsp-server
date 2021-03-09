use std::{convert::TryFrom, path::Path};

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

/// Languages supported by the server.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Language {
    /// The `.wast` language.
    Wast,
    /// The `.wat` language.
    Wat,
}

impl Language {
    /// Compute the language id string for the given language.
    pub fn id(&self) -> &str {
        match self {
            Language::Wast => "wasm.wast",
            Language::Wat => "wasm.wat",
        }
    }
}

impl TryFrom<&str> for Language {
    type Error = anyhow::Error;

    fn try_from(language_id: &str) -> anyhow::Result<Self> {
        use crate::error::Error;
        match language_id {
            "wasm.wast" => Ok(Language::Wast),
            "wasm.wat" => Ok(Language::Wat),
            _ => Err(Error::InvalidLanguageId(language_id.into()).into()),
        }
    }
}

impl TryFrom<&Path> for Language {
    type Error = anyhow::Error;

    fn try_from(path: &Path) -> anyhow::Result<Self> {
        use crate::error::Error;
        let file_ext = path
            .extension()
            .ok_or_else(|| Error::PathExtensionFailed(path.into()))?;
        let file_ext = file_ext.to_str().ok_or(Error::OsStrToStrFailed)?;
        let language_id = format!("wasm.{}", file_ext);
        Language::try_from(language_id.as_str())
    }
}
