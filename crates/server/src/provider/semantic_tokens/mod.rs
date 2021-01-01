//! Provides `textDocument/semanticTokens/*` functionality.

mod builder;
mod node_walker;

use builder::SemanticTokensBuilder;

// Move to the next appropriate node in the syntax tree.
fn goto_next<'a>(cursor: &mut tree_sitter::TreeCursor<'a>, stack: &mut Vec<tree_sitter::Node<'a>>, done: &mut bool) {
    let prev = cursor.node();

    // First try to descend to the first child node.
    if cursor.goto_first_child() {
        stack.push(prev);
    } else {
        // Otherwise try to move to the next sibling node.
        if !cursor.goto_next_sibling() {
            let mut finished = true;
            // Otherwise continue to ascend to parent nodes...
            while cursor.goto_parent() {
                stack.pop();
                // ... until we can move to a sibling node.
                if cursor.goto_next_sibling() {
                    finished = false;
                    break;
                }
                // Otherwise we set `done = true` and stop the outer loop.
            }
            *done = finished;
        }
    }
}

/// Semantic tokens provider definitions for ".wast" files.
pub mod wast {
    use super::goto_next;
    use crate::{
        core::{document::Document, language::wast, session::Session},
        provider::semantic_tokens::SemanticTokensBuilder,
    };
    use anyhow::anyhow;
    use lspower::lsp_types::*;
    use std::sync::Arc;

    pub(crate) async fn full(
        session: Arc<Session>,
        document: &Document,
        params: SemanticTokensParams,
    ) -> anyhow::Result<Option<SemanticTokensResult>> {
        let params = SemanticTokensRangeParams {
            work_done_progress_params: params.work_done_progress_params,
            partial_result_params: params.partial_result_params,
            text_document: params.text_document,
            range: {
                let tree = document.tree.lock().await;
                let node = tree.root_node();
                crate::util::node::range(&node)
            },
        };

        let result = range(session, document, params).await?.map(|result| match result {
            SemanticTokensRangeResult::Tokens(tokens) => SemanticTokensResult::Tokens(tokens),
            SemanticTokensRangeResult::Partial(partial) => SemanticTokensResult::Partial(partial),
        });

        Ok(result)
    }

    pub(crate) async fn range(
        session: Arc<Session>,
        document: &Document,
        params: SemanticTokensRangeParams,
    ) -> anyhow::Result<Option<SemanticTokensRangeResult>> {
        let legend = session.semantic_tokens_legend().await;

        if let Some(node) = {
            let start = tree_sitter::Point {
                row: params.range.start.line as usize,
                column: params.range.start.character as usize,
            };
            let end = tree_sitter::Point {
                row: params.range.end.line as usize,
                column: params.range.end.character as usize,
            };
            document
                .tree
                .lock()
                .await
                .root_node()
                .descendant_for_point_range(start, end)
        } {
            let mut stack = vec![];
            let mut builder = SemanticTokensBuilder::new(legend.as_ref());
            let mut cursor = node.walk();
            let mut done = false;

            // If this node isn't ROOT, reconstruct the stack before starting.
            if !wast::kind::equals::ROOT(node.kind_id()) {
                while cursor.goto_parent() {
                    stack.push(cursor.node());
                }
                stack.reverse();
                cursor.reset(node);
            }

            loop {
                if done {
                    break;
                }

                // handle "root"
                if wast::kind::equals::ROOT(cursor.node().kind_id()) {
                    handle::root(&mut stack, &mut cursor, &mut builder, &mut done)?;
                    continue;
                }

                // handle "_action"
                if wast::kind::equals::ACTION_GET(cursor.node().kind_id()) {
                    handle::action_get(&mut stack, &mut cursor, &mut builder, &mut done)?;
                    continue;
                } else if wast::kind::equals::ACTION_INVOKE(cursor.node().kind_id()) {
                    handle::action_invoke(&mut stack, &mut cursor, &mut builder, &mut done)?;
                    continue;
                }

                // handle "_assertion"
                if wast::kind::equals::ASSERT_EXHAUSTION(cursor.node().kind_id()) {
                    handle::assert_exhaustion(&mut stack, &mut cursor, &mut builder, &mut done)?;
                    continue;
                } else if wast::kind::equals::ASSERT_INVALID(cursor.node().kind_id()) {
                    handle::assert_invalid(&mut stack, &mut cursor, &mut builder, &mut done)?;
                    continue;
                } else if wast::kind::equals::ASSERT_EXHAUSTION(cursor.node().kind_id()) {
                    handle::assert_exhaustion(&mut stack, &mut cursor, &mut builder, &mut done)?;
                    continue;
                } else if wast::kind::equals::ASSERT_INVALID(cursor.node().kind_id()) {
                    handle::assert_invalid(&mut stack, &mut cursor, &mut builder, &mut done)?;
                    continue;
                } else if wast::kind::equals::ASSERT_MALFORMED(cursor.node().kind_id()) {
                    handle::assert_malformed(&mut stack, &mut cursor, &mut builder, &mut done)?;
                    continue;
                } else if wast::kind::equals::ASSERT_RETURN(cursor.node().kind_id()) {
                    handle::assert_return(&mut stack, &mut cursor, &mut builder, &mut done)?;
                    continue;
                } else if wast::kind::equals::ASSERT_RETURN_ARITHMETIC_NAN(cursor.node().kind_id()) {
                    handle::assert_return_arithmetic_nan(&mut stack, &mut cursor, &mut builder, &mut done)?;
                    continue;
                } else if wast::kind::equals::ASSERT_RETURN_CANONICAL_NAN(cursor.node().kind_id()) {
                    handle::assert_return_canonical_nan(&mut stack, &mut cursor, &mut builder, &mut done)?;
                    continue;
                } else if wast::kind::equals::ASSERT_TRAP_ACTION(cursor.node().kind_id()) {
                    handle::assert_trap_action(&mut stack, &mut cursor, &mut builder, &mut done)?;
                    continue;
                } else if wast::kind::equals::ASSERT_TRAP_MODULE(cursor.node().kind_id()) {
                    handle::assert_trap_module(&mut stack, &mut cursor, &mut builder, &mut done)?;
                    continue;
                } else if wast::kind::equals::ASSERT_UNLINKABLE(cursor.node().kind_id()) {
                    handle::assert_unlinkable(&mut stack, &mut cursor, &mut builder, &mut done)?;
                    continue;
                }

                // handle "command"
                if wast::kind::equals::COMMAND(cursor.node().kind_id()) {
                    handle::command(&mut stack, &mut cursor, &mut builder, &mut done)?;
                    continue;
                }

                // handle {"comment_block", "comment_block_annot", "comment_line", "comment_line_annot"}
                if wast::kind::equals::COMMENT_BLOCK(cursor.node().kind_id()) {
                    handle::comment_block(&mut stack, &mut cursor, &mut builder, &mut done)?;
                    continue;
                } else if wast::kind::equals::COMMENT_BLOCK_ANNOT(cursor.node().kind_id()) {
                    handle::comment_block_annot(&mut stack, &mut cursor, &mut builder, &mut done)?;
                    continue;
                } else if wast::kind::equals::COMMENT_LINE(cursor.node().kind_id()) {
                    handle::comment_line(&mut stack, &mut cursor, &mut builder, &mut done)?;
                    continue;
                } else if wast::kind::equals::COMMENT_LINE_ANNOT(cursor.node().kind_id()) {
                    handle::comment_line_annot(&mut stack, &mut cursor, &mut builder, &mut done)?;
                    continue;
                }

                // handle "_meta"
                if wast::kind::equals::META_INPUT(cursor.node().kind_id()) {
                    handle::meta_input(&mut stack, &mut cursor, &mut builder, &mut done)?;
                    continue;
                } else if wast::kind::equals::META_OUTPUT(cursor.node().kind_id()) {
                    handle::meta_output(&mut stack, &mut cursor, &mut builder, &mut done)?;
                    continue;
                } else if wast::kind::equals::META_SCRIPT(cursor.node().kind_id()) {
                    handle::meta_script(&mut stack, &mut cursor, &mut builder, &mut done)?;
                    continue;
                }

                // handle "module"
                if wast::kind::equals::MODULE(cursor.node().kind_id()) {
                    handle::module(&mut stack, &mut cursor, &mut builder, &mut done)?;
                    continue;
                }

                // handle "_module_field"
                if wast::kind::equals::MODULE_FIELD_DATA(cursor.node().kind_id()) {
                    handle::module_field_data(&mut stack, &mut cursor, &mut builder, &mut done)?;
                    continue;
                } else if wast::kind::equals::MODULE_FIELD_ELEM(cursor.node().kind_id()) {
                    handle::module_field_elem(&mut stack, &mut cursor, &mut builder, &mut done)?;
                    continue;
                } else if wast::kind::equals::MODULE_FIELD_EXPORT(cursor.node().kind_id()) {
                    handle::module_field_export(&mut stack, &mut cursor, &mut builder, &mut done)?;
                    continue;
                } else if wast::kind::equals::MODULE_FIELD_FUNC(cursor.node().kind_id()) {
                    handle::module_field_func(&mut stack, &mut cursor, &mut builder, &mut done)?;
                    continue;
                } else if wast::kind::equals::MODULE_FIELD_GLOBAL(cursor.node().kind_id()) {
                    handle::module_field_global(&mut stack, &mut cursor, &mut builder, &mut done)?;
                    continue;
                } else if wast::kind::equals::MODULE_FIELD_IMPORT(cursor.node().kind_id()) {
                    handle::module_field_import(&mut stack, &mut cursor, &mut builder, &mut done)?;
                    continue;
                } else if wast::kind::equals::MODULE_FIELD_MEMORY(cursor.node().kind_id()) {
                    handle::module_field_memory(&mut stack, &mut cursor, &mut builder, &mut done)?;
                    continue;
                } else if wast::kind::equals::MODULE_FIELD_START(cursor.node().kind_id()) {
                    handle::module_field_start(&mut stack, &mut cursor, &mut builder, &mut done)?;
                    continue;
                } else if wast::kind::equals::MODULE_FIELD_TABLE(cursor.node().kind_id()) {
                    handle::module_field_table(&mut stack, &mut cursor, &mut builder, &mut done)?;
                    continue;
                } else if wast::kind::equals::MODULE_FIELD_TYPE(cursor.node().kind_id()) {
                    handle::module_field_type(&mut stack, &mut cursor, &mut builder, &mut done)?;
                    continue;
                }

                // handle "register"
                if wast::kind::equals::REGISTER(cursor.node().kind_id()) {
                    handle::register(&mut stack, &mut cursor, &mut builder, &mut done)?;
                    continue;
                }

                // handle "_script_module"
                if wast::kind::equals::MODULE(cursor.node().kind_id()) {
                    // handled earlier
                    unreachable!();
                } else if wast::kind::equals::SCRIPT_MODULE_BINARY(cursor.node().kind_id()) {
                    handle::script_module_binary(&mut stack, &mut cursor, &mut builder, &mut done)?;
                    continue;
                } else if wast::kind::equals::SCRIPT_MODULE_QUOTE(cursor.node().kind_id()) {
                    handle::script_module_quote(&mut stack, &mut cursor, &mut builder, &mut done)?;
                    continue;
                }

                // catch all case
                goto_next(&mut cursor, &mut stack, &mut done);
            }

            let tokens = builder.build()?;
            let result = SemanticTokensRangeResult::Tokens(tokens);

            Ok(Some(result))
        } else {
            Err(anyhow!("Could not obtain tree node for given range"))
        }
    }

    mod handle {
        use super::super::goto_next;
        use crate::{core::language::wast, provider::semantic_tokens::SemanticTokensBuilder};
        use lspower::lsp_types::*;

        pub(super) fn action_get<'a>(
            stack: &mut Vec<tree_sitter::Node<'a>>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            if let Some(node) = cursor.node().child(1) {
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            goto_next(cursor, stack, done);

            Ok(())
        }

        pub(super) fn action_invoke<'a>(
            stack: &mut Vec<tree_sitter::Node<'a>>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            if let Some(node) = cursor.node().child(1) {
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            goto_next(cursor, stack, done);

            Ok(())
        }

        pub(super) fn assert_exhaustion<'a>(
            stack: &mut Vec<tree_sitter::Node<'a>>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            if let Some(node) = cursor.node().child(1) {
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            goto_next(cursor, stack, done);

            Ok(())
        }

        pub(super) fn assert_invalid<'a>(
            stack: &mut Vec<tree_sitter::Node<'a>>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            if let Some(node) = cursor.node().child(1) {
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            goto_next(cursor, stack, done);

            Ok(())
        }

        pub(super) fn assert_malformed<'a>(
            stack: &mut Vec<tree_sitter::Node<'a>>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            if let Some(node) = cursor.node().child(1) {
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            goto_next(cursor, stack, done);

            Ok(())
        }

        pub(super) fn assert_return<'a>(
            stack: &mut Vec<tree_sitter::Node<'a>>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            if let Some(node) = cursor.node().child(1) {
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            goto_next(cursor, stack, done);

            Ok(())
        }

        pub(super) fn assert_return_arithmetic_nan<'a>(
            stack: &mut Vec<tree_sitter::Node<'a>>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            if let Some(node) = cursor.node().child(1) {
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            goto_next(cursor, stack, done);

            Ok(())
        }

        pub(super) fn assert_return_canonical_nan<'a>(
            stack: &mut Vec<tree_sitter::Node<'a>>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            if let Some(node) = cursor.node().child(1) {
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            goto_next(cursor, stack, done);

            Ok(())
        }

        pub(super) fn assert_trap_action<'a>(
            stack: &mut Vec<tree_sitter::Node<'a>>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            if let Some(node) = cursor.node().child(1) {
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            goto_next(cursor, stack, done);

            Ok(())
        }

        pub(super) fn assert_trap_module<'a>(
            stack: &mut Vec<tree_sitter::Node<'a>>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            if let Some(node) = cursor.node().child(1) {
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            goto_next(cursor, stack, done);

            Ok(())
        }

        pub(super) fn assert_unlinkable<'a>(
            stack: &mut Vec<tree_sitter::Node<'a>>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            if let Some(node) = cursor.node().child(1) {
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            goto_next(cursor, stack, done);

            Ok(())
        }

        pub(super) fn command<'a>(
            _stack: &mut Vec<tree_sitter::Node<'a>>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            _builder: &mut SemanticTokensBuilder<'a>,
            _done: &mut bool,
        ) -> anyhow::Result<()> {
            cursor.goto_first_child();

            Ok(())
        }

        pub(super) fn comment_block<'a>(
            stack: &mut Vec<tree_sitter::Node<'a>>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            let range = crate::util::node::range(&cursor.node());
            builder.push(range, &SemanticTokenType::COMMENT, None)?;

            goto_next(cursor, stack, done);

            Ok(())
        }

        pub(super) fn comment_block_annot<'a>(
            stack: &mut Vec<tree_sitter::Node<'a>>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            let range = crate::util::node::range(&cursor.node());
            builder.push(range, &SemanticTokenType::COMMENT, None)?;

            goto_next(cursor, stack, done);

            Ok(())
        }

        pub(super) fn comment_line<'a>(
            stack: &mut Vec<tree_sitter::Node<'a>>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            let range = crate::util::node::range(&cursor.node());
            builder.push(range, &SemanticTokenType::COMMENT, None)?;

            goto_next(cursor, stack, done);

            Ok(())
        }

        pub(super) fn comment_line_annot<'a>(
            stack: &mut Vec<tree_sitter::Node<'a>>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            let range = crate::util::node::range(&cursor.node());
            builder.push(range, &SemanticTokenType::COMMENT, None)?;

            goto_next(cursor, stack, done);

            Ok(())
        }

        pub(super) fn export<'a>(
            stack: &mut Vec<tree_sitter::Node<'a>>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            // "("
            cursor.goto_first_child();

            // "export"
            cursor.goto_next_sibling();
            {
                let node = cursor.node();
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            // $.name
            cursor.goto_next_sibling();
            {
                let node = cursor.node();
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::STRING, None)?;
            }

            // skip ")"
            cursor.goto_next_sibling();

            goto_next(cursor, stack, done);

            Ok(())
        }

        pub(super) fn import<'a>(
            stack: &mut Vec<tree_sitter::Node<'a>>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            // "("
            cursor.goto_first_child();

            // "import"
            cursor.goto_next_sibling();
            {
                let node = cursor.node();
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            // $.name
            cursor.goto_next_sibling();
            {
                let node = cursor.node();
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::STRING, None)?;
            }

            // $.name
            cursor.goto_next_sibling();
            {
                let node = cursor.node();
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::STRING, None)?;
            }

            // skip ")"
            cursor.goto_next_sibling();

            goto_next(cursor, stack, done);

            Ok(())
        }

        pub(super) fn meta_input<'a>(
            stack: &mut Vec<tree_sitter::Node<'a>>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            if let Some(node) = cursor.node().child(1) {
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            goto_next(cursor, stack, done);

            Ok(())
        }

        pub(super) fn meta_output<'a>(
            stack: &mut Vec<tree_sitter::Node<'a>>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            if let Some(node) = cursor.node().child(1) {
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            goto_next(cursor, stack, done);

            Ok(())
        }

        pub(super) fn meta_script<'a>(
            stack: &mut Vec<tree_sitter::Node<'a>>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            if let Some(node) = cursor.node().child(1) {
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            goto_next(cursor, stack, done);

            Ok(())
        }

        pub(super) fn module<'a>(
            stack: &mut Vec<tree_sitter::Node<'a>>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            if let Some(node) = cursor.node().child(1) {
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            goto_next(cursor, stack, done);

            Ok(())
        }

        pub(super) fn module_field_data<'a>(
            stack: &mut Vec<tree_sitter::Node<'a>>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            if let Some(node) = cursor.node().child(1) {
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            goto_next(cursor, stack, done);

            Ok(())
        }

        pub(super) fn module_field_elem<'a>(
            stack: &mut Vec<tree_sitter::Node<'a>>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            if let Some(node) = cursor.node().child(1) {
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            goto_next(cursor, stack, done);

            Ok(())
        }

        pub(super) fn module_field_export<'a>(
            stack: &mut Vec<tree_sitter::Node<'a>>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            if let Some(node) = cursor.node().child(1) {
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            goto_next(cursor, stack, done);

            Ok(())
        }

        pub(super) fn module_field_func<'a>(
            stack: &mut Vec<tree_sitter::Node<'a>>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            // "("
            cursor.goto_first_child();

            // "func"
            cursor.goto_next_sibling();
            {
                let node = cursor.node();
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
                cursor.goto_next_sibling();
            }

            // optional($.identifier)
            if wast::kind::equals::IDENTIFIER(cursor.node().kind_id()) {
                let node = cursor.node();
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::FUNCTION, None)?;
                cursor.goto_next_sibling();
            }

            // repeat($.export)
            while wast::kind::equals::EXPORT(cursor.node().kind_id()) {
                export(stack, cursor, builder, done)?;
            }

            // optional($.import)
            if wast::kind::equals::IMPORT(cursor.node().kind_id()) {
                import(stack, cursor, builder, done)?;
            }

            // optional($.type_use)
            if wast::kind::equals::TYPE_USE(cursor.node().kind_id()) {
                type_use(stack, cursor, builder, done)?;
            }

            goto_next(cursor, stack, done);

            Ok(())
        }

        pub(super) fn module_field_global<'a>(
            stack: &mut Vec<tree_sitter::Node<'a>>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            // "("
            cursor.goto_first_child();

            // "global"
            cursor.goto_next_sibling();
            {
                let node = cursor.node();
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
                cursor.goto_next_sibling();
            }

            // optional($.identifier)
            if wast::kind::equals::IDENTIFIER(cursor.node().kind_id()) {
                let node = cursor.node();
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::FUNCTION, None)?;
                cursor.goto_next_sibling();
            }

            // repeat($.export)
            while wast::kind::equals::EXPORT(cursor.node().kind_id()) {
                export(stack, cursor, builder, done)?;
            }

            // optional($.import)
            if wast::kind::equals::IMPORT(cursor.node().kind_id()) {
                import(stack, cursor, builder, done)?;
            }

            goto_next(cursor, stack, done);

            Ok(())
        }

        pub(super) fn module_field_import<'a>(
            stack: &mut Vec<tree_sitter::Node<'a>>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            if let Some(node) = cursor.node().child(1) {
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            goto_next(cursor, stack, done);

            Ok(())
        }

        pub(super) fn module_field_memory<'a>(
            stack: &mut Vec<tree_sitter::Node<'a>>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            if let Some(node) = cursor.node().child(1) {
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            goto_next(cursor, stack, done);

            Ok(())
        }

        pub(super) fn module_field_start<'a>(
            stack: &mut Vec<tree_sitter::Node<'a>>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            if let Some(node) = cursor.node().child(1) {
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            goto_next(cursor, stack, done);

            Ok(())
        }

        pub(super) fn module_field_table<'a>(
            stack: &mut Vec<tree_sitter::Node<'a>>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            if let Some(node) = cursor.node().child(1) {
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            goto_next(cursor, stack, done);

            Ok(())
        }

        pub(super) fn module_field_type<'a>(
            stack: &mut Vec<tree_sitter::Node<'a>>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            if let Some(node) = cursor.node().child(1) {
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            goto_next(cursor, stack, done);

            Ok(())
        }

        pub(super) fn register<'a>(
            stack: &mut Vec<tree_sitter::Node<'a>>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            if let Some(node) = cursor.node().child(1) {
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            goto_next(cursor, stack, done);

            Ok(())
        }

        pub(super) fn root<'a>(
            stack: &mut Vec<tree_sitter::Node<'a>>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            _builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            goto_next(cursor, stack, done);

            Ok(())
        }

        pub(super) fn script_module_binary<'a>(
            stack: &mut Vec<tree_sitter::Node<'a>>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            if let Some(node) = cursor.node().child(1) {
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            goto_next(cursor, stack, done);

            Ok(())
        }

        pub(super) fn script_module_quote<'a>(
            stack: &mut Vec<tree_sitter::Node<'a>>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            if let Some(node) = cursor.node().child(1) {
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            goto_next(cursor, stack, done);

            Ok(())
        }

        pub(super) fn type_use<'a>(
            stack: &mut Vec<tree_sitter::Node<'a>>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            // "("
            cursor.goto_first_child();

            // "type"
            cursor.goto_next_sibling();
            {
                let node = cursor.node();
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            // $.index
            cursor.goto_next_sibling();
            {
                let node = cursor.node();
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::VARIABLE, None)?;
            }

            // skip ")"
            cursor.goto_parent();

            goto_next(cursor, stack, done);

            Ok(())
        }
    }
}

/// Semantic tokens provider definitions for ".wat" files.
pub mod wat {
    use super::goto_next;
    use crate::{
        core::{document::Document, language::wat, session::Session},
        provider::semantic_tokens::SemanticTokensBuilder,
    };
    use anyhow::anyhow;
    use lspower::lsp_types::*;
    use std::sync::Arc;

    pub(crate) async fn full(
        session: Arc<Session>,
        document: &Document,
        params: SemanticTokensParams,
    ) -> anyhow::Result<Option<SemanticTokensResult>> {
        let params = SemanticTokensRangeParams {
            work_done_progress_params: params.work_done_progress_params,
            partial_result_params: params.partial_result_params,
            text_document: params.text_document,
            range: {
                let tree = document.tree.lock().await;
                let node = tree.root_node();
                crate::util::node::range(&node)
            },
        };

        let result = range(session, document, params).await?.map(|result| match result {
            SemanticTokensRangeResult::Tokens(tokens) => SemanticTokensResult::Tokens(tokens),
            SemanticTokensRangeResult::Partial(partial) => SemanticTokensResult::Partial(partial),
        });

        Ok(result)
    }

    pub(crate) async fn range(
        session: Arc<Session>,
        document: &Document,
        params: SemanticTokensRangeParams,
    ) -> anyhow::Result<Option<SemanticTokensRangeResult>> {
        let legend = session.semantic_tokens_legend().await;

        if let Some(node) = {
            let start = tree_sitter::Point {
                row: params.range.start.line as usize,
                column: params.range.start.character as usize,
            };
            let end = tree_sitter::Point {
                row: params.range.end.line as usize,
                column: params.range.end.character as usize,
            };
            document
                .tree
                .lock()
                .await
                .root_node()
                .descendant_for_point_range(start, end)
        } {
            let mut stack = vec![];
            let mut builder = SemanticTokensBuilder::new(legend.as_ref());
            let mut cursor = node.walk();
            let mut done = false;

            // If this node isn't ROOT, reconstruct the stack before starting.
            if !wat::kind::equals::ROOT(node.kind_id()) {
                while cursor.goto_parent() {
                    stack.push(cursor.node());
                }
                stack.reverse();
                cursor.reset(node);
            }

            loop {
                if done {
                    break;
                }

                // handle "root"
                if wat::kind::equals::ROOT(cursor.node().kind_id()) {
                    handle::root(&mut stack, &mut cursor, &mut builder, &mut done)?;
                    continue;
                }

                // handle {"comment_block", "comment_block_annot", "comment_line", "comment_line_annot"}
                if wat::kind::equals::COMMENT_BLOCK(cursor.node().kind_id()) {
                    handle::comment_block(&mut stack, &mut cursor, &mut builder, &mut done)?;
                    continue;
                } else if wat::kind::equals::COMMENT_BLOCK_ANNOT(cursor.node().kind_id()) {
                    handle::comment_block_annot(&mut stack, &mut cursor, &mut builder, &mut done)?;
                    continue;
                } else if wat::kind::equals::COMMENT_LINE(cursor.node().kind_id()) {
                    handle::comment_line(&mut stack, &mut cursor, &mut builder, &mut done)?;
                    continue;
                } else if wat::kind::equals::COMMENT_LINE_ANNOT(cursor.node().kind_id()) {
                    handle::comment_line_annot(&mut stack, &mut cursor, &mut builder, &mut done)?;
                    continue;
                }

                // handle "module"
                if wat::kind::equals::MODULE(cursor.node().kind_id()) {
                    handle::module(&mut stack, &mut cursor, &mut builder, &mut done)?;
                    continue;
                }

                // handle "_module_field"
                if wat::kind::equals::MODULE_FIELD_DATA(cursor.node().kind_id()) {
                    handle::module_field_data(&mut stack, &mut cursor, &mut builder, &mut done)?;
                    continue;
                } else if wat::kind::equals::MODULE_FIELD_ELEM(cursor.node().kind_id()) {
                    handle::module_field_elem(&mut stack, &mut cursor, &mut builder, &mut done)?;
                    continue;
                } else if wat::kind::equals::MODULE_FIELD_EXPORT(cursor.node().kind_id()) {
                    handle::module_field_export(&mut stack, &mut cursor, &mut builder, &mut done)?;
                    continue;
                } else if wat::kind::equals::MODULE_FIELD_FUNC(cursor.node().kind_id()) {
                    handle::module_field_func(&mut stack, &mut cursor, &mut builder, &mut done)?;
                    continue;
                } else if wat::kind::equals::MODULE_FIELD_GLOBAL(cursor.node().kind_id()) {
                    handle::module_field_global(&mut stack, &mut cursor, &mut builder, &mut done)?;
                    continue;
                } else if wat::kind::equals::MODULE_FIELD_IMPORT(cursor.node().kind_id()) {
                    handle::module_field_import(&mut stack, &mut cursor, &mut builder, &mut done)?;
                    continue;
                } else if wat::kind::equals::MODULE_FIELD_MEMORY(cursor.node().kind_id()) {
                    handle::module_field_memory(&mut stack, &mut cursor, &mut builder, &mut done)?;
                    continue;
                } else if wat::kind::equals::MODULE_FIELD_START(cursor.node().kind_id()) {
                    handle::module_field_start(&mut stack, &mut cursor, &mut builder, &mut done)?;
                    continue;
                } else if wat::kind::equals::MODULE_FIELD_TABLE(cursor.node().kind_id()) {
                    handle::module_field_table(&mut stack, &mut cursor, &mut builder, &mut done)?;
                    continue;
                } else if wat::kind::equals::MODULE_FIELD_TYPE(cursor.node().kind_id()) {
                    handle::module_field_type(&mut stack, &mut cursor, &mut builder, &mut done)?;
                    continue;
                }

                // FIXME: catch all case
                goto_next(&mut cursor, &mut stack, &mut done);
            }

            let tokens = builder.build()?;
            let result = SemanticTokensRangeResult::Tokens(tokens);

            Ok(Some(result))
        } else {
            Err(anyhow!("Could not obtain tree node for given range"))
        }
    }

    mod handle {
        use super::super::goto_next;
        use crate::{core::language::wat, provider::semantic_tokens::SemanticTokensBuilder};
        use lspower::lsp_types::*;

        pub(super) fn comment_block<'a>(
            stack: &mut Vec<tree_sitter::Node<'a>>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            let range = crate::util::node::range(&cursor.node());
            builder.push(range, &SemanticTokenType::COMMENT, None)?;

            goto_next(cursor, stack, done);

            Ok(())
        }

        pub(super) fn comment_block_annot<'a>(
            stack: &mut Vec<tree_sitter::Node<'a>>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            let range = crate::util::node::range(&cursor.node());
            builder.push(range, &SemanticTokenType::COMMENT, None)?;

            goto_next(cursor, stack, done);

            Ok(())
        }

        pub(super) fn comment_line<'a>(
            stack: &mut Vec<tree_sitter::Node<'a>>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            let range = crate::util::node::range(&cursor.node());
            builder.push(range, &SemanticTokenType::COMMENT, None)?;

            goto_next(cursor, stack, done);

            Ok(())
        }

        pub(super) fn comment_line_annot<'a>(
            stack: &mut Vec<tree_sitter::Node<'a>>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            let range = crate::util::node::range(&cursor.node());
            builder.push(range, &SemanticTokenType::COMMENT, None)?;

            goto_next(cursor, stack, done);

            Ok(())
        }

        pub(super) fn export<'a>(
            stack: &mut Vec<tree_sitter::Node<'a>>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            // "("
            cursor.goto_first_child();

            // "export"
            cursor.goto_next_sibling();
            {
                let node = cursor.node();
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            // $.name
            cursor.goto_next_sibling();
            {
                let node = cursor.node();
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::STRING, None)?;
            }

            // skip ")"
            cursor.goto_next_sibling();

            goto_next(cursor, stack, done);

            Ok(())
        }

        pub(super) fn import<'a>(
            stack: &mut Vec<tree_sitter::Node<'a>>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            // "("
            cursor.goto_first_child();

            // "import"
            cursor.goto_next_sibling();
            {
                let node = cursor.node();
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            // $.name
            cursor.goto_next_sibling();
            {
                let node = cursor.node();
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::STRING, None)?;
            }

            // $.name
            cursor.goto_next_sibling();
            {
                let node = cursor.node();
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::STRING, None)?;
            }

            // skip ")"
            cursor.goto_next_sibling();

            goto_next(cursor, stack, done);

            Ok(())
        }

        pub(super) fn module<'a>(
            stack: &mut Vec<tree_sitter::Node<'a>>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            if let Some(node) = cursor.node().child(1) {
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            goto_next(cursor, stack, done);

            Ok(())
        }

        pub(super) fn module_field_data<'a>(
            stack: &mut Vec<tree_sitter::Node<'a>>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            if let Some(node) = cursor.node().child(1) {
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            goto_next(cursor, stack, done);

            Ok(())
        }

        pub(super) fn module_field_elem<'a>(
            stack: &mut Vec<tree_sitter::Node<'a>>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            if let Some(node) = cursor.node().child(1) {
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            goto_next(cursor, stack, done);

            Ok(())
        }

        pub(super) fn module_field_export<'a>(
            stack: &mut Vec<tree_sitter::Node<'a>>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            if let Some(node) = cursor.node().child(1) {
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            goto_next(cursor, stack, done);

            Ok(())
        }

        pub(super) fn module_field_func<'a>(
            stack: &mut Vec<tree_sitter::Node<'a>>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            // "("
            cursor.goto_first_child();

            // "func"
            cursor.goto_next_sibling();
            {
                let node = cursor.node();
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
                cursor.goto_next_sibling();
            }

            // optional($.identifier)
            if wat::kind::equals::IDENTIFIER(cursor.node().kind_id()) {
                let node = cursor.node();
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::FUNCTION, None)?;
                cursor.goto_next_sibling();
            }

            // repeat($.export)
            while wat::kind::equals::EXPORT(cursor.node().kind_id()) {
                export(stack, cursor, builder, done)?;
            }

            // optional($.import)
            if wat::kind::equals::IMPORT(cursor.node().kind_id()) {
                import(stack, cursor, builder, done)?;
            }

            // optional($.type_use)
            if wat::kind::equals::TYPE_USE(cursor.node().kind_id()) {
                type_use(stack, cursor, builder, done)?;
            }

            goto_next(cursor, stack, done);

            Ok(())
        }

        pub(super) fn module_field_global<'a>(
            stack: &mut Vec<tree_sitter::Node<'a>>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            // "("
            cursor.goto_first_child();

            // "global"
            cursor.goto_next_sibling();
            {
                let node = cursor.node();
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
                cursor.goto_next_sibling();
            }

            // optional($.identifier)
            if wat::kind::equals::IDENTIFIER(cursor.node().kind_id()) {
                let node = cursor.node();
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::FUNCTION, None)?;
                cursor.goto_next_sibling();
            }

            // repeat($.export)
            while wat::kind::equals::EXPORT(cursor.node().kind_id()) {
                export(stack, cursor, builder, done)?;
            }

            // optional($.import)
            if wat::kind::equals::IMPORT(cursor.node().kind_id()) {
                import(stack, cursor, builder, done)?;
            }

            goto_next(cursor, stack, done);

            Ok(())
        }

        pub(super) fn module_field_import<'a>(
            stack: &mut Vec<tree_sitter::Node<'a>>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            if let Some(node) = cursor.node().child(1) {
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            goto_next(cursor, stack, done);

            Ok(())
        }

        pub(super) fn module_field_memory<'a>(
            stack: &mut Vec<tree_sitter::Node<'a>>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            if let Some(node) = cursor.node().child(1) {
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            goto_next(cursor, stack, done);

            Ok(())
        }

        pub(super) fn module_field_start<'a>(
            stack: &mut Vec<tree_sitter::Node<'a>>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            if let Some(node) = cursor.node().child(1) {
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            goto_next(cursor, stack, done);

            Ok(())
        }

        pub(super) fn module_field_table<'a>(
            stack: &mut Vec<tree_sitter::Node<'a>>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            if let Some(node) = cursor.node().child(1) {
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            goto_next(cursor, stack, done);

            Ok(())
        }

        pub(super) fn module_field_type<'a>(
            stack: &mut Vec<tree_sitter::Node<'a>>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            if let Some(node) = cursor.node().child(1) {
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            goto_next(cursor, stack, done);

            Ok(())
        }

        pub(super) fn root<'a>(
            stack: &mut Vec<tree_sitter::Node<'a>>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            _builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            goto_next(cursor, stack, done);

            Ok(())
        }

        pub(super) fn type_use<'a>(
            stack: &mut Vec<tree_sitter::Node<'a>>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            // "("
            cursor.goto_first_child();

            // "type"
            cursor.goto_next_sibling();
            {
                let node = cursor.node();
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            // $.index
            cursor.goto_next_sibling();
            {
                let node = cursor.node();
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::VARIABLE, None)?;
            }

            // skip ")"
            cursor.goto_parent();

            goto_next(cursor, stack, done);

            Ok(())
        }
    }
}
