use crate::core::error::Error::{InvalidLanguageId, OsStrToStrFailed, PathExtensionFailed};
use std::{convert::TryFrom, path::Path};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Language {
    Wast,
    Wat,
}

impl Language {
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
        match language_id {
            "wasm.wast" => Ok(Language::Wast),
            "wasm.wat" => Ok(Language::Wat),
            _ => Err(InvalidLanguageId(language_id.into()).into()),
        }
    }
}

impl TryFrom<&Path> for Language {
    type Error = anyhow::Error;

    fn try_from(path: &Path) -> anyhow::Result<Self> {
        let file_ext = path.extension().ok_or_else(|| PathExtensionFailed(path.into()))?;
        let file_ext = file_ext.to_str().ok_or(OsStrToStrFailed)?;
        let language_id = format!("wasm.{}", file_ext);
        Language::try_from(language_id.as_str())
    }
}

pub mod wast {
    #[cfg(not(target_arch = "wasm32"))]
    #[allow(unsafe_code)]
    pub fn language() -> tree_sitter::Language {
        let inner = unsafe { crate::tree_sitter_wast() };
        inner.into()
    }

    #[cfg(target_arch = "wasm32")]
    pub fn language() -> tree_sitter::Language {
        use wasm_bindgen::JsCast;
        use wasm_bindgen_futures::JsFuture;
        let bytes: &[u8] = include_bytes!("../../../../vendor/tree-sitter-wasm/wast/tree-sitter-wast.wasm");
        let promise = web_tree_sitter_sys::Language::load_bytes(&bytes.into());
        let future = JsFuture::from(promise);
        let result = futures::future::block_on(future).unwrap();
        let inner = result.unchecked_into::<web_tree_sitter_sys::Language>();
        inner.into()
    }
}

pub mod wat {
    #[cfg(not(target_arch = "wasm32"))]
    #[allow(unsafe_code)]
    pub fn language() -> tree_sitter::Language {
        let inner = unsafe { crate::tree_sitter_wat() };
        inner.into()
    }

    #[cfg(target_arch = "wasm32")]
    #[allow(unsafe_code)]
    pub fn language() -> tree_sitter::Language {
        use wasm_bindgen::JsCast;
        use wasm_bindgen_futures::JsFuture;
        let bytes: &[u8] = include_bytes!("../../../../vendor/tree-sitter-wasm/wat/tree-sitter-wat.wasm");
        let promise = web_tree_sitter_sys::Language::load_bytes(&bytes.into());
        let future = JsFuture::from(promise);
        let result = futures::future::block_on(future).unwrap();
        let inner = result.unchecked_into::<web_tree_sitter_sys::Language>();
        inner.into()
    }
}
