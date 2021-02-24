//! Functionality related to runtime errors.

use std::path::PathBuf;
use thiserror::Error;

/// Runtime errors for the WebAssembly parsers.
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Error)]
pub enum Error {
    /// Error that occurs when parsing an invalid language-id string.
    #[error("InvalidLanguageId: {0}")]
    InvalidLanguageId(String),
    /// Error that occurs when [`std::ffi::OsStr::to_str`] returns `None`.
    #[error("OsStrToStrFailed")]
    OsStrToStrFailed,
    /// Error that occurs when [`std::path::Path::extension`] returns `None`.
    #[error("PathExtensionFailed: {0}")]
    PathExtensionFailed(PathBuf),
}
