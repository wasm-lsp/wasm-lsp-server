//! Functions for working with the `.wast` grammar.

use crate::node::{NodeWalker, SyntaxErrors};

pub mod field {
    #![allow(missing_docs)]

    wasm_lsp_macros::field_ids! {
        language: "wasm.wast",
        fields: [
            (IDENTIFIER, "identifier"),
        ],
    }
}

pub mod kind {
    #![allow(missing_docs)]

    wasm_lsp_macros::node_kind_ids! {
        language: "wasm.wast",
        node_kinds: [
            (ACTION_GET, "action_get", true),
            (ACTION_INVOKE, "action_invoke", true),
            (ACTION, "action", true),
            (ALIGN_OFFSET_VALUE, "align_offset_value", true),
            (ALIGN_VALUE, "align_value", true),
            (ANNOTATION_PARENS, "annotation_parens", true),
            (ANNOTATION_PART, "annotation_part", true),
            (ANNOTATION, "annotation", true),
            (ASSERT_EXHAUSTION, "assert_exhaustion", true),
            (ASSERT_INVALID, "assert_invalid", true),
            (ASSERT_MALFORMED, "assert_malformed", true),
            (ASSERT_RETURN_ARITHMETIC_NAN, "assert_return_arithmetic_nan", true),
            (ASSERT_RETURN_CANONICAL_NAN, "assert_return_canonical_nan", true),
            (ASSERT_RETURN, "assert_return", true),
            (ASSERT_TRAP_ACTION, "assert_trap_action", true),
            (ASSERT_TRAP_MODULE, "assert_trap_module", true),
            (ASSERT_UNLINKABLE, "assert_unlinkable", true),
            (ASSERTION, "assertion", true),
            (BLOCK_BLOCK, "block_block", true),
            (BLOCK_IF, "block_if", true),
            (BLOCK_LOOP, "block_loop", true),
            (COMMAND, "command", true),
            (COMMENT_BLOCK_ANNOT, "comment_block_annot", true),
            (COMMENT_BLOCK, "comment_block", true),
            (COMMENT_LINE_ANNOT, "comment_line_annot", true),
            (COMMENT_LINE, "comment_line", true),
            (DEC_FLOAT, "dec_float", true),
            (DEC_NAT, "dec_nat", true),
            (ELEM_EXPR_EXPR, "elem_expr_expr", true),
            (ELEM_EXPR_ITEM, "elem_expr_item", true),
            (ELEM_EXPR, "elem_expr", true),
            (ELEM_KIND, "elem_kind", true),
            (ELEM_LIST, "elem_list", true),
            (ESCAPE_SEQUENCE, "escape_sequence", true),
            (EXPORT_DESC_FUNC, "export_desc_func", true),
            (EXPORT_DESC_GLOBAL, "export_desc_global", true),
            (EXPORT_DESC_MEMORY, "export_desc_memory", true),
            (EXPORT_DESC_TABLE, "export_desc_table", true),
            (EXPORT_DESC, "export_desc", true),
            (EXPORT, "export", true),
            (EXPR_PLAIN_CONST, "expr_plain_const", true),
            (EXPR, "expr", true),
            (EXPR1_BLOCK, "expr1_block", true),
            (EXPR1_CALL, "expr1_call", true),
            (EXPR1_IF, "expr1_if", true),
            (EXPR1_LOOP, "expr1_loop", true),
            (EXPR1_PLAIN, "expr1_plain", true),
            (EXPR1, "expr1", true),
            (FLOAT, "float", true),
            (FUNC_LOCALS_MANY, "func_locals_many", true),
            (FUNC_LOCALS_ONE, "func_locals_one", true),
            (FUNC_LOCALS, "func_locals", true),
            (FUNC_TYPE_PARAMS_MANY, "func_type_params_many", true),
            (FUNC_TYPE_PARAMS_ONE, "func_type_params_one", true),
            (FUNC_TYPE_PARAMS, "func_type_params", true),
            (FUNC_TYPE_RESULTS, "func_type_results", true),
            (FUNC_TYPE, "func_type", true),
            (GLOBAL_TYPE_IMM, "global_type_imm", true),
            (GLOBAL_TYPE_MUT, "global_type_mut", true),
            (GLOBAL_TYPE, "global_type", true),
            (HEX_FLOAT, "hex_float", true),
            (HEX_NAT, "hex_nat", true),
            (IDENTIFIER, "identifier", true),
            (IF_BLOCK, "if_block", true),
            (IMPORT_DESC_FUNC_TYPE, "import_desc_func_type", true),
            (IMPORT_DESC_GLOBAL_TYPE, "import_desc_global_type", true),
            (IMPORT_DESC_MEMORY_TYPE, "import_desc_memory_type", true),
            (IMPORT_DESC_TABLE_TYPE, "import_desc_table_type", true),
            (IMPORT_DESC_TYPE_USE, "import_desc_type_use", true),
            (IMPORT_DESC, "import_desc", true),
            (IMPORT, "import", true),
            (INDEX, "index", true),
            (INSTR_BLOCK, "instr_block", true),
            (INSTR_CALL, "instr_call", true),
            (INSTR_LIST, "instr_list", true),
            (INSTR_PLAIN, "instr_plain", true),
            (INSTR, "instr", true),
            (INT, "int", true),
            (LIMITS, "limits", true),
            (LITERAL_NAN_ARITHMETIC, "literal_nan_arithmetic", true),
            (LITERAL_NAN_CANONICAL, "literal_nan_canonical", true),
            (LITERAL_NAN, "literal_nan", true),
            (MEMORY_FIELDS_DATA, "memory_fields_data", true),
            (MEMORY_FIELDS_TYPE, "memory_fields_type", true),
            (MEMORY_TYPE, "memory_type", true),
            (MEMORY_USE, "memory_use", true),
            (META_INPUT, "meta_input", true),
            (META_OUTPUT, "meta_output", true),
            (META_SCRIPT, "meta_script", true),
            (META, "meta", true),
            (MODULE_FIELD_DATA, "module_field_data", true),
            (MODULE_FIELD_ELEM, "module_field_elem", true),
            (MODULE_FIELD_EXPORT, "module_field_export", true),
            (MODULE_FIELD_FUNC, "module_field_func", true),
            (MODULE_FIELD_GLOBAL, "module_field_global", true),
            (MODULE_FIELD_IMPORT, "module_field_import", true),
            (MODULE_FIELD_MEMORY, "module_field_memory", true),
            (MODULE_FIELD_START, "module_field_start", true),
            (MODULE_FIELD_TABLE, "module_field_table", true),
            (MODULE_FIELD_TYPE, "module_field_type", true),
            (MODULE_FIELD, "module_field", true),
            (MODULE, "module", true),
            (NAME, "name", true),
            (NAN, "nan", true),
            (NAT, "nat", true),
            (NUM_TYPE_F32, "num_type_f32", true),
            (NUM_TYPE_F64, "num_type_f64", true),
            (NUM_TYPE_I32, "num_type_i32", true),
            (NUM_TYPE_I64, "num_type_i64", true),
            (NUM_TYPE_V128, "num_type_v128", true),
            (NUM, "num", true),
            (OFFSET_CONST_EXPR, "offset_const_expr", true),
            (OFFSET_EXPR, "offset_expr", true),
            (OFFSET_VALUE, "offset_value", true),
            (OFFSET, "offset", true),
            (OP_CONST_REF, "op_const_ref", true),
            (OP_CONST, "op_const", true),
            (OP_FUNC_BIND, "op_func_bind", true),
            (OP_INDEX_OPT_OFFSET_OPT_ALIGN_OPT, "op_index_opt_offset_opt_align_opt", true),
            (OP_INDEX_OPT, "op_index_opt", true),
            (OP_INDEX, "op_index", true),
            (OP_LET, "op_let", true),
            (OP_NULLARY, "op_nullary", true),
            (OP_SELECT, "op_select", true),
            (OP_SIMD_CONST, "op_simd_const", true),
            (OP_SIMD_LANE, "op_simd_lane", true),
            (OP_SIMD_OFFSET_OPT_ALIGN_OPT, "opt_simd_offset_opt_align_opt", true),
            (OP_TABLE_COPY, "op_table_copy", true),
            (OP_TABLE_INIT, "op_table_init", true),
            (REF_KIND, "ref_kind", true),
            (REF_TYPE_EXTERNREF, "ref_type_externref", true),
            (REF_TYPE_FUNCREF, "ref_type_funcref", true),
            (REF_TYPE_REF, "ref_type_ref", true),
            (REF_TYPE, "ref_type", true),
            (REGISTER, "register", true),
            (RESERVED, "reserved", true),
            (RESULT_CONST_NAN, "result_const_nan", true),
            (RESULT_CONST, "result_const", true),
            (RESULT_REF_EXTERN, "result_ref_extern", true),
            (RESULT_REF_FUNC, "result_ref_func", true),
            (RESULT_REF_NULL, "result_ref_null", true),
            (RESULT, "result", true),
            (ROOT, "ROOT", true),
            (SCRIPT_MODULE_BINARY, "script_module_binary", true),
            (SCRIPT_MODULE_QUOTE, "script_module_quote", true),
            (SCRIPT_MODULE, "script_module", true),
            (SHARE, "share", true),
            (STRING, "string", true),
            (TABLE_FIELDS_ELEM, "table_fields_elem", true),
            (TABLE_FIELDS_TYPE, "table_fields_type", true),
            (TABLE_TYPE, "table_type", true),
            (TABLE_USE, "table_use", true),
            (TYPE_FIELD, "type_field", true),
            (TYPE_USE, "type_use", true),
            (VALUE_TYPE_NUM_TYPE, "value_type_num_type", true),
            (VALUE_TYPE_REF_TYPE, "value_type_ref_type", true),
            (VALUE_TYPE, "value_type", true),
        ]
    }

    pub mod token {
        #![allow(missing_docs)]

        wasm_lsp_macros::node_kind_ids! {
            language: "wasm.wast",
            node_kinds: [
                (ALIGN, "align", false),
                (ASSERT_EXHAUSTION, "assert_exhaustion", false),
                (ASSERT_INVALID, "assert_invalid", false),
                (ASSERT_MALFORMED, "assert_malformed", false),
                (ASSERT_RETURN_ARITHMETIC_NAN, "assert_return_arithmetic_nan", false),
                (ASSERT_RETURN_CANONICAL_NAN, "assert_return_canonical_nan", false),
                (ASSERT_RETURN, "assert_return", false),
                (ASSERT_TRAP, "assert_trap", false),
                (ASSERT_UNLINKABLE, "assert_unlinkable", false),
                (BINARY, "binary", false),
                (BLOCK, "block", false),
                (BR_TABLE, "br_table", false),
                (CALL_INDIRECT, "call_indirect", false),
                (DATA, "data", false),
                (DECLARE, "declare", false),
                (DOLLAR_SIGN, "$", false),
                (ELEM, "elem", false),
                (ELSE, "else", false),
                (END, "end", false),
                (EQUALS, "=", false),
                (EXPORT, "export", false),
                (EXTERNREF, "externref", false),
                (F32, "f32", false),
                (F64, "f64", false),
                (FULL_STOP, ".", false),
                (FUNC, "func", false),
                (FUNCREF, "funcref", false),
                (GET, "get", false),
                (GLOBAL, "global", false),
                (I32, "i32", false),
                (I64, "i64", false),
                (IF, "if", false),
                (IMPORT, "import", false),
                (INF, "inf", false),
                (INPUT, "input", false),
                (INVOKE, "invoke", false),
                (ITEM, "item", false),
                (LOCAL, "local", false),
                (LOOP, "loop", false),
                (LPAREN_AMPERSAND, "(@", false),
                (LPAREN_SEMICOLON, "(;", false),
                (LPAREN, "(", false),
                (MEMORY, "memory", false),
                (MODULE, "module", false),
                (MUT, "mut", false),
                (OFFSET, "offset", false),
                (OUTPUT, "output", false),
                (PARAM, "param", false),
                (QUOTE, "quote", false),
                (REF, "ref", false),
                (REGISTER, "register", false),
                (RESULT, "result", false),
                (REVERSE_SOLIDUS_REVERSE_SOLIDUS, "\\", false),
                (RPAREN, ")", false),
                (SCRIPT, "script", false),
                (SEMICOLON_SEMICOLON, ";;", false),
                (TABLE, "table", false),
                (THEN, "then", false),
                (TYPE, "type", false),
                (V128, "v128", false),
            ],
        }
    }
}

pub mod grouped {
    #![allow(missing_docs)]

    pub const MODULE_FIELDS: &[u16] = &[
        super::kind::MODULE_FIELD_DATA,
        super::kind::MODULE_FIELD_ELEM,
        super::kind::MODULE_FIELD_FUNC,
        super::kind::MODULE_FIELD_GLOBAL,
        super::kind::MODULE_FIELD_MEMORY,
        super::kind::MODULE_FIELD_TABLE,
        super::kind::MODULE_FIELD_TYPE,
    ];
}

#[allow(missing_docs)]
pub trait Visit<'tree,  C: crate::node::Context<'tree> + 'tree> {
    fn walker(&mut self) -> &'tree mut NodeWalker<'tree, C>;

    fn visit_action_get(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::action_get(self, node)
    }

    fn visit_action_invoke(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::action_invoke(self, node)
    }

    fn visit_action(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::action(self, node)
    }

    fn visit_align_offset_value(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::align_offset_value(self, node)
    }

    fn visit_align_value(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::align_value(self, node)
    }

    fn visit_annotation_parens(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::annotation_parens(self, node)
    }

    fn visit_annotation_part(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::annotation_part(self, node)
    }

    fn visit_annotation(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::annotation(self, node)
    }

    fn visit_assert_exhaustion(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::assert_exhaustion(self, node)
    }

    fn visit_assert_invalid(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::assert_invalid(self, node)
    }

    fn visit_assert_malformed(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::assert_malformed(self, node)
    }

    fn visit_assert_return_arithmetic_nan(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::assert_return_arithmetic_nan(self, node)
    }

    fn visit_assert_return_canonical_nan(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::assert_return_canonical_nan(self, node)
    }

    fn visit_assert_return(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::assert_return(self, node)
    }

    fn visit_assert_trap_action(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::assert_trap_action(self, node)
    }

    fn visit_assert_trap_module(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::assert_trap_module(self, node)
    }

    fn visit_assert_unlinkable(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::assert_unlinkable(self, node)
    }

    fn visit_assertion(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::assertion(self, node)
    }

    fn visit_block_block(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::block_block(self, node)
    }

    fn visit_block_if(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::block_if(self, node)
    }

    fn visit_block_loop(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::block_loop(self, node)
    }

    fn visit_command(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::command(self, node)
    }

    fn visit_comment_block_annot(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::comment_block_annot(self, node)
    }

    fn visit_comment_block(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::comment_block(self, node)
    }

    fn visit_comment_line_annot(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::comment_line_annot(self, node)
    }

    fn visit_comment_line(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::comment_line(self, node)
    }

    fn visit_dec_float(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::dec_float(self, node)
    }

    fn visit_dec_nat(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::dec_nat(self, node)
    }

    fn visit_elem_expr_expr(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::elem_expr_expr(self, node)
    }

    fn visit_elem_expr_item(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::elem_expr_item(self, node)
    }

    fn visit_elem_expr(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::elem_expr(self, node)
    }

    fn visit_elem_kind(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::elem_kind(self, node)
    }

    fn visit_elem_list(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::elem_list(self, node)
    }

    fn visit_escape_sequence(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::escape_sequence(self, node)
    }

    fn visit_export_desc_func(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::export_desc_func(self, node)
    }

    fn visit_export_desc_global(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::export_desc_global(self, node)
    }

    fn visit_export_desc_memory(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::export_desc_memory(self, node)
    }

    fn visit_export_desc_table(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::export_desc_table(self, node)
    }

    fn visit_export_desc(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::export_desc(self, node)
    }

    fn visit_export(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::export(self, node)
    }

    fn visit_expr_plain_const(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::expr_plain_const(self, node)
    }

    fn visit_expr(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::expr(self, node)
    }

    fn visit_expr1_block(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::expr1_block(self, node)
    }

    fn visit_expr1_call(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::expr1_call(self, node)
    }

    fn visit_expr1_if(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::expr1_if(self, node)
    }

    fn visit_expr1_loop(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::expr1_loop(self, node)
    }

    fn visit_expr1_plain(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::expr1_plain(self, node)
    }

    fn visit_expr1(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::expr1(self, node)
    }

    fn visit_float(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::float(self, node)
    }

    fn visit_func_locals_many(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::func_locals_many(self, node)
    }

    fn visit_func_locals_one(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::func_locals_one(self, node)
    }

    fn visit_func_locals(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::func_locals(self, node)
    }

    fn visit_func_type_params_many(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::func_type_params_many(self, node)
    }

    fn visit_func_type_params_one(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::func_type_params_one(self, node)
    }

    fn visit_func_type_params(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::func_type_params(self, node)
    }

    fn visit_func_type_results(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::func_type_results(self, node)
    }

    fn visit_func_type(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::func_type(self, node)
    }

    fn visit_global_type_imm(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::global_type_imm(self, node)
    }

    fn visit_global_type_mut(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::global_type_mut(self, node)
    }

    fn visit_global_type(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::global_type(self, node)
    }

    fn visit_hex_float(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::hex_float(self, node)
    }

    fn visit_hex_nat(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::hex_nat(self, node)
    }

    fn visit_identifier(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::identifier(self, node)
    }

    fn visit_if_block(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::if_block(self, node)
    }

    fn visit_import_desc_func_type(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::import_desc_func_type(self, node)
    }

    fn visit_import_desc_global_type(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::import_desc_global_type(self, node)
    }

    fn visit_import_desc_memory_type(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::import_desc_memory_type(self, node)
    }

    fn visit_import_desc_table_type(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::import_desc_table_type(self, node)
    }

    fn visit_import_desc_type_use(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::import_desc_type_use(self, node)
    }

    fn visit_import_desc(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::import_desc(self, node)
    }

    fn visit_import(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::import(self, node)
    }

    fn visit_index(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::index(self, node)
    }

    fn visit_instr_block(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::instr_block(self, node)
    }

    fn visit_instr_call(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::instr_call(self, node)
    }

    fn visit_instr_list(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::instr_list(self, node)
    }

    fn visit_instr_plain(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::instr_plain(self, node)
    }

    fn visit_instr(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::instr(self, node)
    }

    fn visit_int(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::int(self, node)
    }

    fn visit_limits(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::limits(self, node)
    }

    fn visit_literal_nan_arithmetic(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::literal_nan_arithmetic(self, node)
    }

    fn visit_literal_nan_canonical(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::literal_nan_canonical(self, node)
    }

    fn visit_literal_nan(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::literal_nan(self, node)
    }

    fn visit_memory_fields_data(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::memory_fields_data(self, node)
    }

    fn visit_memory_fields_type(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::memory_fields_type(self, node)
    }

    fn visit_memory_type(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::memory_type(self, node)
    }

    fn visit_memory_use(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::memory_use(self, node)
    }

    fn visit_meta_input(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::meta_input(self, node)
    }

    fn visit_meta_output(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::meta_output(self, node)
    }

    fn visit_meta_script(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::meta_script(self, node)
    }

    fn visit_meta(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::meta(self, node)
    }

    fn visit_module_field_data(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::module_field_data(self, node)
    }

    fn visit_module_field_elem(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::module_field_elem(self, node)
    }

    fn visit_module_field_export(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::module_field_export(self, node)
    }

    fn visit_module_field_func(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::module_field_func(self, node)
    }

    fn visit_module_field_global(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::module_field_global(self, node)
    }

    fn visit_module_field_import(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::module_field_import(self, node)
    }

    fn visit_module_field_memory(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::module_field_memory(self, node)
    }

    fn visit_module_field_start(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::module_field_start(self, node)
    }

    fn visit_module_field_table(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::module_field_table(self, node)
    }

    fn visit_module_field_type(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::module_field_type(self, node)
    }

    fn visit_module_field(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::module_field(self, node)
    }

    fn visit_module(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::module(self, node)
    }

    fn visit_name(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::name(self, node)
    }

    fn visit_nan(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::nan(self, node)
    }

    fn visit_nat(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::nat(self, node)
    }

    fn visit_num_type_f32(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::num_type_f32(self, node)
    }

    fn visit_num_type_f64(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::num_type_f64(self, node)
    }

    fn visit_num_type_i32(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::num_type_i32(self, node)
    }

    fn visit_num_type_i64(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::num_type_i64(self, node)
    }

    fn visit_num_type_v128(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::num_type_v128(self, node)
    }

    fn visit_num(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::num(self, node)
    }

    fn visit_offset_const_expr(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::offset_const_expr(self, node)
    }

    fn visit_offset_expr(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::offset_expr(self, node)
    }

    fn visit_offset_value(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::offset_value(self, node)
    }

    fn visit_offset(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::offset(self, node)
    }

    fn visit_op_const_ref(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::op_const_ref(self, node)
    }

    fn visit_op_const(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::op_const(self, node)
    }

    fn visit_op_func_bind(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::op_func_bind(self, node)
    }

    fn visit_op_index_opt_offset_opt_align_opt(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::op_index_opt_offset_opt_align_opt(self, node)
    }

    fn visit_op_index_opt(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::op_index_opt(self, node)
    }

    fn visit_op_index(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::op_index(self, node)
    }

    fn visit_op_let(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::op_let(self, node)
    }

    fn visit_op_nullary(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::op_nullary(self, node)
    }

    fn visit_op_select(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::op_select(self, node)
    }

    fn visit_op_simd_const(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::op_simd_const(self, node)
    }

    fn visit_op_simd_lane(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::op_simd_lane(self, node)
    }

    fn visit_op_simd_offset_opt_align_opt(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::op_simd_offset_opt_align_opt(self, node)
    }

    fn visit_op_table_copy(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::op_table_copy(self, node)
    }

    fn visit_op_table_init(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::op_table_init(self, node)
    }

    fn visit_ref_kind(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::ref_kind(self, node)
    }

    fn visit_ref_type_externref(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::ref_type_externref(self, node)
    }

    fn visit_ref_type_funcref(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::ref_type_funcref(self, node)
    }

    fn visit_ref_type_ref(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::ref_type_ref(self, node)
    }

    fn visit_ref_type(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::ref_type(self, node)
    }

    fn visit_register(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::register(self, node)
    }

    fn visit_reserved(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::reserved(self, node)
    }

    fn visit_result_const_nan(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::result_const_nan(self, node)
    }

    fn visit_result_const(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::result_const(self, node)
    }

    fn visit_result_ref_extern(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::result_ref_extern(self, node)
    }

    fn visit_result_ref_func(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::result_ref_func(self, node)
    }

    fn visit_result_ref_null(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::result_ref_null(self, node)
    }

    fn visit_result(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::result(self, node)
    }

    fn visit_root(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::root(self, node)
    }

    fn visit_script_module_binary(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::script_module_binary(self, node)
    }

    fn visit_script_module_quote(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::script_module_quote(self, node)
    }

    fn visit_script_module(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::script_module(self, node)
    }

    fn visit_share(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::share(self, node)
    }

    fn visit_string(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::string(self, node)
    }

    fn visit_table_fields_elem(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::table_fields_elem(self, node)
    }

    fn visit_table_fields_type(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::table_fields_type(self, node)
    }

    fn visit_table_type(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::table_type(self, node)
    }

    fn visit_table_use(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::table_use(self, node)
    }

    fn visit_type_field(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::type_field(self, node)
    }

    fn visit_type_use(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::type_use(self, node)
    }

    fn visit_value_type_num_type(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::value_type_num_type(self, node)
    }

    fn visit_value_type_ref_type(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::value_type_ref_type(self, node)
    }

    fn visit_value_type(&mut self, node: &tree_sitter::Node) -> Result<(), SyntaxErrors> {
        visit::value_type(self, node)
    }
}

pub mod visit {
    #![allow(missing_docs)]
    #![allow(unused)]

    use super::*;
    use crate::node::{Context, SyntaxError};

    macro_rules! repeat {
        ($name:ident, $visitor:expr, $node:expr, $life:lifetime, $errors:expr) => {{
            let mut node: tree_sitter::Node<$life> = $node.clone();
            let mut success = false;
            loop {
                let result = module_field($visitor, &node);
                if result.is_ok() {
                    success = true;
                }
                if let Err(mut errs) = result {
                    if success {
                        $visitor.walker().reset(node.clone());
                        break;
                    }
                    $errors.append(&mut errs);
                    return Err($errors);
                }
                node = $visitor.walker().node();
            }
        }};
    }

    pub fn action_get<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn action_invoke<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn action<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn align_offset_value<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn align_value<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn annotation_parens<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn annotation_part<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn annotation<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn assert_exhaustion<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn assert_invalid<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn assert_malformed<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn assert_return_arithmetic_nan<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn assert_return_canonical_nan<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn assert_return<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn assert_trap_action<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn assert_trap_module<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn assert_unlinkable<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn assertion<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn block_block<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn block_if<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn block_loop<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn command<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn comment_block_annot<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn comment_block<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn comment_line_annot<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn comment_line<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn dec_float<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn dec_nat<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn elem_expr_expr<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn elem_expr_item<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn elem_expr<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn elem_kind<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn elem_list<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn escape_sequence<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn export_desc_func<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn export_desc_global<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn export_desc_memory<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn export_desc_table<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn export_desc<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn export<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn expr_plain_const<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn expr<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn expr1_block<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn expr1_call<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn expr1_if<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn expr1_loop<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn expr1_plain<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn expr1<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn float<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn func_locals_many<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn func_locals_one<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn func_locals<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn func_type_params_many<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn func_type_params_one<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn func_type_params<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn func_type_results<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn func_type<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn global_type_imm<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn global_type_mut<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn global_type<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn hex_float<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn hex_nat<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn identifier<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn if_block<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn import_desc_func_type<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn import_desc_global_type<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn import_desc_memory_type<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn import_desc_table_type<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn import_desc_type_use<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn import_desc<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn import<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn index<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn instr_block<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn instr_call<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn instr_list<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn instr_plain<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn instr<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn int<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn limits<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn literal_nan_arithmetic<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn literal_nan_canonical<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn literal_nan<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn memory_fields_data<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn memory_fields_type<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn memory_type<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn memory_use<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn meta_input<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn meta_output<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn meta_script<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn meta<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn module_field_data<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn module_field_elem<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn module_field_export<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn module_field_func<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn module_field_global<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn module_field_import<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn module_field_memory<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn module_field_start<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn module_field_table<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn module_field_type<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn module_field<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn module<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        let mut errors = SyntaxErrors::new();
        let mut walker = visitor.walker();

        walker.step(kind::token::LPAREN, false)?;
        walker.step(kind::token::MODULE, false)?;
        identifier(visitor, node);

        {
            let mut node: tree_sitter::Node<$life> = node.clone();
            let mut success = false;
            loop {
                let result = module_field($visitor, &node);
                if result.is_ok() {
                    success = true;
                }
                if let Err(mut errs) = result {
                    if success {
                        $visitor.walker().reset(node.clone());
                        break;
                    }
                    $errors.append(&mut errs);
                    return Err($errors);
                }
                node = $visitor.walker().node();
            }
        }
        repeat!(module_field, visitor, node, 'tree, errors);


        walker.step(kind::token::RPAREN, false)?;

        Ok(())
    }

    pub fn name<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn nan<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn nat<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn num_type_f32<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn num_type_f64<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn num_type_i32<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn num_type_i64<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn num_type_v128<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn num<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn offset_const_expr<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn offset_expr<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn offset_value<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn offset<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn op_const_ref<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn op_const<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn op_func_bind<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn op_index_opt_offset_opt_align_opt<'tree, C, V>(
        visitor: &mut V,
        node: &tree_sitter::Node,
    ) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn op_index_opt<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn op_index<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn op_let<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn op_nullary<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn op_select<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn op_simd_const<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn op_simd_lane<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn op_simd_offset_opt_align_opt<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn op_table_copy<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn op_table_init<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn ref_kind<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn ref_type_externref<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn ref_type_funcref<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn ref_type_ref<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn ref_type<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn register<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn reserved<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn result_const_nan<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn result_const<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn result_ref_extern<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn result_ref_func<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn result_ref_null<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn result<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    // pub fn root<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    // where
    //     C: Context<'tree> + 'tree,
    //     V: Visit<'tree, C> + ?Sized,
    // {
    //     let mut errors = SyntaxErrors::new();

    //     if let Err(mut errs) = module(visitor, node) {
    //         errors.append(&mut errs);
    //     } else {
    //         return Ok(());
    //     }

    //     let mut module_field_success = false;
    //     loop {
    //         if visitor.walker().done {
    //             if module_field_success {
    //                 return Ok(());
    //             } else {
    //                 break;
    //             }
    //         }
    //         if let Err(mut errs) = module_field(visitor, node) {
    //             errors.append(&mut errs);
    //             break;
    //         } else {
    //             module_field_success = true;
    //         }
    //     }

    //     Err(errors)
    // }

    pub fn root<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        let mut errors = SyntaxErrors::new();

        let mut command_success = false;
        loop {
            if visitor.walker().done {
                if command_success {
                    return Ok(());
                } else {
                    break;
                }
            }
            if let Err(mut errs) = module_field(visitor, node) {
                errors.append(&mut errs);
                break;
            } else {
                module_field_success = true;
            }
        }

        Err(errors)
    }

    pub fn script_module_binary<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn script_module_quote<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn script_module<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn share<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn string<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn table_fields_elem<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn table_fields_type<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn table_type<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn table_use<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn type_field<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn type_use<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn value_type_num_type<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn value_type_ref_type<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }

    pub fn value_type<'tree, C, V>(visitor: &mut V, node: &tree_sitter::Node) -> Result<(), SyntaxErrors>
    where
        C: Context<'tree> + 'tree,
        V: Visit<'tree, C> + ?Sized,
    {
        Ok(())
    }
}
