//! Functionality related to [`tree-sitter::Language`].

use crate::error::Error::{InvalidLanguageId, OsStrToStrFailed, PathExtensionFailed};
use std::{convert::TryFrom, path::Path};

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

/// Functions for working with the `.wast` grammar.
pub mod wast;

/// Functions for working with the `.wat` grammar.
pub mod wat;
