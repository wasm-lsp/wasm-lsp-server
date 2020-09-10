//! Core functionality related to document languages.

use crate::core::error::Error;
use std::convert::TryFrom;

/// Languages supported by the server.
#[derive(Clone, Copy, Debug)]
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
            _ => Err(Error::InvalidLanguageId(language_id.into()).into()),
        }
    }
}

/// Functions for working with the `.wast` grammar.
pub mod wast {
    #[allow(unsafe_code)]
    /// Tree-sitter language for the `.wast` grammar.
    pub fn language() -> tree_sitter::Language {
        unsafe { crate::tree_sitter_wast() }
    }

    pub mod field {
        #![allow(missing_docs)]

        use lazy_static::lazy_static;

        lazy_static! {
            pub static ref IDENTIFIER: u16 = super::language().field_id_for_name("identifier").unwrap();
        }
    }

    pub mod kind {
        #![allow(missing_docs)]

        use lazy_static::lazy_static;

        lazy_static! {
            pub static ref COMMAND: u16 = super::language().id_for_node_kind("command", true);
            pub static ref INSTR_PLAIN: u16 = super::language().id_for_node_kind("instr_plain", true);
            pub static ref INSTR: u16 = super::language().id_for_node_kind("instr", true);
            pub static ref MODULE_FIELD_DATA: u16 = super::language().id_for_node_kind("module_field_data", true);
            pub static ref MODULE_FIELD_ELEM: u16 = super::language().id_for_node_kind("module_field_elem", true);
            pub static ref MODULE_FIELD_FUNC: u16 = super::language().id_for_node_kind("module_field_func", true);
            pub static ref MODULE_FIELD_GLOBAL: u16 = super::language().id_for_node_kind("module_field_global", true);
            pub static ref MODULE_FIELD_MEMORY: u16 = super::language().id_for_node_kind("module_field_memory", true);
            pub static ref MODULE_FIELD_TABLE: u16 = super::language().id_for_node_kind("module_field_table", true);
            pub static ref MODULE_FIELD_TYPE: u16 = super::language().id_for_node_kind("module_field_type", true);
            pub static ref MODULE_FIELD: u16 = super::language().id_for_node_kind("module_field", true);
            pub static ref MODULE: u16 = super::language().id_for_node_kind("module", true);
            pub static ref PARSE: u16 = super::language().id_for_node_kind("PARSE", true);
        }
    }
}

/// Functions for working with the `.wat` grammar.
pub mod wat {
    /// Tree-sitter language for the `.wat` grammar.
    #[allow(unsafe_code)]
    pub fn language() -> tree_sitter::Language {
        unsafe { crate::tree_sitter_wat() }
    }

    pub mod field {
        #![allow(missing_docs)]

        use lazy_static::lazy_static;

        lazy_static! {
            pub static ref IDENTIFIER: u16 = super::language().field_id_for_name("identifier").unwrap();
        }
    }

    pub mod kind {
        #![allow(missing_docs)]

        use lazy_static::lazy_static;

        lazy_static! {
            pub static ref INSTR_PLAIN: u16 = super::language().id_for_node_kind("instr_plain", true);
            pub static ref INSTR: u16 = super::language().id_for_node_kind("instr", true);
            pub static ref MODULE_FIELD_DATA: u16 = super::language().id_for_node_kind("module_field_data", true);
            pub static ref MODULE_FIELD_ELEM: u16 = super::language().id_for_node_kind("module_field_elem", true);
            pub static ref MODULE_FIELD_FUNC: u16 = super::language().id_for_node_kind("module_field_func", true);
            pub static ref MODULE_FIELD_GLOBAL: u16 = super::language().id_for_node_kind("module_field_global", true);
            pub static ref MODULE_FIELD_MEMORY: u16 = super::language().id_for_node_kind("module_field_memory", true);
            pub static ref MODULE_FIELD_TABLE: u16 = super::language().id_for_node_kind("module_field_table", true);
            pub static ref MODULE_FIELD_TYPE: u16 = super::language().id_for_node_kind("module_field_type", true);
            pub static ref MODULE_FIELD: u16 = super::language().id_for_node_kind("module_field", true);
            pub static ref MODULE: u16 = super::language().id_for_node_kind("module", true);
            pub static ref PARSE: u16 = super::language().id_for_node_kind("PARSE", true);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_from_invalid_language_id() {
        use std::convert::TryInto;
        let result: anyhow::Result<Language> = "".try_into();
        assert!(result.is_err());
    }
}
