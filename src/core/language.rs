use failure::{Error, Fallible};
use std::convert::TryFrom;

#[derive(Clone, Copy, Debug)]
pub(crate) enum Language {
    Wast,
    Wat,
    Wit,
    Witx,
}

impl TryFrom<String> for Language {
    type Error = Error;

    fn try_from(language_id: String) -> Fallible<Self> {
        match language_id.as_ref() {
            "wasm.wast" => Ok(Language::Wast),
            "wasm.wat" => Ok(Language::Wat),
            "wasm.wit" => Ok(Language::Wit),
            "wasm.witx" => Ok(Language::Witx),
            _ => Err(crate::core::error::Error::CoreInvalidLanguageId(language_id).into()),
        }
    }
}
