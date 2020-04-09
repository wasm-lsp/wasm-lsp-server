//! Core functionality related to document languages.

use crate::core::error::Fallible;
use std::convert::TryFrom;

/// Languages supported by the server.
#[derive(Clone, Copy, Debug)]
pub(crate) enum Language {
    Wast,
    Wat,
    Wit,
    Witx,
}

impl TryFrom<String> for Language {
    type Error = anyhow::Error;

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

pub(crate) mod wast {
    #[allow(unsafe_code)]
    pub(crate) fn language() -> tree_sitter::Language {
        unsafe { crate::tree_sitter_wast() }
    }

    pub(crate) mod field {
        use lazy_static::lazy_static;

        lazy_static! {
            pub(crate) static ref COMMAND: u16 = super::language().field_id_for_name("command").unwrap();
            pub(crate) static ref FIELD: u16 = super::language().field_id_for_name("field").unwrap();
            pub(crate) static ref ID: u16 = super::language().field_id_for_name("id").unwrap();
        }
    }

    pub(crate) mod kind {
        use lazy_static::lazy_static;

        lazy_static! {
            pub(crate) static ref COMMAND: u16 = super::language().id_for_node_kind("command", true);
            pub(crate) static ref DATA: u16 = super::language().id_for_node_kind("data", true);
            pub(crate) static ref ELEM: u16 = super::language().id_for_node_kind("elem", true);
            pub(crate) static ref ENTRYPOINT: u16 = super::language().id_for_node_kind("ENTRYPOINT", true);
            pub(crate) static ref FUNC: u16 = super::language().id_for_node_kind("func", true);
            pub(crate) static ref GLOBAL: u16 = super::language().id_for_node_kind("global", true);
            pub(crate) static ref MEMORY: u16 = super::language().id_for_node_kind("mem", true);
            pub(crate) static ref MODULE: u16 = super::language().id_for_node_kind("module", true);
            pub(crate) static ref MODULE_INLINE: u16 = super::language().id_for_node_kind("module_inline", true);
            pub(crate) static ref TABLE: u16 = super::language().id_for_node_kind("table", true);
            pub(crate) static ref TYPE: u16 = super::language().id_for_node_kind("type", true);
        }
    }
}

pub(crate) mod wat {
    #[allow(unsafe_code)]
    pub(crate) fn language() -> tree_sitter::Language {
        unsafe { crate::tree_sitter_wat() }
    }

    pub(crate) mod field {
        use lazy_static::lazy_static;

        lazy_static! {
            pub(crate) static ref FIELD: u16 = super::language().field_id_for_name("field").unwrap();
            pub(crate) static ref ID: u16 = super::language().field_id_for_name("id").unwrap();
        }
    }

    pub(crate) mod kind {
        use lazy_static::lazy_static;

        lazy_static! {
            pub(crate) static ref DATA: u16 = super::language().id_for_node_kind("data", true);
            pub(crate) static ref ELEM: u16 = super::language().id_for_node_kind("elem", true);
            pub(crate) static ref ENTRYPOINT: u16 = super::language().id_for_node_kind("ENTRYPOINT", true);
            pub(crate) static ref FUNC: u16 = super::language().id_for_node_kind("func", true);
            pub(crate) static ref GLOBAL: u16 = super::language().id_for_node_kind("global", true);
            pub(crate) static ref MEMORY: u16 = super::language().id_for_node_kind("mem", true);
            pub(crate) static ref MODULE: u16 = super::language().id_for_node_kind("module", true);
            pub(crate) static ref MODULE_INLINE: u16 = super::language().id_for_node_kind("module_inline", true);
            pub(crate) static ref TABLE: u16 = super::language().id_for_node_kind("table", true);
            pub(crate) static ref TYPE: u16 = super::language().id_for_node_kind("type", true);
        }
    }
}

pub(crate) mod wit {
    #[allow(unsafe_code)]
    pub(crate) fn language() -> tree_sitter::Language {
        unsafe { crate::tree_sitter_wit() }
    }
}

pub(crate) mod witx {
    #[allow(unsafe_code)]
    pub(crate) fn language() -> tree_sitter::Language {
        unsafe { crate::tree_sitter_witx() }
    }
}
