//! Core functionality related to document languages.

use std::convert::TryFrom;
use wasm_language_server_shared::core::error::{Error, Fallible};

/// Languages supported by the server.
#[derive(Clone, Copy, Debug)]
pub enum Language {
    /// The `.wast` language.
    Wast,
    /// The `.wat` language.
    Wat,
    /// The `.witx` language.
    Wit,
    /// The `.witx` language.
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
            _ => Err(Error::CoreInvalidLanguageId(language_id).into()),
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
            pub static ref COMMAND: u16 = super::language().field_id_for_name("command").unwrap();
            pub static ref IDENTIFIER: u16 = super::language().field_id_for_name("identifier").unwrap();
            pub static ref MODULE_FIELD: u16 = super::language().field_id_for_name("module_field").unwrap();
        }
    }

    pub mod kind {
        #![allow(missing_docs)]

        use lazy_static::lazy_static;

        lazy_static! {
            pub static ref COMMAND: u16 = super::language().id_for_node_kind("command", true);
            pub static ref COMMENT_BLOCK_ANNOT: u16 = super::language().id_for_node_kind("comment_block_annot", true);
            pub static ref COMMENT_BLOCK: u16 = super::language().id_for_node_kind("comment_block", true);
            pub static ref COMMENT_LINE_ANNOT: u16 = super::language().id_for_node_kind("comment_line_annot", true);
            pub static ref COMMENT_LINE: u16 = super::language().id_for_node_kind("comment_line", true);
            pub static ref INLINE_MODULE: u16 = super::language().id_for_node_kind("inline_module", true);
            pub static ref INSTR: u16 = super::language().id_for_node_kind("instr", true);
            pub static ref INSTR_PLAIN: u16 = super::language().id_for_node_kind("instr_plain", true);
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

        pub fn is_comment(kind_id: &u16) -> bool {
            [*COMMENT_BLOCK_ANNOT, *COMMENT_BLOCK, *COMMENT_LINE_ANNOT, *COMMENT_LINE].contains(kind_id)
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
            pub static ref MODULE_FIELD: u16 = super::language().field_id_for_name("module_field").unwrap();
        }
    }

    pub mod kind {
        #![allow(missing_docs)]

        use lazy_static::lazy_static;

        lazy_static! {
            pub static ref COMMENT_BLOCK_ANNOT: u16 = super::language().id_for_node_kind("comment_block_annot", true);
            pub static ref COMMENT_BLOCK: u16 = super::language().id_for_node_kind("comment_block", true);
            pub static ref COMMENT_LINE_ANNOT: u16 = super::language().id_for_node_kind("comment_line_annot", true);
            pub static ref COMMENT_LINE: u16 = super::language().id_for_node_kind("comment_line", true);
            pub static ref INLINE_MODULE: u16 = super::language().id_for_node_kind("inline_module", true);
            pub static ref INSTR: u16 = super::language().id_for_node_kind("instr", true);
            pub static ref INSTR_PLAIN: u16 = super::language().id_for_node_kind("instr_plain", true);
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

        pub fn is_comment(kind_id: &u16) -> bool {
            [*COMMENT_BLOCK_ANNOT, *COMMENT_BLOCK, *COMMENT_LINE_ANNOT, *COMMENT_LINE].contains(kind_id)
        }
    }
}

/// Functions for working with the `.wit` grammar.
pub mod wit {
    /// Tree-sitter language for the `.wit` grammar.
    #[allow(unsafe_code)]
    pub fn language() -> tree_sitter::Language {
        unsafe { crate::tree_sitter_wit() }
    }
}

/// Functions for working with the `.witx` grammar.
pub mod witx {
    /// Tree-sitter language for the `.witx` grammar.
    #[allow(unsafe_code)]
    pub fn language() -> tree_sitter::Language {
        unsafe { crate::tree_sitter_witx() }
    }
}
