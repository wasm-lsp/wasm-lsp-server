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
            pub(crate) static ref IDENTIFIER: u16 = super::language().field_id_for_name("identifier").unwrap();
            pub(crate) static ref MODULE_FIELD: u16 = super::language().field_id_for_name("module_field").unwrap();
        }
    }

    pub(crate) mod kind {
        use lazy_static::lazy_static;

        lazy_static! {
            pub(crate) static ref COMMAND: u16 = super::language().id_for_node_kind("command", true);
            pub(crate) static ref COMMENT_BLOCK: u16 = super::language().id_for_node_kind("comment_block", true);
            pub(crate) static ref COMMENT_LINE: u16 = super::language().id_for_node_kind("comment_line", true);
            pub(crate) static ref INLINE_MODULE: u16 = super::language().id_for_node_kind("inline_module", true);
            pub(crate) static ref MODULE_FIELD_DATA: u16 =
                super::language().id_for_node_kind("module_field_data", true);
            pub(crate) static ref MODULE_FIELD_ELEM: u16 =
                super::language().id_for_node_kind("module_field_elem", true);
            pub(crate) static ref MODULE_FIELD_FUNC: u16 =
                super::language().id_for_node_kind("module_field_func", true);
            pub(crate) static ref MODULE_FIELD_GLOBAL: u16 =
                super::language().id_for_node_kind("module_field_global", true);
            pub(crate) static ref MODULE_FIELD_MEMORY: u16 =
                super::language().id_for_node_kind("module_field_memory", true);
            pub(crate) static ref MODULE_FIELD_TABLE: u16 =
                super::language().id_for_node_kind("module_field_table", true);
            pub(crate) static ref MODULE_FIELD_TYPE: u16 =
                super::language().id_for_node_kind("module_field_type", true);
            pub(crate) static ref MODULE: u16 = super::language().id_for_node_kind("module", true);
            pub(crate) static ref PARSE: u16 = super::language().id_for_node_kind("PARSE", true);
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
            pub(crate) static ref IDENTIFIER: u16 = super::language().field_id_for_name("identifier").unwrap();
            pub(crate) static ref MODULE_FIELD: u16 = super::language().field_id_for_name("module_field").unwrap();
        }
    }

    pub(crate) mod kind {
        use lazy_static::lazy_static;

        lazy_static! {
            pub(crate) static ref INLINE_MODULE: u16 = super::language().id_for_node_kind("inline_module", true);
            pub(crate) static ref COMMENT_BLOCK: u16 = super::language().id_for_node_kind("comment_block", true);
            pub(crate) static ref COMMENT_LINE: u16 = super::language().id_for_node_kind("comment_line", true);
            pub(crate) static ref MODULE_FIELD_DATA: u16 =
                super::language().id_for_node_kind("module_field_data", true);
            pub(crate) static ref MODULE_FIELD_ELEM: u16 =
                super::language().id_for_node_kind("module_field_elem", true);
            pub(crate) static ref MODULE_FIELD_FUNC: u16 =
                super::language().id_for_node_kind("module_field_func", true);
            pub(crate) static ref MODULE_FIELD_GLOBAL: u16 =
                super::language().id_for_node_kind("module_field_global", true);
            pub(crate) static ref MODULE_FIELD_MEMORY: u16 =
                super::language().id_for_node_kind("module_field_memory", true);
            pub(crate) static ref MODULE_FIELD_TABLE: u16 =
                super::language().id_for_node_kind("module_field_table", true);
            pub(crate) static ref MODULE_FIELD_TYPE: u16 =
                super::language().id_for_node_kind("module_field_type", true);
            pub(crate) static ref MODULE: u16 = super::language().id_for_node_kind("module", true);
            pub(crate) static ref PARSE: u16 = super::language().id_for_node_kind("PARSE", true);
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
