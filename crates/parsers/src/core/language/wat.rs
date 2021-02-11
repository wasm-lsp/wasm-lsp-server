use lazy_static::lazy_static;

#[cfg(not(target_arch = "wasm32"))]
#[allow(unsafe_code)]
pub fn language() -> tree_sitter::Language {
    let inner = unsafe { crate::tree_sitter_wat() };
    inner.into()
}

#[cfg(target_arch = "wasm32")]
#[allow(unsafe_code)]
pub fn language() -> tree_sitter::Language {
    use wasm_bindgen::JsCast;
    use wasm_bindgen_futures::JsFuture;
    let bytes: &[u8] = include_bytes!("../../../../vendor/tree-sitter-wasm/wat/tree-sitter-wat.wasm");
    let promise = web_tree_sitter_sys::Language::load_bytes(&bytes.into());
    let future = JsFuture::from(promise);
    let result = futures::future::block_on(future).unwrap();
    let inner = result.unchecked_into::<web_tree_sitter_sys::Language>();
    inner.into()
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
