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
    let bytes: &[u8] = include_bytes!("../../../../../vendor/tree-sitter-wasm/wat/tree-sitter-wat.wasm");
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
        pub static ref ALIGN_OFFSET_VALUE: u16 = language().id_for_node_kind("align_offset_value", true);
        pub static ref ALIGN_VALUE: u16 = language().id_for_node_kind("align_value", true);
        pub static ref ANNOTATION_PARENS: u16 = language().id_for_node_kind("annotation_parens", true);
        pub static ref ANNOTATION_PART: u16 = language().id_for_node_kind("annotation_part", true);
        pub static ref ANNOTATION: u16 = language().id_for_node_kind("annotation", true);
        pub static ref BLOCK_BLOCK: u16 = language().id_for_node_kind("block_block", true);
        pub static ref BLOCK_IF: u16 = language().id_for_node_kind("block_if", true);
        pub static ref BLOCK_LOOP: u16 = language().id_for_node_kind("block_loop", true);
        pub static ref COMMENT_BLOCK_ANNOT: u16 = language().id_for_node_kind("comment_block_annot", true);
        pub static ref COMMENT_BLOCK: u16 = language().id_for_node_kind("comment_block", true);
        pub static ref COMMENT_LINE_ANNOT: u16 = language().id_for_node_kind("comment_line_annot", true);
        pub static ref COMMENT_LINE: u16 = language().id_for_node_kind("comment_line", true);
        pub static ref DEC_FLOAT: u16 = language().id_for_node_kind("dec_float", true);
        pub static ref DEC_NAT: u16 = language().id_for_node_kind("dec_nat", true);
        pub static ref ELEM_EXPR_EXPR: u16 = language().id_for_node_kind("elem_expr_expr", true);
        pub static ref ELEM_EXPR_ITEM: u16 = language().id_for_node_kind("elem_expr_item", true);
        pub static ref ELEM_EXPR: u16 = language().id_for_node_kind("elem_expr", true);
        pub static ref ELEM_KIND: u16 = language().id_for_node_kind("elem_kind", true);
        pub static ref ELEM_LIST: u16 = language().id_for_node_kind("elem_list", true);
        pub static ref ESCAPE_SEQUENCE: u16 = language().id_for_node_kind("escape_sequence", true);
        pub static ref EXPORT_DESC_FUNC: u16 = language().id_for_node_kind("export_desc_func", true);
        pub static ref EXPORT_DESC_GLOBAL: u16 = language().id_for_node_kind("export_desc_global", true);
        pub static ref EXPORT_DESC_MEMORY: u16 = language().id_for_node_kind("export_desc_memory", true);
        pub static ref EXPORT_DESC_TABLE: u16 = language().id_for_node_kind("export_desc_table", true);
        pub static ref EXPORT_DESC: u16 = language().id_for_node_kind("export_desc", true);
        pub static ref EXPORT: u16 = language().id_for_node_kind("export", true);
        pub static ref EXPR: u16 = language().id_for_node_kind("expr", true);
        pub static ref EXPR1_BLOCK: u16 = language().id_for_node_kind("expr1_block", true);
        pub static ref EXPR1_CALL: u16 = language().id_for_node_kind("expr1_call", true);
        pub static ref EXPR1_IF: u16 = language().id_for_node_kind("expr1_if", true);
        pub static ref EXPR1_LOOP: u16 = language().id_for_node_kind("expr1_loop", true);
        pub static ref EXPR1_PLAIN: u16 = language().id_for_node_kind("expr1_plain", true);
        pub static ref EXPR1: u16 = language().id_for_node_kind("expr1", true);
        pub static ref FLOAT: u16 = language().id_for_node_kind("float", true);
        pub static ref FUNC_LOCALS_MANY: u16 = language().id_for_node_kind("func_locals_many", true);
        pub static ref FUNC_LOCALS_ONE: u16 = language().id_for_node_kind("func_locals_one", true);
        pub static ref FUNC_LOCALS: u16 = language().id_for_node_kind("func_locals", true);
        pub static ref FUNC_TYPE_PARAMS_MANY: u16 = language().id_for_node_kind("func_type_params_many", true);
        pub static ref FUNC_TYPE_PARAMS_ONE: u16 = language().id_for_node_kind("func_type_params_one", true);
        pub static ref FUNC_TYPE_PARAMS: u16 = language().id_for_node_kind("func_type_params", true);
        pub static ref FUNC_TYPE_RESULTS: u16 = language().id_for_node_kind("func_type_results", true);
        pub static ref FUNC_TYPE: u16 = language().id_for_node_kind("func_type", true);
        pub static ref GLOBAL_TYPE_IMM: u16 = language().id_for_node_kind("global_type_imm", true);
        pub static ref GLOBAL_TYPE_MUT: u16 = language().id_for_node_kind("global_type_mut", true);
        pub static ref GLOBAL_TYPE: u16 = language().id_for_node_kind("global_type", true);
        pub static ref HEX_FLOAT: u16 = language().id_for_node_kind("hex_float", true);
        pub static ref HEX_NAT: u16 = language().id_for_node_kind("hex_nat", true);
        pub static ref IDENTIFIER: u16 = language().id_for_node_kind("identifier", true);
        pub static ref IF_BLOCK: u16 = language().id_for_node_kind("if_block", true);
        pub static ref IMPORT_DESC_FUNC_TYPE: u16 = language().id_for_node_kind("import_desc_func_type", true);
        pub static ref IMPORT_DESC_GLOBAL_TYPE: u16 = language().id_for_node_kind("import_desc_global_type", true);
        pub static ref IMPORT_DESC_MEMORY_TYPE: u16 = language().id_for_node_kind("import_desc_memory_type", true);
        pub static ref IMPORT_DESC_TABLE_TYPE: u16 = language().id_for_node_kind("import_desc_table_type", true);
        pub static ref IMPORT_DESC_TYPE_USE: u16 = language().id_for_node_kind("import_desc_type_use", true);
        pub static ref IMPORT_DESC: u16 = language().id_for_node_kind("import_desc", true);
        pub static ref IMPORT: u16 = language().id_for_node_kind("import", true);
        pub static ref INDEX: u16 = language().id_for_node_kind("index", true);
        pub static ref INSTR_BLOCK: u16 = language().id_for_node_kind("instr_block", true);
        pub static ref INSTR_CALL: u16 = language().id_for_node_kind("instr_call", true);
        pub static ref INSTR_LIST: u16 = language().id_for_node_kind("instr_list", true);
        pub static ref INSTR_PLAIN: u16 = language().id_for_node_kind("instr_plain", true);
        pub static ref INSTR: u16 = language().id_for_node_kind("instr", true);
        pub static ref INT: u16 = language().id_for_node_kind("int", true);
        pub static ref LIMITS: u16 = language().id_for_node_kind("limits", true);
        pub static ref MEMORY_FIELDS_DATA: u16 = language().id_for_node_kind("memory_fields_data", true);
        pub static ref MEMORY_FIELDS_TYPE: u16 = language().id_for_node_kind("memory_fields_type", true);
        pub static ref MEMORY_TYPE: u16 = language().id_for_node_kind("memory_type", true);
        pub static ref MEMORY_USE: u16 = language().id_for_node_kind("memory_use", true);
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
        pub static ref MODULE_FIELD: u16 = language().id_for_node_kind("module_field", true);
        pub static ref MODULE: u16 = language().id_for_node_kind("module", true);
        pub static ref NAME: u16 = language().id_for_node_kind("name", true);
        pub static ref NAN: u16 = language().id_for_node_kind("nan", true);
        pub static ref NAT: u16 = language().id_for_node_kind("nat", true);
        pub static ref NUM_TYPE_F32: u16 = language().id_for_node_kind("num_type_f32", true);
        pub static ref NUM_TYPE_F64: u16 = language().id_for_node_kind("num_type_f64", true);
        pub static ref NUM_TYPE_I32: u16 = language().id_for_node_kind("num_type_i32", true);
        pub static ref NUM_TYPE_I64: u16 = language().id_for_node_kind("num_type_i64", true);
        pub static ref NUM_TYPE_V128: u16 = language().id_for_node_kind("num_type_v128", true);
        pub static ref NUM: u16 = language().id_for_node_kind("num", true);
        pub static ref OFFSET_CONST_EXPR: u16 = language().id_for_node_kind("offset_const_expr", true);
        pub static ref OFFSET_EXPR: u16 = language().id_for_node_kind("offset_expr", true);
        pub static ref OFFSET_VALUE: u16 = language().id_for_node_kind("offset_value", true);
        pub static ref OFFSET: u16 = language().id_for_node_kind("offset", true);
        pub static ref OP_CONST: u16 = language().id_for_node_kind("op_const", true);
        pub static ref OP_FUNC_BIND: u16 = language().id_for_node_kind("op_func_bind", true);
        pub static ref OP_INDEX_OPT_OFFSET_OPT_ALIGN_OPT: u16 =
            language().id_for_node_kind("op_index_opt_offset_opt_align_opt", true);
        pub static ref OP_INDEX_OPT: u16 = language().id_for_node_kind("op_index_opt", true);
        pub static ref OP_INDEX: u16 = language().id_for_node_kind("op_index", true);
        pub static ref OP_LET: u16 = language().id_for_node_kind("op_let", true);
        pub static ref OP_NULLARY: u16 = language().id_for_node_kind("op_nullary", true);
        pub static ref OP_SELECT: u16 = language().id_for_node_kind("op_select", true);
        pub static ref OP_SIMD_CONST: u16 = language().id_for_node_kind("op_simd_const", true);
        pub static ref OP_SIMD_LANE: u16 = language().id_for_node_kind("op_simd_lane", true);
        pub static ref OP_SIMD_OFFSET_OPT_ALIGN_OPT: u16 =
            language().id_for_node_kind("opt_simd_offset_opt_align_opt", true);
        pub static ref OP_TABLE_COPY: u16 = language().id_for_node_kind("op_table_copy", true);
        pub static ref OP_TABLE_INIT: u16 = language().id_for_node_kind("op_table_init", true);
        pub static ref REF_KIND: u16 = language().id_for_node_kind("ref_kind", true);
        pub static ref REF_TYPE_EXTERNREF: u16 = language().id_for_node_kind("ref_type_externref", true);
        pub static ref REF_TYPE_FUNCREF: u16 = language().id_for_node_kind("ref_type_funcref", true);
        pub static ref REF_TYPE_REF: u16 = language().id_for_node_kind("ref_type_ref", true);
        pub static ref REF_TYPE: u16 = language().id_for_node_kind("ref_type", true);
        pub static ref RESERVED: u16 = language().id_for_node_kind("reserved", true);
        pub static ref ROOT: u16 = language().id_for_node_kind("ROOT", true);
        pub static ref SHARE: u16 = language().id_for_node_kind("share", true);
        pub static ref STRING: u16 = language().id_for_node_kind("string", true);
        pub static ref TABLE_FIELDS_ELEM: u16 = language().id_for_node_kind("table_fields_elem", true);
        pub static ref TABLE_FIELDS_TYPE: u16 = language().id_for_node_kind("table_fields_type", true);
        pub static ref TABLE_TYPE: u16 = language().id_for_node_kind("table_type", true);
        pub static ref TABLE_USE: u16 = language().id_for_node_kind("table_use", true);
        pub static ref TYPE_FIELD: u16 = language().id_for_node_kind("type_field", true);
        pub static ref TYPE_USE: u16 = language().id_for_node_kind("type_use", true);
        pub static ref VALUE_TYPE_NUM_TYPE: u16 = language().id_for_node_kind("value_type_num_type", true);
        pub static ref VALUE_TYPE_REF_TYPE: u16 = language().id_for_node_kind("value_type_ref_type", true);
        pub static ref VALUE_TYPE: u16 = language().id_for_node_kind("value_type", true);
    }

    lazy_static! {
        pub static ref LPAREN: u16 = language().id_for_node_kind("(", false);
        pub static ref RPAREN: u16 = language().id_for_node_kind(")", false);
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
    }
}

pub mod grouped {
    use lazy_static::lazy_static;

    lazy_static! {
        pub static ref MODULE_FIELDS: Vec<u16> = vec![
            *super::kind::MODULE_FIELD_DATA,
            *super::kind::MODULE_FIELD_ELEM,
            *super::kind::MODULE_FIELD_FUNC,
            *super::kind::MODULE_FIELD_GLOBAL,
            *super::kind::MODULE_FIELD_MEMORY,
            *super::kind::MODULE_FIELD_TABLE,
            *super::kind::MODULE_FIELD_TYPE,
        ];
    }
}
