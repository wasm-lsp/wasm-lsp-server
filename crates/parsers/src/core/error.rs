use std::path::PathBuf;
use thiserror::Error;

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Error)]
pub enum Error {
    #[error("InvalidLanguageId: {0}")]
    InvalidLanguageId(String),
    #[error("OsStrToStrFailed")]
    OsStrToStrFailed,
    #[error("PathExtensionFailed: {0}")]
    PathExtensionFailed(PathBuf),
}
