//! Core functionality related to document languages.

use crate::core::error::Error::{InvalidLanguageId, OsStrToStrFailed, PathExtensionFailed};
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
pub mod wast {
    use lazy_static::lazy_static;

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
            pub static ref MODULE: u16 = super::language().id_for_node_kind("module", true);
            pub static ref ROOT: u16 = super::language().id_for_node_kind("ROOT", true);
        }

        pub mod equals {
            #![allow(missing_docs)]
            #![allow(non_snake_case)]

            pub fn COMMAND(kind_id: u16) -> bool {
                *super::COMMAND == kind_id
            }

            pub fn MODULE(kind_id: u16) -> bool {
                *super::MODULE == kind_id
            }

            pub fn ROOT(kind_id: u16) -> bool {
                *super::ROOT == kind_id
            }
        }
    }

    lazy_static! {
        #[allow(missing_docs)]
        pub static ref MODULE_FIELDS: Vec<u16> = vec![
            *kind::MODULE_FIELD_DATA,
            *kind::MODULE_FIELD_ELEM,
            *kind::MODULE_FIELD_FUNC,
            *kind::MODULE_FIELD_GLOBAL,
            *kind::MODULE_FIELD_MEMORY,
            *kind::MODULE_FIELD_TABLE,
            *kind::MODULE_FIELD_TYPE,
        ];
    }
}

/// Functions for working with the `.wat` grammar.
pub mod wat {
    use lazy_static::lazy_static;

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
            pub static ref MODULE: u16 = super::language().id_for_node_kind("module", true);
            pub static ref ROOT: u16 = super::language().id_for_node_kind("ROOT", true);
        }

        pub mod equals {
            #![allow(missing_docs)]
            #![allow(non_snake_case)]

            pub fn MODULE(kind_id: u16) -> bool {
                *super::MODULE == kind_id
            }

            pub fn ROOT(kind_id: u16) -> bool {
                *super::ROOT == kind_id
            }
        }
    }

    lazy_static! {
        #[allow(missing_docs)]
        pub static ref MODULE_FIELDS: Vec<u16> = vec![
            *kind::MODULE_FIELD_DATA,
            *kind::MODULE_FIELD_ELEM,
            *kind::MODULE_FIELD_FUNC,
            *kind::MODULE_FIELD_GLOBAL,
            *kind::MODULE_FIELD_MEMORY,
            *kind::MODULE_FIELD_TABLE,
            *kind::MODULE_FIELD_TYPE,
        ];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_from_str_then_id() -> anyhow::Result<()> {
        assert_eq!("wasm.wast", Language::try_from("wasm.wast")?.id());
        assert_eq!("wasm.wat", Language::try_from("wasm.wat")?.id());
        Ok(())
    }

    #[test]
    fn try_from_str_invalid_language_id() {
        let result = Language::try_from("");
        assert!(result.is_err());
    }

    #[test]
    fn try_from_ext_then_id() -> anyhow::Result<()> {
        assert_eq!("wasm.wast", Language::try_from(Path::new("foo.wast"))?.id());
        assert_eq!("wasm.wat", Language::try_from(Path::new("bar.wat"))?.id());
        Ok(())
    }

    #[test]
    fn try_from_ext_invalid_language_id() {
        let result = Language::try_from(Path::new("foo.txt"));
        assert!(result.is_err());
    }
}
