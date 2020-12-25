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
            pub static ref ACTION_GET: u16 = super::language().id_for_node_kind("action_get", true);
            pub static ref ACTION_INVOKE: u16 = super::language().id_for_node_kind("action_invoke", true);
            pub static ref ANNOTATION: u16 = super::language().id_for_node_kind("annotation", true);
            pub static ref ASSERT_EXHAUSTION: u16 = super::language().id_for_node_kind("assert_exhaustion", true);
            pub static ref ASSERT_INVALID: u16 = super::language().id_for_node_kind("assert_invalid", true);
            pub static ref ASSERT_MALFORMED: u16 = super::language().id_for_node_kind("assert_malformed", true);
            pub static ref ASSERT_RETURN_ARITHMETIC_NAN: u16 =
                super::language().id_for_node_kind("assert_return_arithmetic_nan", true);
            pub static ref ASSERT_RETURN_CANONICAL_NAN: u16 =
                super::language().id_for_node_kind("assert_return_canonical_nan", true);
            pub static ref ASSERT_RETURN: u16 = super::language().id_for_node_kind("assert_return", true);
            pub static ref ASSERT_TRAP_ACTION: u16 = super::language().id_for_node_kind("assert_trap_action", true);
            pub static ref ASSERT_TRAP_MODULE: u16 = super::language().id_for_node_kind("assert_trap_module", true);
            pub static ref ASSERT_UNLINKABLE: u16 = super::language().id_for_node_kind("assert_unlinkable", true);
            pub static ref COMMAND: u16 = super::language().id_for_node_kind("command", true);
            pub static ref COMMENT_BLOCK_ANNOT: u16 = super::language().id_for_node_kind("comment_block_annot", true);
            pub static ref COMMENT_BLOCK: u16 = super::language().id_for_node_kind("comment_block", true);
            pub static ref COMMENT_LINE_ANNOT: u16 = super::language().id_for_node_kind("comment_line_annot", true);
            pub static ref COMMENT_LINE: u16 = super::language().id_for_node_kind("comment_line", true);
            pub static ref EXPORT: u16 = super::language().id_for_node_kind("export", true);
            pub static ref IDENTIFIER: u16 = super::language().id_for_node_kind("identifier", true);
            pub static ref IMPORT: u16 = super::language().id_for_node_kind("import", true);
            pub static ref INDEX: u16 = super::language().id_for_node_kind("index", true);
            pub static ref INSTR_PLAIN: u16 = super::language().id_for_node_kind("instr_plain", true);
            pub static ref INSTR: u16 = super::language().id_for_node_kind("instr", true);
            pub static ref LPAREN: u16 = super::language().id_for_node_kind("(", false);
            pub static ref META_INPUT: u16 = super::language().id_for_node_kind("meta_input", true);
            pub static ref META_OUTPUT: u16 = super::language().id_for_node_kind("meta_output", true);
            pub static ref META_SCRIPT: u16 = super::language().id_for_node_kind("meta_script", true);
            pub static ref MODULE_FIELD_DATA: u16 = super::language().id_for_node_kind("module_field_data", true);
            pub static ref MODULE_FIELD_ELEM: u16 = super::language().id_for_node_kind("module_field_elem", true);
            pub static ref MODULE_FIELD_EXPORT: u16 = super::language().id_for_node_kind("module_field_export", true);
            pub static ref MODULE_FIELD_FUNC: u16 = super::language().id_for_node_kind("module_field_func", true);
            pub static ref MODULE_FIELD_GLOBAL: u16 = super::language().id_for_node_kind("module_field_global", true);
            pub static ref MODULE_FIELD_IMPORT: u16 = super::language().id_for_node_kind("module_field_import", true);
            pub static ref MODULE_FIELD_MEMORY: u16 = super::language().id_for_node_kind("module_field_memory", true);
            pub static ref MODULE_FIELD_START: u16 = super::language().id_for_node_kind("module_field_start", true);
            pub static ref MODULE_FIELD_TABLE: u16 = super::language().id_for_node_kind("module_field_table", true);
            pub static ref MODULE_FIELD_TYPE: u16 = super::language().id_for_node_kind("module_field_type", true);
            pub static ref MODULE: u16 = super::language().id_for_node_kind("module", true);
            pub static ref NAME: u16 = super::language().id_for_node_kind("name", true);
            pub static ref REGISTER: u16 = super::language().id_for_node_kind("register", true);
            pub static ref ROOT: u16 = super::language().id_for_node_kind("ROOT", true);
            pub static ref RPAREN: u16 = super::language().id_for_node_kind(")", false);
            pub static ref SCRIPT_MODULE_BINARY: u16 = super::language().id_for_node_kind("script_module_binary", true);
            pub static ref SCRIPT_MODULE_QUOTE: u16 = super::language().id_for_node_kind("script_module_quote", true);
            pub static ref TYPE_USE: u16 = super::language().id_for_node_kind("type_use", true);
        }

        pub mod equals {
            #![allow(missing_docs)]
            #![allow(non_snake_case)]

            pub fn ACTION_GET(kind_id: u16) -> bool {
                *super::ACTION_GET == kind_id
            }

            pub fn ACTION_INVOKE(kind_id: u16) -> bool {
                *super::ACTION_INVOKE == kind_id
            }

            pub fn ANNOTATION(kind_id: u16) -> bool {
                *super::ANNOTATION == kind_id
            }

            pub fn ASSERT_EXHAUSTION(kind_id: u16) -> bool {
                *super::ASSERT_EXHAUSTION == kind_id
            }

            pub fn ASSERT_INVALID(kind_id: u16) -> bool {
                *super::ASSERT_INVALID == kind_id
            }

            pub fn ASSERT_MALFORMED(kind_id: u16) -> bool {
                *super::ASSERT_MALFORMED == kind_id
            }

            pub fn ASSERT_RETURN_ARITHMETIC_NAN(kind_id: u16) -> bool {
                *super::ASSERT_RETURN_ARITHMETIC_NAN == kind_id
            }

            pub fn ASSERT_RETURN_CANONICAL_NAN(kind_id: u16) -> bool {
                *super::ASSERT_RETURN_CANONICAL_NAN == kind_id
            }

            pub fn ASSERT_RETURN(kind_id: u16) -> bool {
                *super::ASSERT_RETURN == kind_id
            }

            pub fn ASSERT_TRAP_ACTION(kind_id: u16) -> bool {
                *super::ASSERT_TRAP_ACTION == kind_id
            }

            pub fn ASSERT_TRAP_MODULE(kind_id: u16) -> bool {
                *super::ASSERT_TRAP_MODULE == kind_id
            }

            pub fn ASSERT_UNLINKABLE(kind_id: u16) -> bool {
                *super::ASSERT_UNLINKABLE == kind_id
            }

            pub fn ASSERTION(kind_id: u16) -> bool {
                [
                    ASSERT_EXHAUSTION,
                    ASSERT_INVALID,
                    ASSERT_MALFORMED,
                    ASSERT_RETURN,
                    ASSERT_RETURN_ARITHMETIC_NAN,
                    ASSERT_RETURN_CANONICAL_NAN,
                    ASSERT_TRAP_ACTION,
                    ASSERT_TRAP_MODULE,
                    ASSERT_UNLINKABLE,
                ]
                .iter()
                .any(|pred| pred(kind_id))
            }

            pub fn COMMAND(kind_id: u16) -> bool {
                *super::COMMAND == kind_id
            }

            pub fn COMMENT_BLOCK_ANNOT(kind_id: u16) -> bool {
                *super::COMMENT_BLOCK_ANNOT == kind_id
            }

            pub fn COMMENT_BLOCK(kind_id: u16) -> bool {
                *super::COMMENT_BLOCK == kind_id
            }

            pub fn COMMENT_LINE_ANNOT(kind_id: u16) -> bool {
                *super::COMMENT_LINE_ANNOT == kind_id
            }

            pub fn COMMENT_LINE(kind_id: u16) -> bool {
                *super::COMMENT_LINE == kind_id
            }

            pub fn EXPORT(kind_id: u16) -> bool {
                *super::EXPORT == kind_id
            }

            pub fn IDENTIFIER(kind_id: u16) -> bool {
                *super::IDENTIFIER == kind_id
            }

            pub fn IMPORT(kind_id: u16) -> bool {
                *super::IMPORT == kind_id
            }

            pub fn INDEX(kind_id: u16) -> bool {
                *super::INDEX == kind_id
            }

            pub fn INSTR_PLAIN(kind_id: u16) -> bool {
                *super::INSTR_PLAIN == kind_id
            }

            pub fn INSTR(kind_id: u16) -> bool {
                *super::INSTR == kind_id
            }

            pub fn LPAREN(kind_id: u16) -> bool {
                *super::LPAREN == kind_id
            }

            pub fn META_INPUT(kind_id: u16) -> bool {
                *super::META_INPUT == kind_id
            }

            pub fn META_OUTPUT(kind_id: u16) -> bool {
                *super::META_OUTPUT == kind_id
            }

            pub fn META_SCRIPT(kind_id: u16) -> bool {
                *super::META_SCRIPT == kind_id
            }

            pub fn MODULE_FIELD_DATA(kind_id: u16) -> bool {
                *super::MODULE_FIELD_DATA == kind_id
            }

            pub fn MODULE_FIELD_ELEM(kind_id: u16) -> bool {
                *super::MODULE_FIELD_ELEM == kind_id
            }

            pub fn MODULE_FIELD_EXPORT(kind_id: u16) -> bool {
                *super::MODULE_FIELD_EXPORT == kind_id
            }

            pub fn MODULE_FIELD_FUNC(kind_id: u16) -> bool {
                *super::MODULE_FIELD_FUNC == kind_id
            }

            pub fn MODULE_FIELD_GLOBAL(kind_id: u16) -> bool {
                *super::MODULE_FIELD_GLOBAL == kind_id
            }

            pub fn MODULE_FIELD_IMPORT(kind_id: u16) -> bool {
                *super::MODULE_FIELD_IMPORT == kind_id
            }

            pub fn MODULE_FIELD_MEMORY(kind_id: u16) -> bool {
                *super::MODULE_FIELD_MEMORY == kind_id
            }

            pub fn MODULE_FIELD_START(kind_id: u16) -> bool {
                *super::MODULE_FIELD_START == kind_id
            }

            pub fn MODULE_FIELD_TABLE(kind_id: u16) -> bool {
                *super::MODULE_FIELD_TABLE == kind_id
            }

            pub fn MODULE_FIELD_TYPE(kind_id: u16) -> bool {
                *super::MODULE_FIELD_TYPE == kind_id
            }

            pub fn MODULE(kind_id: u16) -> bool {
                *super::MODULE == kind_id
            }

            pub fn NAME(kind_id: u16) -> bool {
                *super::NAME == kind_id
            }

            pub fn REGISTER(kind_id: u16) -> bool {
                *super::REGISTER == kind_id
            }

            pub fn ROOT(kind_id: u16) -> bool {
                *super::ROOT == kind_id
            }

            pub fn RPAREN(kind_id: u16) -> bool {
                *super::RPAREN == kind_id
            }

            pub fn SCRIPT_MODULE_BINARY(kind_id: u16) -> bool {
                *super::SCRIPT_MODULE_BINARY == kind_id
            }

            pub fn SCRIPT_MODULE_QUOTE(kind_id: u16) -> bool {
                *super::SCRIPT_MODULE_QUOTE == kind_id
            }

            pub fn TYPE_USE(kind_id: u16) -> bool {
                *super::TYPE_USE == kind_id
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
            pub static ref ANNOTATION: u16 = super::language().id_for_node_kind("annotation", true);
            pub static ref COMMENT_BLOCK_ANNOT: u16 = super::language().id_for_node_kind("comment_block_annot", true);
            pub static ref COMMENT_BLOCK: u16 = super::language().id_for_node_kind("comment_block", true);
            pub static ref COMMENT_LINE_ANNOT: u16 = super::language().id_for_node_kind("comment_line_annot", true);
            pub static ref COMMENT_LINE: u16 = super::language().id_for_node_kind("comment_line", true);
            pub static ref EXPORT: u16 = super::language().id_for_node_kind("export", true);
            pub static ref IDENTIFIER: u16 = super::language().id_for_node_kind("identifier", true);
            pub static ref IMPORT: u16 = super::language().id_for_node_kind("import", true);
            pub static ref INDEX: u16 = super::language().id_for_node_kind("index", true);
            pub static ref INSTR_PLAIN: u16 = super::language().id_for_node_kind("instr_plain", true);
            pub static ref INSTR: u16 = super::language().id_for_node_kind("instr", true);
            pub static ref LPAREN: u16 = super::language().id_for_node_kind("(", false);
            pub static ref MODULE_FIELD_DATA: u16 = super::language().id_for_node_kind("module_field_data", true);
            pub static ref MODULE_FIELD_ELEM: u16 = super::language().id_for_node_kind("module_field_elem", true);
            pub static ref MODULE_FIELD_EXPORT: u16 = super::language().id_for_node_kind("module_field_export", true);
            pub static ref MODULE_FIELD_FUNC: u16 = super::language().id_for_node_kind("module_field_func", true);
            pub static ref MODULE_FIELD_GLOBAL: u16 = super::language().id_for_node_kind("module_field_global", true);
            pub static ref MODULE_FIELD_IMPORT: u16 = super::language().id_for_node_kind("module_field_import", true);
            pub static ref MODULE_FIELD_MEMORY: u16 = super::language().id_for_node_kind("module_field_memory", true);
            pub static ref MODULE_FIELD_START: u16 = super::language().id_for_node_kind("module_field_start", true);
            pub static ref MODULE_FIELD_TABLE: u16 = super::language().id_for_node_kind("module_field_table", true);
            pub static ref MODULE_FIELD_TYPE: u16 = super::language().id_for_node_kind("module_field_type", true);
            pub static ref MODULE: u16 = super::language().id_for_node_kind("module", true);
            pub static ref NAME: u16 = super::language().id_for_node_kind("name", true);
            pub static ref ROOT: u16 = super::language().id_for_node_kind("ROOT", true);
            pub static ref RPAREN: u16 = super::language().id_for_node_kind(")", false);
            pub static ref TYPE_USE: u16 = super::language().id_for_node_kind("type_use", true);
        }

        pub mod equals {
            #![allow(missing_docs)]
            #![allow(non_snake_case)]

            pub fn ANNOTATION(kind_id: u16) -> bool {
                *super::ANNOTATION == kind_id
            }

            pub fn COMMENT_BLOCK_ANNOT(kind_id: u16) -> bool {
                *super::COMMENT_BLOCK_ANNOT == kind_id
            }

            pub fn COMMENT_BLOCK(kind_id: u16) -> bool {
                *super::COMMENT_BLOCK == kind_id
            }

            pub fn COMMENT_LINE_ANNOT(kind_id: u16) -> bool {
                *super::COMMENT_LINE_ANNOT == kind_id
            }

            pub fn COMMENT_LINE(kind_id: u16) -> bool {
                *super::COMMENT_LINE == kind_id
            }

            pub fn EXPORT(kind_id: u16) -> bool {
                *super::EXPORT == kind_id
            }

            pub fn IDENTIFIER(kind_id: u16) -> bool {
                *super::IDENTIFIER == kind_id
            }

            pub fn IMPORT(kind_id: u16) -> bool {
                *super::IMPORT == kind_id
            }

            pub fn INDEX(kind_id: u16) -> bool {
                *super::INDEX == kind_id
            }

            pub fn INSTR_PLAIN(kind_id: u16) -> bool {
                *super::INSTR_PLAIN == kind_id
            }

            pub fn INSTR(kind_id: u16) -> bool {
                *super::INSTR == kind_id
            }

            pub fn LPAREN(kind_id: u16) -> bool {
                *super::LPAREN == kind_id
            }

            pub fn MODULE_FIELD_DATA(kind_id: u16) -> bool {
                *super::MODULE_FIELD_DATA == kind_id
            }

            pub fn MODULE_FIELD_ELEM(kind_id: u16) -> bool {
                *super::MODULE_FIELD_ELEM == kind_id
            }

            pub fn MODULE_FIELD_EXPORT(kind_id: u16) -> bool {
                *super::MODULE_FIELD_EXPORT == kind_id
            }

            pub fn MODULE_FIELD_FUNC(kind_id: u16) -> bool {
                *super::MODULE_FIELD_FUNC == kind_id
            }

            pub fn MODULE_FIELD_GLOBAL(kind_id: u16) -> bool {
                *super::MODULE_FIELD_GLOBAL == kind_id
            }

            pub fn MODULE_FIELD_IMPORT(kind_id: u16) -> bool {
                *super::MODULE_FIELD_IMPORT == kind_id
            }

            pub fn MODULE_FIELD_MEMORY(kind_id: u16) -> bool {
                *super::MODULE_FIELD_MEMORY == kind_id
            }

            pub fn MODULE_FIELD_START(kind_id: u16) -> bool {
                *super::MODULE_FIELD_START == kind_id
            }

            pub fn MODULE_FIELD_TABLE(kind_id: u16) -> bool {
                *super::MODULE_FIELD_TABLE == kind_id
            }

            pub fn MODULE_FIELD_TYPE(kind_id: u16) -> bool {
                *super::MODULE_FIELD_TYPE == kind_id
            }

            pub fn MODULE(kind_id: u16) -> bool {
                *super::MODULE == kind_id
            }

            pub fn NAME(kind_id: u16) -> bool {
                *super::NAME == kind_id
            }

            pub fn ROOT(kind_id: u16) -> bool {
                *super::ROOT == kind_id
            }

            pub fn RPAREN(kind_id: u16) -> bool {
                *super::RPAREN == kind_id
            }

            pub fn TYPE_USE(kind_id: u16) -> bool {
                *super::TYPE_USE == kind_id
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
