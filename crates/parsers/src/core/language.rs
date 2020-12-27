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

        use super::language;
        use lazy_static::lazy_static;

        lazy_static! {
            pub static ref ACTION_GET: u16 = language().id_for_node_kind("action_get", true);
            pub static ref ACTION_INVOKE: u16 = language().id_for_node_kind("action_invoke", true);
            pub static ref ANNOTATION: u16 = language().id_for_node_kind("annotation", true);
            pub static ref ASSERT_EXHAUSTION: u16 = language().id_for_node_kind("assert_exhaustion", true);
            pub static ref ASSERT_INVALID: u16 = language().id_for_node_kind("assert_invalid", true);
            pub static ref ASSERT_MALFORMED: u16 = language().id_for_node_kind("assert_malformed", true);
            pub static ref ASSERT_RETURN_ARITHMETIC_NAN: u16 =
                language().id_for_node_kind("assert_return_arithmetic_nan", true);
            pub static ref ASSERT_RETURN_CANONICAL_NAN: u16 =
                language().id_for_node_kind("assert_return_canonical_nan", true);
            pub static ref ASSERT_RETURN: u16 = language().id_for_node_kind("assert_return", true);
            pub static ref ASSERT_TRAP_ACTION: u16 = language().id_for_node_kind("assert_trap_action", true);
            pub static ref ASSERT_TRAP_MODULE: u16 = language().id_for_node_kind("assert_trap_module", true);
            pub static ref ASSERT_UNLINKABLE: u16 = language().id_for_node_kind("assert_unlinkable", true);
            pub static ref COMMAND: u16 = language().id_for_node_kind("command", true);
            pub static ref COMMENT_BLOCK_ANNOT: u16 = language().id_for_node_kind("comment_block_annot", true);
            pub static ref COMMENT_BLOCK: u16 = language().id_for_node_kind("comment_block", true);
            pub static ref COMMENT_LINE_ANNOT: u16 = language().id_for_node_kind("comment_line_annot", true);
            pub static ref COMMENT_LINE: u16 = language().id_for_node_kind("comment_line", true);
            pub static ref EXPORT: u16 = language().id_for_node_kind("export", true);
            pub static ref IDENTIFIER: u16 = language().id_for_node_kind("identifier", true);
            pub static ref IMPORT: u16 = language().id_for_node_kind("import", true);
            pub static ref INDEX: u16 = language().id_for_node_kind("index", true);
            pub static ref INSTR_PLAIN: u16 = language().id_for_node_kind("instr_plain", true);
            pub static ref INSTR: u16 = language().id_for_node_kind("instr", true);
            pub static ref META_INPUT: u16 = language().id_for_node_kind("meta_input", true);
            pub static ref META_OUTPUT: u16 = language().id_for_node_kind("meta_output", true);
            pub static ref META_SCRIPT: u16 = language().id_for_node_kind("meta_script", true);
            pub static ref MODULE_FIELD_DATA: u16 = language().id_for_node_kind("module_field_data", true);
            pub static ref MODULE_FIELD_ELEM: u16 = language().id_for_node_kind("module_field_elem", true);
            pub static ref MODULE_FIELD_EXPORT: u16 = language().id_for_node_kind("module_field_export", true);
            pub static ref MODULE_FIELD_FUNC: u16 = language().id_for_node_kind("module_field_func", true);
            pub static ref MODULE_FIELD_GLOBAL: u16 = language().id_for_node_kind("module_field_global", true);
            pub static ref MODULE_FIELD_IMPORT: u16 = language().id_for_node_kind("module_field_import", true);
            pub static ref MODULE_FIELD_MEMORY: u16 = language().id_for_node_kind("module_field_memory", true);
            pub static ref MODULE_FIELD_START: u16 = language().id_for_node_kind("module_field_start", true);
            pub static ref MODULE_FIELD_TABLE: u16 = language().id_for_node_kind("module_field_table", true);
            pub static ref MODULE_FIELD_TYPE: u16 = language().id_for_node_kind("module_field_type", true);
            pub static ref MODULE: u16 = language().id_for_node_kind("module", true);
            pub static ref NAME: u16 = language().id_for_node_kind("name", true);
            pub static ref REGISTER: u16 = language().id_for_node_kind("register", true);
            pub static ref ROOT: u16 = language().id_for_node_kind("ROOT", true);
            pub static ref SCRIPT_MODULE_BINARY: u16 = language().id_for_node_kind("script_module_binary", true);
            pub static ref SCRIPT_MODULE_QUOTE: u16 = language().id_for_node_kind("script_module_quote", true);
            pub static ref TYPE_USE: u16 = language().id_for_node_kind("type_use", true);
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

        pub mod token {
            #![allow(missing_docs)]

            use super::language;
            use lazy_static::lazy_static;

            lazy_static! {
                pub static ref ALIGN: u16 = language().id_for_node_kind("align", false);
                pub static ref ASSERT_EXHAUSTION: u16 = language().id_for_node_kind("assert_exhaustion", false);
                pub static ref ASSERT_INVALID: u16 = language().id_for_node_kind("assert_invalid", false);
                pub static ref ASSERT_MALFORMED: u16 = language().id_for_node_kind("assert_malformed", false);
                pub static ref ASSERT_RETURN_ARITHMETIC_NAN: u16 =
                    language().id_for_node_kind("assert_return_arithmetic_nan", false);
                pub static ref ASSERT_RETURN_CANONICAL_NAN: u16 =
                    language().id_for_node_kind("assert_return_canonical_nan", false);
                pub static ref ASSERT_RETURN: u16 = language().id_for_node_kind("assert_return", false);
                pub static ref ASSERT_TRAP: u16 = language().id_for_node_kind("assert_trap", false);
                pub static ref ASSERT_UNLINKABLE: u16 = language().id_for_node_kind("assert_unlinkable", false);
                pub static ref BINARY: u16 = language().id_for_node_kind("binary", false);
                pub static ref BLOCK: u16 = language().id_for_node_kind("block", false);
                pub static ref BR_TABLE: u16 = language().id_for_node_kind("br_table", false);
                pub static ref CALL_INDIRECT: u16 = language().id_for_node_kind("call_indirect", false);
                pub static ref DATA: u16 = language().id_for_node_kind("data", false);
                pub static ref DECLARE: u16 = language().id_for_node_kind("declare", false);
                pub static ref DOLLAR_SIGN: u16 = language().id_for_node_kind("$", false);
                pub static ref ELEM: u16 = language().id_for_node_kind("elem", false);
                pub static ref ELSE: u16 = language().id_for_node_kind("else", false);
                pub static ref END: u16 = language().id_for_node_kind("end", false);
                pub static ref EQUALS: u16 = language().id_for_node_kind("=", false);
                pub static ref EXPORT: u16 = language().id_for_node_kind("export", false);
                pub static ref EXTERNREF: u16 = language().id_for_node_kind("externref", false);
                pub static ref F32: u16 = language().id_for_node_kind("f32", false);
                pub static ref F64: u16 = language().id_for_node_kind("f64", false);
                pub static ref FULL_STOP: u16 = language().id_for_node_kind(".", false);
                pub static ref FUNC: u16 = language().id_for_node_kind("func", false);
                pub static ref FUNCREF: u16 = language().id_for_node_kind("funcref", false);
                pub static ref GET: u16 = language().id_for_node_kind("get", false);
                pub static ref GLOBAL: u16 = language().id_for_node_kind("global", false);
                pub static ref I32: u16 = language().id_for_node_kind("i32", false);
                pub static ref I64: u16 = language().id_for_node_kind("i64", false);
                pub static ref IF: u16 = language().id_for_node_kind("if", false);
                pub static ref IMPORT: u16 = language().id_for_node_kind("import", false);
                pub static ref INF: u16 = language().id_for_node_kind("inf", false);
                pub static ref INPUT: u16 = language().id_for_node_kind("input", false);
                pub static ref INVOKE: u16 = language().id_for_node_kind("invoke", false);
                pub static ref ITEM: u16 = language().id_for_node_kind("item", false);
                pub static ref LOCAL: u16 = language().id_for_node_kind("local", false);
                pub static ref LOOP: u16 = language().id_for_node_kind("loop", false);
                pub static ref LPAREN_AMPERSAND: u16 = language().id_for_node_kind("(@", false);
                pub static ref LPAREN_SEMICOLON: u16 = language().id_for_node_kind("(;", false);
                pub static ref LPAREN: u16 = language().id_for_node_kind("(", false);
                pub static ref MEMORY: u16 = language().id_for_node_kind("memory", false);
                pub static ref MUT: u16 = language().id_for_node_kind("mut", false);
                pub static ref OFFSET: u16 = language().id_for_node_kind("offset", false);
                pub static ref OUTPUT: u16 = language().id_for_node_kind("output", false);
                pub static ref PARAM: u16 = language().id_for_node_kind("param", false);
                pub static ref QUOTE: u16 = language().id_for_node_kind("quote", false);
                pub static ref REF: u16 = language().id_for_node_kind("ref", false);
                pub static ref REGISTER: u16 = language().id_for_node_kind("register", false);
                pub static ref RESULT: u16 = language().id_for_node_kind("result", false);
                pub static ref REVERSE_SOLIDUS_REVERSE_SOLIDUS: u16 = language().id_for_node_kind("\\", false);
                pub static ref RPAREN: u16 = language().id_for_node_kind(")", false);
                pub static ref SCRIPT: u16 = language().id_for_node_kind("script", false);
                pub static ref SEMICOLON_SEMICOLON: u16 = language().id_for_node_kind(";;", false);
                pub static ref TABLE: u16 = language().id_for_node_kind("table", false);
                pub static ref THEN: u16 = language().id_for_node_kind("then", false);
                pub static ref TYPE: u16 = language().id_for_node_kind("type", false);
                pub static ref V128: u16 = language().id_for_node_kind("v128", false);
            }

            pub mod equals {
                #![allow(missing_docs)]
                #![allow(non_snake_case)]

                pub fn ALIGN(kind_id: u16) -> bool {
                    *super::ALIGN == kind_id
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

                pub fn ASSERT_TRAP(kind_id: u16) -> bool {
                    *super::ASSERT_TRAP == kind_id
                }

                pub fn ASSERT_UNLINKABLE(kind_id: u16) -> bool {
                    *super::ASSERT_UNLINKABLE == kind_id
                }

                pub fn BINARY(kind_id: u16) -> bool {
                    *super::BINARY == kind_id
                }

                pub fn BLOCK(kind_id: u16) -> bool {
                    *super::BLOCK == kind_id
                }

                pub fn BR_TABLE(kind_id: u16) -> bool {
                    *super::BR_TABLE == kind_id
                }

                pub fn CALL_INDIRECT(kind_id: u16) -> bool {
                    *super::CALL_INDIRECT == kind_id
                }

                pub fn DATA(kind_id: u16) -> bool {
                    *super::DATA == kind_id
                }

                pub fn DECLARE(kind_id: u16) -> bool {
                    *super::DECLARE == kind_id
                }

                pub fn DOLLAR_SIGN(kind_id: u16) -> bool {
                    *super::DOLLAR_SIGN == kind_id
                }

                pub fn ELEM(kind_id: u16) -> bool {
                    *super::ELEM == kind_id
                }

                pub fn ELSE(kind_id: u16) -> bool {
                    *super::ELSE == kind_id
                }

                pub fn END(kind_id: u16) -> bool {
                    *super::END == kind_id
                }

                pub fn EQUALS(kind_id: u16) -> bool {
                    *super::EQUALS == kind_id
                }

                pub fn EXPORT(kind_id: u16) -> bool {
                    *super::EXPORT == kind_id
                }

                pub fn EXTERNREF(kind_id: u16) -> bool {
                    *super::EXTERNREF == kind_id
                }

                pub fn F32(kind_id: u16) -> bool {
                    *super::F32 == kind_id
                }

                pub fn F64(kind_id: u16) -> bool {
                    *super::F64 == kind_id
                }

                pub fn FULL_STOP(kind_id: u16) -> bool {
                    *super::FULL_STOP == kind_id
                }

                pub fn FUNC(kind_id: u16) -> bool {
                    *super::FUNC == kind_id
                }

                pub fn FUNCREF(kind_id: u16) -> bool {
                    *super::FUNCREF == kind_id
                }

                pub fn GLOBAL(kind_id: u16) -> bool {
                    *super::GLOBAL == kind_id
                }

                pub fn I32(kind_id: u16) -> bool {
                    *super::I32 == kind_id
                }

                pub fn I64(kind_id: u16) -> bool {
                    *super::I64 == kind_id
                }

                pub fn IF(kind_id: u16) -> bool {
                    *super::IF == kind_id
                }

                pub fn IMPORT(kind_id: u16) -> bool {
                    *super::IMPORT == kind_id
                }

                pub fn INF(kind_id: u16) -> bool {
                    *super::INF == kind_id
                }

                pub fn INPUT(kind_id: u16) -> bool {
                    *super::INPUT == kind_id
                }

                pub fn INVOKE(kind_id: u16) -> bool {
                    *super::INVOKE == kind_id
                }

                pub fn ITEM(kind_id: u16) -> bool {
                    *super::ITEM == kind_id
                }

                pub fn LOCAL(kind_id: u16) -> bool {
                    *super::LOCAL == kind_id
                }

                pub fn LOOP(kind_id: u16) -> bool {
                    *super::LOOP == kind_id
                }

                pub fn LPAREN_AMPERSAND(kind_id: u16) -> bool {
                    *super::LPAREN_AMPERSAND == kind_id
                }

                pub fn LPAREN_SEMICOLON(kind_id: u16) -> bool {
                    *super::LPAREN_SEMICOLON == kind_id
                }

                pub fn LPAREN(kind_id: u16) -> bool {
                    *super::LPAREN == kind_id
                }

                pub fn MEMORY(kind_id: u16) -> bool {
                    *super::MEMORY == kind_id
                }

                pub fn MUT(kind_id: u16) -> bool {
                    *super::MUT == kind_id
                }

                pub fn OFFSET(kind_id: u16) -> bool {
                    *super::OFFSET == kind_id
                }

                pub fn OUTPUT(kind_id: u16) -> bool {
                    *super::OUTPUT == kind_id
                }

                pub fn PARAM(kind_id: u16) -> bool {
                    *super::PARAM == kind_id
                }

                pub fn QUOTE(kind_id: u16) -> bool {
                    *super::QUOTE == kind_id
                }

                pub fn REF(kind_id: u16) -> bool {
                    *super::REF == kind_id
                }

                pub fn REGISTER(kind_id: u16) -> bool {
                    *super::REGISTER == kind_id
                }

                pub fn RESULT(kind_id: u16) -> bool {
                    *super::RESULT == kind_id
                }

                pub fn REVERSE_SOLIDUS_REVERSE_SOLIDUS(kind_id: u16) -> bool {
                    *super::REVERSE_SOLIDUS_REVERSE_SOLIDUS == kind_id
                }

                pub fn RPAREN(kind_id: u16) -> bool {
                    *super::RPAREN == kind_id
                }

                pub fn SEMICOLON_SEMICOLON(kind_id: u16) -> bool {
                    *super::SEMICOLON_SEMICOLON == kind_id
                }

                pub fn TABLE(kind_id: u16) -> bool {
                    *super::TABLE == kind_id
                }

                pub fn THEN(kind_id: u16) -> bool {
                    *super::THEN == kind_id
                }

                pub fn TYPE(kind_id: u16) -> bool {
                    *super::TYPE == kind_id
                }

                pub fn V128(kind_id: u16) -> bool {
                    *super::V128 == kind_id
                }
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

        use super::language;
        use lazy_static::lazy_static;

        lazy_static! {
            pub static ref IDENTIFIER: u16 = language().field_id_for_name("identifier").unwrap();
        }
    }

    pub mod kind {
        #![allow(missing_docs)]

        use super::language;
        use lazy_static::lazy_static;

        lazy_static! {
            pub static ref ANNOTATION: u16 = language().id_for_node_kind("annotation", true);
            pub static ref COMMENT_BLOCK_ANNOT: u16 = language().id_for_node_kind("comment_block_annot", true);
            pub static ref COMMENT_BLOCK: u16 = language().id_for_node_kind("comment_block", true);
            pub static ref COMMENT_LINE_ANNOT: u16 = language().id_for_node_kind("comment_line_annot", true);
            pub static ref COMMENT_LINE: u16 = language().id_for_node_kind("comment_line", true);
            pub static ref EXPORT: u16 = language().id_for_node_kind("export", true);
            pub static ref IDENTIFIER: u16 = language().id_for_node_kind("identifier", true);
            pub static ref IMPORT: u16 = language().id_for_node_kind("import", true);
            pub static ref INDEX: u16 = language().id_for_node_kind("index", true);
            pub static ref INSTR_PLAIN: u16 = language().id_for_node_kind("instr_plain", true);
            pub static ref INSTR: u16 = language().id_for_node_kind("instr", true);
            pub static ref LPAREN: u16 = language().id_for_node_kind("(", false);
            pub static ref MODULE_FIELD_DATA: u16 = language().id_for_node_kind("module_field_data", true);
            pub static ref MODULE_FIELD_ELEM: u16 = language().id_for_node_kind("module_field_elem", true);
            pub static ref MODULE_FIELD_EXPORT: u16 = language().id_for_node_kind("module_field_export", true);
            pub static ref MODULE_FIELD_FUNC: u16 = language().id_for_node_kind("module_field_func", true);
            pub static ref MODULE_FIELD_GLOBAL: u16 = language().id_for_node_kind("module_field_global", true);
            pub static ref MODULE_FIELD_IMPORT: u16 = language().id_for_node_kind("module_field_import", true);
            pub static ref MODULE_FIELD_MEMORY: u16 = language().id_for_node_kind("module_field_memory", true);
            pub static ref MODULE_FIELD_START: u16 = language().id_for_node_kind("module_field_start", true);
            pub static ref MODULE_FIELD_TABLE: u16 = language().id_for_node_kind("module_field_table", true);
            pub static ref MODULE_FIELD_TYPE: u16 = language().id_for_node_kind("module_field_type", true);
            pub static ref MODULE: u16 = language().id_for_node_kind("module", true);
            pub static ref NAME: u16 = language().id_for_node_kind("name", true);
            pub static ref ROOT: u16 = language().id_for_node_kind("ROOT", true);
            pub static ref RPAREN: u16 = language().id_for_node_kind(")", false);
            pub static ref TYPE_USE: u16 = language().id_for_node_kind("type_use", true);
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
