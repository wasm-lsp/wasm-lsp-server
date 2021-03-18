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
                (CONST, "const", false),
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
                (NAN_ARITHMETIC, "nan:arithmetic", false),
                (NAN_CANONICAL, "nan:canonical", false),
                (OFFSET, "offset", false),
                (OUTPUT, "output", false),
                (PARAM, "param", false),
                (QUOTE, "quote", false),
                (REF_EXTERN, "ref.extern", false),
                (REF_FUNC, "ref.func", false),
                (REF_NULL, "ref.null", false),
                (REF, "ref", false),
                (REGISTER, "register", false),
                (RESULT, "result", false),
                (REVERSE_SOLIDUS_REVERSE_SOLIDUS, "\\", false),
                (RPAREN, ")", false),
                (SCRIPT, "script", false),
                (SEMICOLON_RPAREN, ";)", false),
                (SEMICOLON_SEMICOLON, ";;", false),
                (START, "start", false),
                (TABLE, "table", false),
                (THEN, "then", false),
                (TYPE, "type", false),
                (V128, "v128", false),
                (ZERO_X, "0x", false),
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
pub trait Visitor<'tree, Ctx: crate::node::Context<'tree> + 'tree> {
    fn walker(&mut self) -> &mut NodeWalker<'tree, Ctx>;

    fn node(&self) -> tree_sitter::Node<'tree>;

    fn reset(&mut self, node: tree_sitter::Node<'tree>);

    fn visit_action_get(&mut self) -> Result<(), SyntaxErrors> {
        visit::action_get(self)
    }

    fn visit_action_invoke(&mut self) -> Result<(), SyntaxErrors> {
        visit::action_invoke(self)
    }

    fn visit_action(&mut self) -> Result<(), SyntaxErrors> {
        visit::action(self)
    }

    fn visit_align_offset_value(&mut self) -> Result<(), SyntaxErrors> {
        visit::align_offset_value(self)
    }

    fn visit_align_value(&mut self) -> Result<(), SyntaxErrors> {
        visit::align_value(self)
    }

    fn visit_annotation_parens(&mut self) -> Result<(), SyntaxErrors> {
        visit::annotation_parens(self)
    }

    fn visit_annotation_part(&mut self) -> Result<(), SyntaxErrors> {
        visit::annotation_part(self)
    }

    fn visit_annotation(&mut self) -> Result<(), SyntaxErrors> {
        visit::annotation(self)
    }

    fn visit_assert_exhaustion(&mut self) -> Result<(), SyntaxErrors> {
        visit::assert_exhaustion(self)
    }

    fn visit_assert_invalid(&mut self) -> Result<(), SyntaxErrors> {
        visit::assert_invalid(self)
    }

    fn visit_assert_malformed(&mut self) -> Result<(), SyntaxErrors> {
        visit::assert_malformed(self)
    }

    fn visit_assert_return_arithmetic_nan(&mut self) -> Result<(), SyntaxErrors> {
        visit::assert_return_arithmetic_nan(self)
    }

    fn visit_assert_return_canonical_nan(&mut self) -> Result<(), SyntaxErrors> {
        visit::assert_return_canonical_nan(self)
    }

    fn visit_assert_return(&mut self) -> Result<(), SyntaxErrors> {
        visit::assert_return(self)
    }

    fn visit_assert_trap_action(&mut self) -> Result<(), SyntaxErrors> {
        visit::assert_trap_action(self)
    }

    fn visit_assert_trap_module(&mut self) -> Result<(), SyntaxErrors> {
        visit::assert_trap_module(self)
    }

    fn visit_assert_unlinkable(&mut self) -> Result<(), SyntaxErrors> {
        visit::assert_unlinkable(self)
    }

    fn visit_assertion(&mut self) -> Result<(), SyntaxErrors> {
        visit::assertion(self)
    }

    fn visit_block_block(&mut self) -> Result<(), SyntaxErrors> {
        visit::block_block(self)
    }

    fn visit_block_if(&mut self) -> Result<(), SyntaxErrors> {
        visit::block_if(self)
    }

    fn visit_block_loop(&mut self) -> Result<(), SyntaxErrors> {
        visit::block_loop(self)
    }

    fn visit_command(&mut self) -> Result<(), SyntaxErrors> {
        visit::command(self)
    }

    fn visit_comment_block_annot(&mut self) -> Result<(), SyntaxErrors> {
        visit::comment_block_annot(self)
    }

    fn visit_comment_block(&mut self) -> Result<(), SyntaxErrors> {
        visit::comment_block(self)
    }

    fn visit_comment_line_annot(&mut self) -> Result<(), SyntaxErrors> {
        visit::comment_line_annot(self)
    }

    fn visit_comment_line(&mut self) -> Result<(), SyntaxErrors> {
        visit::comment_line(self)
    }

    fn visit_dec_float(&mut self) -> Result<(), SyntaxErrors> {
        visit::dec_float(self)
    }

    fn visit_dec_nat(&mut self) -> Result<(), SyntaxErrors> {
        visit::dec_nat(self)
    }

    fn visit_elem_expr_expr(&mut self) -> Result<(), SyntaxErrors> {
        visit::elem_expr_expr(self)
    }

    fn visit_elem_expr_item(&mut self) -> Result<(), SyntaxErrors> {
        visit::elem_expr_item(self)
    }

    fn visit_elem_expr(&mut self) -> Result<(), SyntaxErrors> {
        visit::elem_expr(self)
    }

    fn visit_elem_kind(&mut self) -> Result<(), SyntaxErrors> {
        visit::elem_kind(self)
    }

    fn visit_elem_list(&mut self) -> Result<(), SyntaxErrors> {
        visit::elem_list(self)
    }

    fn visit_escape_sequence(&mut self) -> Result<(), SyntaxErrors> {
        visit::escape_sequence(self)
    }

    fn visit_export_desc_func(&mut self) -> Result<(), SyntaxErrors> {
        visit::export_desc_func(self)
    }

    fn visit_export_desc_global(&mut self) -> Result<(), SyntaxErrors> {
        visit::export_desc_global(self)
    }

    fn visit_export_desc_memory(&mut self) -> Result<(), SyntaxErrors> {
        visit::export_desc_memory(self)
    }

    fn visit_export_desc_table(&mut self) -> Result<(), SyntaxErrors> {
        visit::export_desc_table(self)
    }

    fn visit_export_desc(&mut self) -> Result<(), SyntaxErrors> {
        visit::export_desc(self)
    }

    fn visit_export(&mut self) -> Result<(), SyntaxErrors> {
        visit::export(self)
    }

    fn visit_expr_plain_const(&mut self) -> Result<(), SyntaxErrors> {
        visit::expr_plain_const(self)
    }

    fn visit_expr(&mut self) -> Result<(), SyntaxErrors> {
        visit::expr(self)
    }

    fn visit_expr1_block(&mut self) -> Result<(), SyntaxErrors> {
        visit::expr1_block(self)
    }

    fn visit_expr1_call(&mut self) -> Result<(), SyntaxErrors> {
        visit::expr1_call(self)
    }

    fn visit_expr1_if(&mut self) -> Result<(), SyntaxErrors> {
        visit::expr1_if(self)
    }

    fn visit_expr1_loop(&mut self) -> Result<(), SyntaxErrors> {
        visit::expr1_loop(self)
    }

    fn visit_expr1_plain(&mut self) -> Result<(), SyntaxErrors> {
        visit::expr1_plain(self)
    }

    fn visit_expr1(&mut self) -> Result<(), SyntaxErrors> {
        visit::expr1(self)
    }

    fn visit_float(&mut self) -> Result<(), SyntaxErrors> {
        visit::float(self)
    }

    fn visit_func_locals_many(&mut self) -> Result<(), SyntaxErrors> {
        visit::func_locals_many(self)
    }

    fn visit_func_locals_one(&mut self) -> Result<(), SyntaxErrors> {
        visit::func_locals_one(self)
    }

    fn visit_func_locals(&mut self) -> Result<(), SyntaxErrors> {
        visit::func_locals(self)
    }

    fn visit_func_type_params_many(&mut self) -> Result<(), SyntaxErrors> {
        visit::func_type_params_many(self)
    }

    fn visit_func_type_params_one(&mut self) -> Result<(), SyntaxErrors> {
        visit::func_type_params_one(self)
    }

    fn visit_func_type_params(&mut self) -> Result<(), SyntaxErrors> {
        visit::func_type_params(self)
    }

    fn visit_func_type_results(&mut self) -> Result<(), SyntaxErrors> {
        visit::func_type_results(self)
    }

    fn visit_func_type(&mut self) -> Result<(), SyntaxErrors> {
        visit::func_type(self)
    }

    fn visit_global_type_imm(&mut self) -> Result<(), SyntaxErrors> {
        visit::global_type_imm(self)
    }

    fn visit_global_type_mut(&mut self) -> Result<(), SyntaxErrors> {
        visit::global_type_mut(self)
    }

    fn visit_global_type(&mut self) -> Result<(), SyntaxErrors> {
        visit::global_type(self)
    }

    fn visit_hex_float(&mut self) -> Result<(), SyntaxErrors> {
        visit::hex_float(self)
    }

    fn visit_hex_nat(&mut self) -> Result<(), SyntaxErrors> {
        visit::hex_nat(self)
    }

    fn visit_identifier(&mut self) -> Result<(), SyntaxErrors> {
        visit::identifier(self)
    }

    fn visit_if_block(&mut self) -> Result<(), SyntaxErrors> {
        visit::if_block(self)
    }

    fn visit_import_desc_func_type(&mut self) -> Result<(), SyntaxErrors> {
        visit::import_desc_func_type(self)
    }

    fn visit_import_desc_global_type(&mut self) -> Result<(), SyntaxErrors> {
        visit::import_desc_global_type(self)
    }

    fn visit_import_desc_memory_type(&mut self) -> Result<(), SyntaxErrors> {
        visit::import_desc_memory_type(self)
    }

    fn visit_import_desc_table_type(&mut self) -> Result<(), SyntaxErrors> {
        visit::import_desc_table_type(self)
    }

    fn visit_import_desc_type_use(&mut self) -> Result<(), SyntaxErrors> {
        visit::import_desc_type_use(self)
    }

    fn visit_import_desc(&mut self) -> Result<(), SyntaxErrors> {
        visit::import_desc(self)
    }

    fn visit_import(&mut self) -> Result<(), SyntaxErrors> {
        visit::import(self)
    }

    fn visit_index(&mut self) -> Result<(), SyntaxErrors> {
        visit::index(self)
    }

    fn visit_instr_block(&mut self) -> Result<(), SyntaxErrors> {
        visit::instr_block(self)
    }

    fn visit_instr_call(&mut self) -> Result<(), SyntaxErrors> {
        visit::instr_call(self)
    }

    fn visit_instr_list(&mut self) -> Result<(), SyntaxErrors> {
        visit::instr_list(self)
    }

    fn visit_instr_plain(&mut self) -> Result<(), SyntaxErrors> {
        visit::instr_plain(self)
    }

    fn visit_instr(&mut self) -> Result<(), SyntaxErrors> {
        visit::instr(self)
    }

    fn visit_int(&mut self) -> Result<(), SyntaxErrors> {
        visit::int(self)
    }

    fn visit_limits(&mut self) -> Result<(), SyntaxErrors> {
        visit::limits(self)
    }

    fn visit_literal_nan_arithmetic(&mut self) -> Result<(), SyntaxErrors> {
        visit::literal_nan_arithmetic(self)
    }

    fn visit_literal_nan_canonical(&mut self) -> Result<(), SyntaxErrors> {
        visit::literal_nan_canonical(self)
    }

    fn visit_literal_nan(&mut self) -> Result<(), SyntaxErrors> {
        visit::literal_nan(self)
    }

    fn visit_memory_fields_data(&mut self) -> Result<(), SyntaxErrors> {
        visit::memory_fields_data(self)
    }

    fn visit_memory_fields_type(&mut self) -> Result<(), SyntaxErrors> {
        visit::memory_fields_type(self)
    }

    fn visit_memory_type(&mut self) -> Result<(), SyntaxErrors> {
        visit::memory_type(self)
    }

    fn visit_memory_use(&mut self) -> Result<(), SyntaxErrors> {
        visit::memory_use(self)
    }

    fn visit_meta_input(&mut self) -> Result<(), SyntaxErrors> {
        visit::meta_input(self)
    }

    fn visit_meta_output(&mut self) -> Result<(), SyntaxErrors> {
        visit::meta_output(self)
    }

    fn visit_meta_script(&mut self) -> Result<(), SyntaxErrors> {
        visit::meta_script(self)
    }

    fn visit_meta(&mut self) -> Result<(), SyntaxErrors> {
        visit::meta(self)
    }

    fn visit_module_field_data(&mut self) -> Result<(), SyntaxErrors> {
        visit::module_field_data(self)
    }

    fn visit_module_field_elem(&mut self) -> Result<(), SyntaxErrors> {
        visit::module_field_elem(self)
    }

    fn visit_module_field_export(&mut self) -> Result<(), SyntaxErrors> {
        visit::module_field_export(self)
    }

    fn visit_module_field_func(&mut self) -> Result<(), SyntaxErrors> {
        visit::module_field_func(self)
    }

    fn visit_module_field_global(&mut self) -> Result<(), SyntaxErrors> {
        visit::module_field_global(self)
    }

    fn visit_module_field_import(&mut self) -> Result<(), SyntaxErrors> {
        visit::module_field_import(self)
    }

    fn visit_module_field_memory(&mut self) -> Result<(), SyntaxErrors> {
        visit::module_field_memory(self)
    }

    fn visit_module_field_start(&mut self) -> Result<(), SyntaxErrors> {
        visit::module_field_start(self)
    }

    fn visit_module_field_table(&mut self) -> Result<(), SyntaxErrors> {
        visit::module_field_table(self)
    }

    fn visit_module_field_type(&mut self) -> Result<(), SyntaxErrors> {
        visit::module_field_type(self)
    }

    fn visit_module_field(&mut self) -> Result<(), SyntaxErrors> {
        visit::module_field(self)
    }

    fn visit_module(&mut self) -> Result<(), SyntaxErrors> {
        visit::module(self)
    }

    fn visit_name(&mut self) -> Result<(), SyntaxErrors> {
        visit::name(self)
    }

    fn visit_nan(&mut self) -> Result<(), SyntaxErrors> {
        visit::nan(self)
    }

    fn visit_nat(&mut self) -> Result<(), SyntaxErrors> {
        visit::nat(self)
    }

    fn visit_num_type_f32(&mut self) -> Result<(), SyntaxErrors> {
        visit::num_type_f32(self)
    }

    fn visit_num_type_f64(&mut self) -> Result<(), SyntaxErrors> {
        visit::num_type_f64(self)
    }

    fn visit_num_type_i32(&mut self) -> Result<(), SyntaxErrors> {
        visit::num_type_i32(self)
    }

    fn visit_num_type_i64(&mut self) -> Result<(), SyntaxErrors> {
        visit::num_type_i64(self)
    }

    fn visit_num_type_v128(&mut self) -> Result<(), SyntaxErrors> {
        visit::num_type_v128(self)
    }

    fn visit_num(&mut self) -> Result<(), SyntaxErrors> {
        visit::num(self)
    }

    fn visit_offset_const_expr(&mut self) -> Result<(), SyntaxErrors> {
        visit::offset_const_expr(self)
    }

    fn visit_offset_expr(&mut self) -> Result<(), SyntaxErrors> {
        visit::offset_expr(self)
    }

    fn visit_offset_value(&mut self) -> Result<(), SyntaxErrors> {
        visit::offset_value(self)
    }

    fn visit_offset(&mut self) -> Result<(), SyntaxErrors> {
        visit::offset(self)
    }

    fn visit_op_const_ref(&mut self) -> Result<(), SyntaxErrors> {
        visit::op_const_ref(self)
    }

    fn visit_op_const(&mut self) -> Result<(), SyntaxErrors> {
        visit::op_const(self)
    }

    fn visit_op_func_bind(&mut self) -> Result<(), SyntaxErrors> {
        visit::op_func_bind(self)
    }

    fn visit_op_index_opt_offset_opt_align_opt(&mut self) -> Result<(), SyntaxErrors> {
        visit::op_index_opt_offset_opt_align_opt(self)
    }

    fn visit_op_index_opt(&mut self) -> Result<(), SyntaxErrors> {
        visit::op_index_opt(self)
    }

    fn visit_op_index(&mut self) -> Result<(), SyntaxErrors> {
        visit::op_index(self)
    }

    fn visit_op_let(&mut self) -> Result<(), SyntaxErrors> {
        visit::op_let(self)
    }

    fn visit_op_nullary(&mut self) -> Result<(), SyntaxErrors> {
        visit::op_nullary(self)
    }

    fn visit_op_select(&mut self) -> Result<(), SyntaxErrors> {
        visit::op_select(self)
    }

    fn visit_op_simd_const(&mut self) -> Result<(), SyntaxErrors> {
        visit::op_simd_const(self)
    }

    fn visit_op_simd_lane(&mut self) -> Result<(), SyntaxErrors> {
        visit::op_simd_lane(self)
    }

    fn visit_op_simd_offset_opt_align_opt(&mut self) -> Result<(), SyntaxErrors> {
        visit::op_simd_offset_opt_align_opt(self)
    }

    fn visit_op_table_copy(&mut self) -> Result<(), SyntaxErrors> {
        visit::op_table_copy(self)
    }

    fn visit_op_table_init(&mut self) -> Result<(), SyntaxErrors> {
        visit::op_table_init(self)
    }

    fn visit_ref_kind(&mut self) -> Result<(), SyntaxErrors> {
        visit::ref_kind(self)
    }

    fn visit_ref_type_externref(&mut self) -> Result<(), SyntaxErrors> {
        visit::ref_type_externref(self)
    }

    fn visit_ref_type_funcref(&mut self) -> Result<(), SyntaxErrors> {
        visit::ref_type_funcref(self)
    }

    fn visit_ref_type_ref(&mut self) -> Result<(), SyntaxErrors> {
        visit::ref_type_ref(self)
    }

    fn visit_ref_type(&mut self) -> Result<(), SyntaxErrors> {
        visit::ref_type(self)
    }

    fn visit_register(&mut self) -> Result<(), SyntaxErrors> {
        visit::register(self)
    }

    fn visit_reserved(&mut self) -> Result<(), SyntaxErrors> {
        visit::reserved(self)
    }

    fn visit_result_const_nan(&mut self) -> Result<(), SyntaxErrors> {
        visit::result_const_nan(self)
    }

    fn visit_result_const(&mut self) -> Result<(), SyntaxErrors> {
        visit::result_const(self)
    }

    fn visit_result_ref_extern(&mut self) -> Result<(), SyntaxErrors> {
        visit::result_ref_extern(self)
    }

    fn visit_result_ref_func(&mut self) -> Result<(), SyntaxErrors> {
        visit::result_ref_func(self)
    }

    fn visit_result_ref_null(&mut self) -> Result<(), SyntaxErrors> {
        visit::result_ref_null(self)
    }

    fn visit_result(&mut self) -> Result<(), SyntaxErrors> {
        visit::result(self)
    }

    fn visit_root(&mut self) -> Result<(), SyntaxErrors> {
        visit::root(self)
    }

    fn visit_script_module_binary(&mut self) -> Result<(), SyntaxErrors> {
        visit::script_module_binary(self)
    }

    fn visit_script_module_quote(&mut self) -> Result<(), SyntaxErrors> {
        visit::script_module_quote(self)
    }

    fn visit_script_module(&mut self) -> Result<(), SyntaxErrors> {
        visit::script_module(self)
    }

    fn visit_share(&mut self) -> Result<(), SyntaxErrors> {
        visit::share(self)
    }

    fn visit_string(&mut self) -> Result<(), SyntaxErrors> {
        visit::string(self)
    }

    fn visit_table_fields_elem(&mut self) -> Result<(), SyntaxErrors> {
        visit::table_fields_elem(self)
    }

    fn visit_table_fields_type(&mut self) -> Result<(), SyntaxErrors> {
        visit::table_fields_type(self)
    }

    fn visit_table_type(&mut self) -> Result<(), SyntaxErrors> {
        visit::table_type(self)
    }

    fn visit_table_use(&mut self) -> Result<(), SyntaxErrors> {
        visit::table_use(self)
    }

    fn visit_type_field(&mut self) -> Result<(), SyntaxErrors> {
        visit::type_field(self)
    }

    fn visit_type_use(&mut self) -> Result<(), SyntaxErrors> {
        visit::type_use(self)
    }

    fn visit_value_type_num_type(&mut self) -> Result<(), SyntaxErrors> {
        visit::value_type_num_type(self)
    }

    fn visit_value_type_ref_type(&mut self) -> Result<(), SyntaxErrors> {
        visit::value_type_ref_type(self)
    }

    fn visit_value_type(&mut self) -> Result<(), SyntaxErrors> {
        visit::value_type(self)
    }
}

pub mod utils {
    #![allow(missing_docs)]
    #![allow(unused)]

    use super::*;
    use crate::node::{Context, SyntaxError};

    pub trait Choice<'tree, Ctx, Vis>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        #[inline]
        fn choice(&self, visitor: &mut Vis) -> Result<(), SyntaxErrors>;
    }

    impl<'tree, Ctx, Vis> Choice<'tree, Ctx, Vis> for ()
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        #[inline]
        fn choice(&self, _: &mut Vis) -> Result<(), SyntaxErrors> {
            Ok(())
        }
    }

    impl<'tree, Ctx, Vis, A> Choice<'tree, Ctx, Vis> for (A,)
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
        A: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
    {
        #[inline]
        fn choice(&self, visitor: &mut Vis) -> Result<(), SyntaxErrors> {
            self.0(visitor)
        }
    }

    impl<'tree, Ctx, Vis, A, B> Choice<'tree, Ctx, Vis> for (A, B)
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
        A: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
        B: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
    {
        #[inline]
        fn choice(&self, visitor: &mut Vis) -> Result<(), SyntaxErrors> {
            let mut errors = SyntaxErrors::new();

            if let Err(mut errs) = restore(&self.0)(visitor) {
                errors.append(&mut errs);
            } else {
                return Ok(());
            }

            if let Err(mut errs) = restore(&self.1)(visitor) {
                errors.append(&mut errs);
            } else {
                return Ok(());
            }

            Err(errors)
        }
    }

    impl<'tree, Ctx, Vis, A, B, C> Choice<'tree, Ctx, Vis> for (A, B, C)
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
        A: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
        B: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
        C: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
    {
        #[inline]
        fn choice(&self, visitor: &mut Vis) -> Result<(), SyntaxErrors> {
            let mut errors = SyntaxErrors::new();

            if let Err(mut errs) = restore(&self.0)(visitor) {
                errors.append(&mut errs);
            } else {
                return Ok(());
            }

            if let Err(mut errs) = restore(&self.1)(visitor) {
                errors.append(&mut errs);
            } else {
                return Ok(());
            }

            if let Err(mut errs) = restore(&self.2)(visitor) {
                errors.append(&mut errs);
            } else {
                return Ok(());
            }

            Err(errors)
        }
    }

    impl<'tree, Ctx, Vis, A, B, C, D> Choice<'tree, Ctx, Vis> for (A, B, C, D)
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
        A: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
        B: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
        C: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
        D: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
    {
        #[inline]
        fn choice(&self, visitor: &mut Vis) -> Result<(), SyntaxErrors> {
            let mut errors = SyntaxErrors::new();

            if let Err(mut errs) = restore(&self.0)(visitor) {
                errors.append(&mut errs);
            } else {
                return Ok(());
            }

            if let Err(mut errs) = restore(&self.1)(visitor) {
                errors.append(&mut errs);
            } else {
                return Ok(());
            }

            if let Err(mut errs) = restore(&self.2)(visitor) {
                errors.append(&mut errs);
            } else {
                return Ok(());
            }

            if let Err(mut errs) = restore(&self.3)(visitor) {
                errors.append(&mut errs);
            } else {
                return Ok(());
            }

            Err(errors)
        }
    }

    impl<'tree, Ctx, Vis, A, B, C, D, E> Choice<'tree, Ctx, Vis> for (A, B, C, D, E)
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
        A: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
        B: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
        C: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
        D: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
        E: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
    {
        #[inline]
        fn choice(&self, visitor: &mut Vis) -> Result<(), SyntaxErrors> {
            let mut errors = SyntaxErrors::new();

            if let Err(mut errs) = restore(&self.0)(visitor) {
                errors.append(&mut errs);
            } else {
                return Ok(());
            }

            if let Err(mut errs) = restore(&self.1)(visitor) {
                errors.append(&mut errs);
            } else {
                return Ok(());
            }

            if let Err(mut errs) = restore(&self.2)(visitor) {
                errors.append(&mut errs);
            } else {
                return Ok(());
            }

            if let Err(mut errs) = restore(&self.3)(visitor) {
                errors.append(&mut errs);
            } else {
                return Ok(());
            }

            if let Err(mut errs) = restore(&self.4)(visitor) {
                errors.append(&mut errs);
            } else {
                return Ok(());
            }

            Err(errors)
        }
    }

    impl<'tree, Ctx, Vis, A, B, C, D, E, F> Choice<'tree, Ctx, Vis> for (A, B, C, D, E, F)
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
        A: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
        B: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
        C: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
        D: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
        E: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
        F: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
    {
        #[inline]
        fn choice(&self, visitor: &mut Vis) -> Result<(), SyntaxErrors> {
            let mut errors = SyntaxErrors::new();

            if let Err(mut errs) = restore(&self.0)(visitor) {
                errors.append(&mut errs);
            } else {
                return Ok(());
            }

            if let Err(mut errs) = restore(&self.1)(visitor) {
                errors.append(&mut errs);
            } else {
                return Ok(());
            }

            if let Err(mut errs) = restore(&self.2)(visitor) {
                errors.append(&mut errs);
            } else {
                return Ok(());
            }

            if let Err(mut errs) = restore(&self.3)(visitor) {
                errors.append(&mut errs);
            } else {
                return Ok(());
            }

            if let Err(mut errs) = restore(&self.4)(visitor) {
                errors.append(&mut errs);
            } else {
                return Ok(());
            }

            if let Err(mut errs) = restore(&self.5)(visitor) {
                errors.append(&mut errs);
            } else {
                return Ok(());
            }

            Err(errors)
        }
    }

    impl<'tree, Ctx, Vis, A, B, C, D, E, F, G> Choice<'tree, Ctx, Vis> for (A, B, C, D, E, F, G)
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
        A: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
        B: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
        C: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
        D: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
        E: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
        F: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
        G: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
    {
        #[inline]
        fn choice(&self, visitor: &mut Vis) -> Result<(), SyntaxErrors> {
            let mut errors = SyntaxErrors::new();

            if let Err(mut errs) = restore(&self.0)(visitor) {
                errors.append(&mut errs);
            } else {
                return Ok(());
            }

            if let Err(mut errs) = restore(&self.1)(visitor) {
                errors.append(&mut errs);
            } else {
                return Ok(());
            }

            if let Err(mut errs) = restore(&self.2)(visitor) {
                errors.append(&mut errs);
            } else {
                return Ok(());
            }

            if let Err(mut errs) = restore(&self.3)(visitor) {
                errors.append(&mut errs);
            } else {
                return Ok(());
            }

            if let Err(mut errs) = restore(&self.4)(visitor) {
                errors.append(&mut errs);
            } else {
                return Ok(());
            }

            if let Err(mut errs) = restore(&self.5)(visitor) {
                errors.append(&mut errs);
            } else {
                return Ok(());
            }

            if let Err(mut errs) = restore(&self.6)(visitor) {
                errors.append(&mut errs);
            } else {
                return Ok(());
            }

            Err(errors)
        }
    }

    impl<'tree, Ctx, Vis, A, B, C, D, E, F, G, H> Choice<'tree, Ctx, Vis> for (A, B, C, D, E, F, G, H)
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
        A: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
        B: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
        C: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
        D: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
        E: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
        F: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
        G: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
        H: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
    {
        #[inline]
        fn choice(&self, visitor: &mut Vis) -> Result<(), SyntaxErrors> {
            let mut errors = SyntaxErrors::new();

            if let Err(mut errs) = restore(&self.0)(visitor) {
                errors.append(&mut errs);
            } else {
                return Ok(());
            }

            if let Err(mut errs) = restore(&self.1)(visitor) {
                errors.append(&mut errs);
            } else {
                return Ok(());
            }

            if let Err(mut errs) = restore(&self.2)(visitor) {
                errors.append(&mut errs);
            } else {
                return Ok(());
            }

            if let Err(mut errs) = restore(&self.3)(visitor) {
                errors.append(&mut errs);
            } else {
                return Ok(());
            }

            if let Err(mut errs) = restore(&self.4)(visitor) {
                errors.append(&mut errs);
            } else {
                return Ok(());
            }

            if let Err(mut errs) = restore(&self.5)(visitor) {
                errors.append(&mut errs);
            } else {
                return Ok(());
            }

            if let Err(mut errs) = restore(&self.6)(visitor) {
                errors.append(&mut errs);
            } else {
                return Ok(());
            }

            if let Err(mut errs) = restore(&self.7)(visitor) {
                errors.append(&mut errs);
            } else {
                return Ok(());
            }

            Err(errors)
        }
    }

    impl<'tree, Ctx, Vis, A, B, C, D, E, F, G, H, I> Choice<'tree, Ctx, Vis> for (A, B, C, D, E, F, G, H, I)
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
        A: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
        B: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
        C: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
        D: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
        E: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
        F: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
        G: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
        H: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
        I: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
    {
        #[inline]
        fn choice(&self, visitor: &mut Vis) -> Result<(), SyntaxErrors> {
            let mut errors = SyntaxErrors::new();

            if let Err(mut errs) = restore(&self.0)(visitor) {
                errors.append(&mut errs);
            } else {
                return Ok(());
            }

            if let Err(mut errs) = restore(&self.1)(visitor) {
                errors.append(&mut errs);
            } else {
                return Ok(());
            }

            if let Err(mut errs) = restore(&self.2)(visitor) {
                errors.append(&mut errs);
            } else {
                return Ok(());
            }

            if let Err(mut errs) = restore(&self.3)(visitor) {
                errors.append(&mut errs);
            } else {
                return Ok(());
            }

            if let Err(mut errs) = restore(&self.4)(visitor) {
                errors.append(&mut errs);
            } else {
                return Ok(());
            }

            if let Err(mut errs) = restore(&self.5)(visitor) {
                errors.append(&mut errs);
            } else {
                return Ok(());
            }

            if let Err(mut errs) = restore(&self.6)(visitor) {
                errors.append(&mut errs);
            } else {
                return Ok(());
            }

            if let Err(mut errs) = restore(&self.7)(visitor) {
                errors.append(&mut errs);
            } else {
                return Ok(());
            }

            if let Err(mut errs) = restore(&self.8)(visitor) {
                errors.append(&mut errs);
            } else {
                return Ok(());
            }

            Err(errors)
        }
    }

    impl<'tree, Ctx, Vis, A, B, C, D, E, F, G, H, I, J> Choice<'tree, Ctx, Vis> for (A, B, C, D, E, F, G, H, I, J)
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
        A: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
        B: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
        C: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
        D: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
        E: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
        F: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
        G: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
        H: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
        I: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
        J: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
    {
        #[inline]
        fn choice(&self, visitor: &mut Vis) -> Result<(), SyntaxErrors> {
            let mut errors = SyntaxErrors::new();

            if let Err(mut errs) = restore(&self.0)(visitor) {
                errors.append(&mut errs);
            } else {
                return Ok(());
            }

            if let Err(mut errs) = restore(&self.1)(visitor) {
                errors.append(&mut errs);
            } else {
                return Ok(());
            }

            if let Err(mut errs) = restore(&self.2)(visitor) {
                errors.append(&mut errs);
            } else {
                return Ok(());
            }

            if let Err(mut errs) = restore(&self.3)(visitor) {
                errors.append(&mut errs);
            } else {
                return Ok(());
            }

            if let Err(mut errs) = restore(&self.4)(visitor) {
                errors.append(&mut errs);
            } else {
                return Ok(());
            }

            if let Err(mut errs) = restore(&self.5)(visitor) {
                errors.append(&mut errs);
            } else {
                return Ok(());
            }

            if let Err(mut errs) = restore(&self.6)(visitor) {
                errors.append(&mut errs);
            } else {
                return Ok(());
            }

            if let Err(mut errs) = restore(&self.7)(visitor) {
                errors.append(&mut errs);
            } else {
                return Ok(());
            }

            if let Err(mut errs) = restore(&self.8)(visitor) {
                errors.append(&mut errs);
            } else {
                return Ok(());
            }

            if let Err(mut errs) = restore(&self.9)(visitor) {
                errors.append(&mut errs);
            } else {
                return Ok(());
            }

            Err(errors)
        }
    }

    #[inline]
    pub fn choice<'tree, Ctx, Vis, T>(funs: T) -> impl Fn(&mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
        T: Choice<'tree, Ctx, Vis>,
    {
        move |visitor| funs.choice(visitor)
    }

    #[inline]
    pub fn done<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        if visitor.walker().done {
            return Ok(());
        } else {
            let mut errors = SyntaxErrors::new();
            errors.push(SyntaxError::MoreNodes);
            return Err(errors);
        }
    }

    #[inline]
    pub fn optional<'tree, Ctx, Vis>(
        fun: impl Fn(&mut Vis) -> Result<(), SyntaxErrors>,
    ) -> impl Fn(&mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        move |visitor| {
            let prev = visitor.node();
            if fun(visitor).is_err() {
                visitor.reset(prev);
            }

            Ok(())
        }
    }

    pub fn repeat<'tree, Ctx, Vis>(
        fun: impl Fn(&mut Vis) -> Result<(), SyntaxErrors>,
    ) -> impl Fn(&mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        move |visitor| {
            loop {
                let prev = visitor.node();
                if visitor.walker().done {
                    break;
                }
                if fun(visitor).is_err() {
                    visitor.reset(prev);
                    break;
                }
            }

            Ok(())
        }
    }

    #[inline]
    pub fn repeat1<'tree, Ctx, Vis>(
        fun: impl Fn(&mut Vis) -> Result<(), SyntaxErrors>,
    ) -> impl Fn(&mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        move |visitor| {
            let mut errors = SyntaxErrors::new();

            if visitor.walker().done {
                errors.push(SyntaxError::DoneEarly);
                return Err(errors);
            }

            let mut succeeded_once = false;
            loop {
                let prev = visitor.node();
                if let Err(mut errs) = fun(visitor) {
                    if succeeded_once {
                        visitor.reset(prev);
                        break;
                    }
                    errors.append(&mut errs);
                    return Err(errors);
                } else {
                    succeeded_once = true;
                }
            }

            Ok(())
        }
    }

    pub trait Seq<'tree, Ctx, Vis>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        #[inline]
        fn seq(&self, visitor: &mut Vis) -> Result<(), SyntaxErrors>;
    }

    impl<'tree, Ctx, Vis> Seq<'tree, Ctx, Vis> for ()
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        #[inline]
        fn seq(&self, _: &mut Vis) -> Result<(), SyntaxErrors> {
            Ok(())
        }
    }

    impl<'tree, Ctx, Vis, A> Seq<'tree, Ctx, Vis> for (A,)
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
        A: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
    {
        #[inline]
        fn seq(&self, visitor: &mut Vis) -> Result<(), SyntaxErrors> {
            self.0(visitor)?;
            Ok(())
        }
    }

    impl<'tree, Ctx, Vis, A, B> Seq<'tree, Ctx, Vis> for (A, B)
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
        A: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
        B: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
    {
        #[inline]
        fn seq(&self, visitor: &mut Vis) -> Result<(), SyntaxErrors> {
            self.0(visitor)?;
            self.1(visitor)?;
            Ok(())
        }
    }

    impl<'tree, Ctx, Vis, A, B, C> Seq<'tree, Ctx, Vis> for (A, B, C)
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
        A: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
        B: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
        C: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
    {
        #[inline]
        fn seq(&self, visitor: &mut Vis) -> Result<(), SyntaxErrors> {
            self.0(visitor)?;
            self.1(visitor)?;
            self.2(visitor)?;
            Ok(())
        }
    }

    impl<'tree, Ctx, Vis, A, B, C, D> Seq<'tree, Ctx, Vis> for (A, B, C, D)
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
        A: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
        B: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
        C: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
        D: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
    {
        #[inline]
        fn seq(&self, visitor: &mut Vis) -> Result<(), SyntaxErrors> {
            self.0(visitor)?;
            self.1(visitor)?;
            self.2(visitor)?;
            self.3(visitor)?;
            Ok(())
        }
    }

    impl<'tree, Ctx, Vis, A, B, C, D, E> Seq<'tree, Ctx, Vis> for (A, B, C, D, E)
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
        A: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
        B: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
        C: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
        D: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
        E: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
    {
        #[inline]
        fn seq(&self, visitor: &mut Vis) -> Result<(), SyntaxErrors> {
            self.0(visitor)?;
            self.1(visitor)?;
            self.2(visitor)?;
            self.3(visitor)?;
            self.4(visitor)?;
            Ok(())
        }
    }

    impl<'tree, Ctx, Vis, A, B, C, D, E, F> Seq<'tree, Ctx, Vis> for (A, B, C, D, E, F)
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
        A: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
        B: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
        C: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
        D: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
        E: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
        F: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
    {
        #[inline]
        fn seq(&self, visitor: &mut Vis) -> Result<(), SyntaxErrors> {
            self.0(visitor)?;
            self.1(visitor)?;
            self.2(visitor)?;
            self.3(visitor)?;
            self.4(visitor)?;
            self.5(visitor)?;
            Ok(())
        }
    }

    impl<'tree, Ctx, Vis, A, B, C, D, E, F, G> Seq<'tree, Ctx, Vis> for (A, B, C, D, E, F, G)
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
        A: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
        B: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
        C: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
        D: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
        E: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
        F: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
        G: Fn(&mut Vis) -> Result<(), SyntaxErrors>,
    {
        #[inline]
        fn seq(&self, visitor: &mut Vis) -> Result<(), SyntaxErrors> {
            self.0(visitor)?;
            self.1(visitor)?;
            self.2(visitor)?;
            self.3(visitor)?;
            self.4(visitor)?;
            self.5(visitor)?;
            self.6(visitor)?;
            Ok(())
        }
    }

    #[inline]
    pub fn seq<'tree, Ctx, Vis, T>(funs: T) -> impl Fn(&mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
        T: Seq<'tree, Ctx, Vis>,
    {
        move |visitor| funs.seq(visitor)
    }

    #[inline]
    pub fn restore<'tree, Ctx, Vis>(
        fun: impl Fn(&mut Vis) -> Result<(), SyntaxErrors>,
    ) -> impl Fn(&mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        move |visitor| {
            let prev = visitor.node();
            if let Err(mut errs) = fun(visitor) {
                visitor.reset(prev);
                let mut errors = SyntaxErrors::new();
                errors.append(&mut errs);
                return Err(errors);
            }
            Ok(())
        }
    }
}

pub mod visit {
    #![allow(missing_docs)]
    #![allow(unused)]

    use super::*;
    use crate::node::{Context, SyntaxError};

    #[inline]
    pub fn action_get<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn action_invoke<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        visitor.walker().rule(kind::ACTION_INVOKE)?;
        utils::seq((
            token::LPAREN,
            token::INVOKE,
            utils::optional(identifier),
            name,
            utils::repeat(expr_plain_const),
            token::RPAREN,
        ))(visitor)
    }

    #[inline]
    pub fn action<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        visitor.walker().rule(kind::ACTION)?;
        utils::choice((action_invoke, action_get))(visitor)
    }

    #[inline]
    pub fn align_offset_value<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn align_value<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn annotation_parens<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn annotation_part<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn annotation<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn assert_exhaustion<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        visitor.walker().rule(kind::ASSERT_EXHAUSTION)?;
        utils::seq((token::LPAREN, token::ASSERT_EXHAUSTION, action, string, token::RPAREN))(visitor)
    }

    #[inline]
    pub fn assert_invalid<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        visitor.walker().rule(kind::ASSERT_INVALID)?;
        utils::seq((
            token::LPAREN,
            token::ASSERT_INVALID,
            script_module,
            string,
            token::RPAREN,
        ))(visitor)
    }

    #[inline]
    pub fn assert_malformed<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        visitor.walker().rule(kind::ASSERT_MALFORMED)?;
        utils::seq((
            token::LPAREN,
            token::ASSERT_MALFORMED,
            script_module,
            string,
            token::RPAREN,
        ))(visitor)
    }

    #[inline]
    pub fn assert_return_arithmetic_nan<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        visitor.walker().rule(kind::ASSERT_RETURN_ARITHMETIC_NAN)?;
        utils::seq((
            token::LPAREN,
            token::ASSERT_RETURN_ARITHMETIC_NAN,
            action,
            utils::repeat(result),
            token::RPAREN,
        ))(visitor)
    }

    #[inline]
    pub fn assert_return_canonical_nan<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        visitor.walker().rule(kind::ASSERT_RETURN_CANONICAL_NAN)?;
        utils::seq((
            token::LPAREN,
            token::ASSERT_RETURN_CANONICAL_NAN,
            action,
            utils::repeat(result),
            token::RPAREN,
        ))(visitor)
    }

    #[inline]
    pub fn assert_return<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        visitor.walker().rule(kind::ASSERT_RETURN)?;
        utils::seq((
            token::LPAREN,
            token::ASSERT_RETURN,
            action,
            utils::repeat(result),
            token::RPAREN,
        ))(visitor)
    }

    #[inline]
    pub fn assert_trap_action<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        visitor.walker().rule(kind::ASSERT_TRAP_ACTION)?;
        utils::seq((token::LPAREN, token::ASSERT_TRAP, action, string, token::RPAREN))(visitor)
    }

    #[inline]
    pub fn assert_trap_module<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        visitor.walker().rule(kind::ASSERT_TRAP_MODULE)?;
        utils::seq((token::LPAREN, token::ASSERT_TRAP, script_module, string, token::RPAREN))(visitor)
    }

    #[inline]
    pub fn assert_unlinkable<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        visitor.walker().rule(kind::ASSERT_UNLINKABLE)?;
        utils::seq((
            token::LPAREN,
            token::ASSERT_UNLINKABLE,
            script_module,
            string,
            token::RPAREN,
        ))(visitor)
    }

    #[inline]
    pub fn assertion<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        visitor.walker().rule(kind::ASSERTION)?;
        utils::choice((
            assert_malformed,
            assert_invalid,
            assert_unlinkable,
            assert_trap_action,
            assert_trap_module,
            assert_return,
            assert_return_canonical_nan,
            assert_return_arithmetic_nan,
            assert_exhaustion,
        ))(visitor)
    }

    #[inline]
    pub fn block_block<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn block_if<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn block_loop<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn command<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        visitor.walker().rule(kind::COMMAND)?;
        utils::choice((action, assertion, meta, register, script_module))(visitor)
    }

    #[inline]
    pub fn comment_block_annot<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn comment_block<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn comment_line_annot<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn comment_line<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn dec_float<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn dec_nat<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        visitor.walker().token(kind::DEC_NAT)?;
        Ok(())
    }

    #[inline]
    pub fn elem_expr_expr<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn elem_expr_item<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn elem_expr<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn elem_kind<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn elem_list<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn escape_sequence<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn export_desc_func<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn export_desc_global<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn export_desc_memory<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn export_desc_table<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn export_desc<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn export<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn expr_plain_const<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        visitor.walker().rule(kind::EXPR_PLAIN_CONST)?;
        utils::seq((
            token::LPAREN,
            utils::choice((op_const_ref, op_simd_const)),
            token::RPAREN,
        ))(visitor)
    }

    #[inline]
    pub fn expr<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn expr1_block<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn expr1_call<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn expr1_if<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn expr1_loop<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn expr1_plain<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn expr1<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn float<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn func_locals_many<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn func_locals_one<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn func_locals<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn func_type_params_many<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn func_type_params_one<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn func_type_params<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn func_type_results<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn func_type<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn global_type_imm<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn global_type_mut<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn global_type<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn hex_float<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn hex_nat<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        visitor.walker().token(kind::HEX_NAT)?;
        Ok(())
    }

    #[inline]
    pub fn identifier<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        visitor.walker().token(kind::IDENTIFIER)?;
        Ok(())
    }

    #[inline]
    pub fn if_block<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn import_desc_func_type<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn import_desc_global_type<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn import_desc_memory_type<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn import_desc_table_type<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn import_desc_type_use<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn import_desc<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn import<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn index<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        visitor.walker().rule(kind::INDEX)?;
        utils::choice((nat, identifier))(visitor)
    }

    #[inline]
    pub fn instr_block<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn instr_call<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn instr_list<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn instr_plain<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn instr<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn int<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn limits<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn literal_nan_arithmetic<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        visitor.walker().rule(kind::LITERAL_NAN_ARITHMETIC)?;
        token::NAN_ARITHMETIC(visitor)
    }

    #[inline]
    pub fn literal_nan_canonical<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        visitor.walker().rule(kind::LITERAL_NAN_CANONICAL)?;
        token::NAN_CANONICAL(visitor)
    }

    #[inline]
    pub fn literal_nan<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        visitor.walker().rule(kind::LITERAL_NAN)?;
        utils::choice((literal_nan_arithmetic, literal_nan_canonical))(visitor)
    }

    #[inline]
    pub fn memory_fields_data<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn memory_fields_type<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn memory_type<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn memory_use<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn meta_input<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        visitor.walker().rule(kind::META_INPUT)?;
        utils::seq((
            token::LPAREN,
            token::INPUT,
            utils::optional(identifier),
            string,
            token::RPAREN,
        ))(visitor)
    }

    #[inline]
    pub fn meta_output<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        visitor.walker().rule(kind::META_OUTPUT)?;
        utils::seq((
            token::LPAREN,
            token::OUTPUT,
            utils::optional(identifier),
            utils::optional(string),
            token::RPAREN,
        ))(visitor)
    }

    #[inline]
    pub fn meta_script<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        visitor.walker().rule(kind::META_SCRIPT)?;
        utils::seq((
            token::LPAREN,
            token::SCRIPT,
            utils::optional(identifier),
            utils::repeat(command),
            token::RPAREN,
        ))(visitor)
    }

    #[inline]
    pub fn meta<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        visitor.walker().rule(kind::META)?;
        utils::choice((meta_script, meta_input, meta_output))(visitor)
    }

    #[inline]
    pub fn module_field_data<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn module_field_elem<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn module_field_export<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn module_field_func<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn module_field_global<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn module_field_import<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn module_field_memory<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn module_field_start<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        visitor.walker().rule(kind::MODULE_FIELD_START)?;
        utils::seq((token::LPAREN, token::START, index, token::RPAREN))(visitor)
    }

    #[inline]
    pub fn module_field_table<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn module_field_type<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn module_field<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        visitor.walker().rule(kind::MODULE_FIELD)?;
        utils::choice((
            // module_field_type,
            // module_field_global,
            // module_field_table,
            // module_field_memory,
            // module_field_func,
            // module_field_elem,
            // module_field_data,
            module_field_start,
            /* module_field_import,
             * module_field_export, */
        ))(visitor)
    }

    #[inline]
    pub fn module<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        utils::seq((
            token::LPAREN,
            utils::optional(identifier),
            utils::repeat(module_field),
            token::RPAREN,
        ))(visitor)
    }

    #[inline]
    pub fn name<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn nan<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn nat<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        visitor.walker().rule(kind::NAT)?;
        utils::choice((dec_nat, hex_nat))(visitor)
    }

    #[inline]
    pub fn num_type_f32<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn num_type_f64<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn num_type_i32<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn num_type_i64<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn num_type_v128<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn num<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn offset_const_expr<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn offset_expr<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn offset_value<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn offset<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        visitor.walker().rule(kind::OP_CONST_REF)?;
        utils::choice((
            op_const,
            utils::seq((token::REF_NULL, utils::choice((ref_kind, index)))),
            utils::seq((token::REF_EXTERN, nat)),
        ))(visitor)
    }

    #[inline]
    pub fn op_const_ref<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn op_const<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn op_func_bind<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn op_index_opt_offset_opt_align_opt<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn op_index_opt<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn op_index<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn op_let<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn op_nullary<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn op_select<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn op_simd_const<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn op_simd_lane<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn op_simd_offset_opt_align_opt<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn op_table_copy<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn op_table_init<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn ref_kind<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn ref_type_externref<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn ref_type_funcref<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn ref_type_ref<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn ref_type<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn register<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        visitor.walker().rule(kind::REGISTER)?;
        utils::seq((
            token::LPAREN,
            token::REGISTER,
            name,
            utils::optional(identifier),
            token::RPAREN,
        ))(visitor)
    }

    #[inline]
    pub fn reserved<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn result_const_nan<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        visitor.walker().rule(kind::RESULT_CONST_NAN)?;
        utils::seq((
            utils::choice((token::F32, token::F64, token::I32, token::I64)),
            token::FULL_STOP,
            token::CONST,
            literal_nan,
        ))(visitor)
    }

    #[inline]
    pub fn result_const<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        visitor.walker().rule(kind::RESULT_CONST)?;
        utils::choice((op_const_ref, op_simd_const))(visitor)
    }

    #[inline]
    pub fn result_ref_extern<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        visitor.walker().rule(kind::RESULT_REF_EXTERN)?;
        token::REF_EXTERN(visitor)
    }

    #[inline]
    pub fn result_ref_func<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        visitor.walker().rule(kind::RESULT_REF_FUNC)?;
        token::REF_FUNC(visitor)
    }

    #[inline]
    pub fn result_ref_null<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        visitor.walker().rule(kind::RESULT_REF_NULL)?;
        token::REF_NULL(visitor)
    }

    #[inline]
    pub fn result<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        visitor.walker().rule(kind::RESULT)?;
        utils::seq((
            token::LPAREN,
            utils::choice((
                result_const,
                result_const_nan,
                result_ref_func,
                result_ref_extern,
                result_ref_null,
            )),
            token::RPAREN,
        ))(visitor)
    }

    #[inline]
    pub fn root<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        utils::choice((utils::repeat(command), utils::repeat1(module_field)))(visitor)?;
        utils::done(visitor)
    }

    #[inline]
    pub fn script_module_binary<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        visitor.walker().rule(kind::SCRIPT_MODULE_BINARY)?;
        utils::choice((
            token::LPAREN,
            token::MODULE,
            utils::optional(identifier),
            token::BINARY,
            utils::repeat(string),
            token::RPAREN,
        ))(visitor)
    }

    #[inline]
    pub fn script_module_quote<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        visitor.walker().rule(kind::SCRIPT_MODULE_BINARY)?;
        utils::choice((
            token::LPAREN,
            token::MODULE,
            utils::optional(identifier),
            token::QUOTE,
            utils::repeat(string),
            token::RPAREN,
        ))(visitor)
    }

    #[inline]
    pub fn script_module<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        visitor.walker().rule(kind::SCRIPT_MODULE)?;
        utils::choice((module, script_module_binary, script_module_quote))(visitor)
    }

    #[inline]
    pub fn share<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn string<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn table_fields_elem<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn table_fields_type<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn table_type<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn table_use<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn type_field<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn type_use<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn value_type_num_type<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn value_type_ref_type<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    #[inline]
    pub fn value_type<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
    where
        Ctx: Context<'tree> + 'tree,
        Vis: Visitor<'tree, Ctx> + ?Sized,
    {
        Ok(())
    }

    mod token {
        use super::*;

        macro_rules! make {
            ($name:tt) => {
                #[inline]
                #[allow(non_snake_case)]
                pub fn $name<'tree, Ctx, Vis>(visitor: &mut Vis) -> Result<(), SyntaxErrors>
                where
                    Ctx: Context<'tree> + 'tree,
                    Vis: Visitor<'tree, Ctx> + ?Sized,
                {
                    visitor.walker().token(kind::token::$name)?;
                    Ok(())
                }
            };
        }

        make!(ALIGN);
        make!(ASSERT_EXHAUSTION);
        make!(ASSERT_INVALID);
        make!(ASSERT_MALFORMED);
        make!(ASSERT_RETURN_ARITHMETIC_NAN);
        make!(ASSERT_RETURN_CANONICAL_NAN);
        make!(ASSERT_RETURN);
        make!(ASSERT_TRAP);
        make!(ASSERT_UNLINKABLE);
        make!(BINARY);
        make!(BLOCK);
        make!(BR_TABLE);
        make!(CALL_INDIRECT);
        make!(CONST);
        make!(DATA);
        make!(DECLARE);
        make!(DOLLAR_SIGN);
        make!(ELEM);
        make!(ELSE);
        make!(END);
        make!(EQUALS);
        make!(EXPORT);
        make!(EXTERNREF);
        make!(F32);
        make!(F64);
        make!(FULL_STOP);
        make!(FUNC);
        make!(FUNCREF);
        make!(GET);
        make!(GLOBAL);
        make!(I32);
        make!(I64);
        make!(IF);
        make!(IMPORT);
        make!(INF);
        make!(INPUT);
        make!(INVOKE);
        make!(ITEM);
        make!(LOCAL);
        make!(LOOP);
        make!(LPAREN_AMPERSAND);
        make!(LPAREN_SEMICOLON);
        make!(LPAREN);
        make!(MEMORY);
        make!(MODULE);
        make!(MUT);
        make!(NAN_ARITHMETIC);
        make!(NAN_CANONICAL);
        make!(OFFSET);
        make!(OUTPUT);
        make!(PARAM);
        make!(QUOTE);
        make!(REF_EXTERN);
        make!(REF_FUNC);
        make!(REF_NULL);
        make!(REF);
        make!(REGISTER);
        make!(RESULT);
        make!(REVERSE_SOLIDUS_REVERSE_SOLIDUS);
        make!(RPAREN);
        make!(SCRIPT);
        make!(SEMICOLON_RPAREN);
        make!(SEMICOLON_SEMICOLON);
        make!(START);
        make!(TABLE);
        make!(THEN);
        make!(TYPE);
        make!(V128);
        make!(ZERO_X);
    }
}

use crate::node::{context::basic::Context, BasicNodeWalker};

#[allow(missing_docs)]
pub struct BasicVisitor<'tree> {
    walker: BasicNodeWalker<'tree>,
}

#[allow(missing_docs)]
impl<'tree> BasicVisitor<'tree> {
    pub fn new(start: tree_sitter::Node<'tree>) -> Self {
        let language = wasm_lsp_languages::Language::Wast;
        let walker = BasicNodeWalker::new(language, start);
        Self { walker }
    }
}

impl<'tree> Visitor<'tree, Context<'tree>> for BasicVisitor<'tree> {
    #[inline]
    fn node(&self) -> tree_sitter::Node<'tree> {
        self.walker.node()
    }

    #[inline]
    fn reset(&mut self, node: tree_sitter::Node<'tree>) {
        self.walker.reset(node)
    }

    #[inline]
    fn walker(&mut self) -> &mut NodeWalker<'tree, Context<'tree>> {
        &mut self.walker
    }
}
