//! Provides `textDocument/semanticTokens/*` functionality.

use anyhow::anyhow;
use lspower::lsp_types::*;
use std::collections::HashMap;

#[derive(Clone, Debug, Default)]
pub(crate) struct SemanticTokensBuilder<'a> {
    prev_start: u32,
    prev_line: u32,
    data_is_sorted_and_delta_encoded: bool,
    data: Vec<SemanticToken>,
    token_modifier_map: HashMap<&'a SemanticTokenModifier, u32>,
    token_type_map: HashMap<&'a SemanticTokenType, u32>,
    has_legend: bool,
}

impl<'a> SemanticTokensBuilder<'a> {
    pub(crate) fn new(legend: Option<&'a SemanticTokensLegend>) -> Self {
        let data_is_sorted_and_delta_encoded = true;

        let mut token_modifier_map = HashMap::new();
        let mut token_type_map = HashMap::new();
        let mut has_legend = false;

        if let Some(legend) = legend {
            has_legend = true;

            for (i, token_type) in legend.token_types.iter().enumerate() {
                let _ = token_type_map.insert(token_type, i as u32);
            }

            for (i, token_modifier) in legend.token_modifiers.iter().enumerate() {
                let _ = token_modifier_map.insert(token_modifier, i as u32);
            }
        }

        SemanticTokensBuilder {
            data_is_sorted_and_delta_encoded,
            token_modifier_map,
            token_type_map,
            has_legend,
            ..Default::default()
        }
    }

    pub(crate) fn build(self) -> anyhow::Result<SemanticTokens> {
        let data = if !self.data_is_sorted_and_delta_encoded {
            Self::sort_and_delta_encode(&self.data)
        } else {
            self.data
        };

        Ok(SemanticTokens {
            data,
            ..Default::default()
        })
    }

    pub(crate) fn push(
        &mut self,
        range: Range,
        token_type: &SemanticTokenType,
        token_modifiers: Option<Vec<&SemanticTokenModifier>>,
    ) -> anyhow::Result<()> {
        if !self.has_legend {
            return Err(anyhow!("Legend must be provided in constructor"));
        }

        // FIXME: support multiline
        if range.start.line != range.end.line {
            return Err(anyhow!("`range` cannot span multiple lines"));
        }

        if let Some(&n_token_type) = self.token_type_map.get(token_type) {
            let line = range.start.line;
            let char = range.start.character;
            let length = range.end.character - range.start.character;

            let mut n_token_modifiers = 0;

            if let Some(token_modifiers) = token_modifiers {
                for token_modifier in token_modifiers {
                    if let Some(n_token_modifier) = self.token_modifier_map.get(token_modifier) {
                        n_token_modifiers |= 1 << n_token_modifier;
                    } else {
                        return Err(anyhow!("`token_modifier` is not in the provided legend"));
                    }
                }
            }

            self.push_encoded(line, char, length, n_token_type, n_token_modifiers)?;
        } else {
            return Err(anyhow!("`token_type` is not in the provided legend"));
        }

        Ok(())
    }

    fn push_encoded(
        &mut self,
        line: u32,
        char: u32,
        length: u32,
        token_type: u32,
        token_modifiers_bitset: u32,
    ) -> anyhow::Result<()> {
        #[allow(clippy::clippy::collapsible_if)]
        if self.data_is_sorted_and_delta_encoded {
            if line < self.prev_line || (line == self.prev_line && char < self.prev_start) {
                // Push calls were ordered and are no longer ordered.
                self.data_is_sorted_and_delta_encoded = false;

                // Remove delta encoding from data.
                let mut prev_line = 0;
                let mut prev_start = 0;

                for i in 0 .. self.data.len() {
                    let mut delta_line = self.data[i].delta_line;
                    let mut delta_start = self.data[i].delta_start;

                    if delta_line == 0 {
                        delta_line = prev_line;
                        delta_start += prev_start;
                    } else {
                        delta_line += prev_line;
                    }

                    self.data[i].delta_line = delta_line;
                    self.data[i].delta_start = delta_start;

                    prev_line = delta_line;
                    prev_start = delta_start;
                }
            }
        }

        let mut delta_line = line;
        let mut delta_start = char;

        if self.data_is_sorted_and_delta_encoded && !self.data.is_empty() {
            delta_line -= self.prev_line;
            if delta_line == 0 {
                delta_start -= self.prev_start;
            }
        }

        self.data.push(SemanticToken {
            delta_line,
            delta_start,
            length,
            token_type,
            token_modifiers_bitset,
        });

        self.prev_line = line;
        self.prev_start = char;

        Ok(())
    }

    pub(crate) fn sort_and_delta_encode(data: &[SemanticToken]) -> Vec<SemanticToken> {
        let pos = {
            let mut pos = (0 .. data.len()).collect::<Vec<_>>();
            pos.sort_by(|&a, &b| {
                let a_line = data[a].delta_line;
                let b_line = data[b].delta_line;

                if a_line == b_line {
                    let a_start = data[a].delta_start;
                    let b_start = data[b].delta_start;
                    a_start.partial_cmp(&b_start).unwrap()
                } else {
                    a_line.partial_cmp(&b_line).unwrap()
                }
            });
            pos
        };

        let mut result = Vec::with_capacity(data.len());
        let mut prev_line = 0;
        let mut prev_start = 0;

        for i in 0 .. data.len() {
            let token = data[pos[i]];

            let delta_line = token.delta_line - prev_line;
            let delta_start = if delta_line == 0 {
                token.delta_start - prev_start
            } else {
                token.delta_start
            };

            result.push(SemanticToken {
                delta_line,
                delta_start,
                ..token
            });

            prev_line = delta_line;
            prev_start = delta_start;
        }

        result
    }
}

// Move to the next appropriate node in the syntax tree.
fn goto_next(cursor: &mut tree_sitter::TreeCursor, done: &mut bool) {
    // First try to descend to the first child node.
    if !cursor.goto_first_child() {
        // Otherwise try to move to the next sibling node.
        if !cursor.goto_next_sibling() {
            let mut finished = true;
            // Otherwise continue to ascend to parent nodes...
            while cursor.goto_parent() {
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
            _stack: &mut Vec<tree_sitter::Node>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            if let Some(node) = cursor.node().child(1) {
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            goto_next(cursor, done);

            Ok(())
        }

        pub(super) fn action_invoke<'a>(
            _stack: &mut Vec<tree_sitter::Node>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            if let Some(node) = cursor.node().child(1) {
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            goto_next(cursor, done);

            Ok(())
        }

        pub(super) fn assert_exhaustion<'a>(
            _stack: &mut Vec<tree_sitter::Node>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            if let Some(node) = cursor.node().child(1) {
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            goto_next(cursor, done);

            Ok(())
        }

        pub(super) fn assert_invalid<'a>(
            _stack: &mut Vec<tree_sitter::Node>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            if let Some(node) = cursor.node().child(1) {
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            goto_next(cursor, done);

            Ok(())
        }

        pub(super) fn assert_malformed<'a>(
            _stack: &mut Vec<tree_sitter::Node>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            if let Some(node) = cursor.node().child(1) {
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            goto_next(cursor, done);

            Ok(())
        }

        pub(super) fn assert_return<'a>(
            _stack: &mut Vec<tree_sitter::Node>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            if let Some(node) = cursor.node().child(1) {
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            goto_next(cursor, done);

            Ok(())
        }

        pub(super) fn assert_return_arithmetic_nan<'a>(
            _stack: &mut Vec<tree_sitter::Node>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            if let Some(node) = cursor.node().child(1) {
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            goto_next(cursor, done);

            Ok(())
        }

        pub(super) fn assert_return_canonical_nan<'a>(
            _stack: &mut Vec<tree_sitter::Node>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            if let Some(node) = cursor.node().child(1) {
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            goto_next(cursor, done);

            Ok(())
        }

        pub(super) fn assert_trap_action<'a>(
            _stack: &mut Vec<tree_sitter::Node>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            if let Some(node) = cursor.node().child(1) {
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            goto_next(cursor, done);

            Ok(())
        }

        pub(super) fn assert_trap_module<'a>(
            _stack: &mut Vec<tree_sitter::Node>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            if let Some(node) = cursor.node().child(1) {
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            goto_next(cursor, done);

            Ok(())
        }

        pub(super) fn assert_unlinkable<'a>(
            _stack: &mut Vec<tree_sitter::Node>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            if let Some(node) = cursor.node().child(1) {
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            goto_next(cursor, done);

            Ok(())
        }

        pub(super) fn command<'a>(
            _stack: &mut Vec<tree_sitter::Node>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            _builder: &mut SemanticTokensBuilder<'a>,
            _done: &mut bool,
        ) -> anyhow::Result<()> {
            cursor.goto_first_child();

            Ok(())
        }

        pub(super) fn comment_block<'a>(
            _stack: &mut Vec<tree_sitter::Node>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            let range = crate::util::node::range(&cursor.node());
            builder.push(range, &SemanticTokenType::COMMENT, None)?;

            goto_next(cursor, done);

            Ok(())
        }

        pub(super) fn comment_block_annot<'a>(
            _stack: &mut Vec<tree_sitter::Node>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            let range = crate::util::node::range(&cursor.node());
            builder.push(range, &SemanticTokenType::COMMENT, None)?;

            goto_next(cursor, done);

            Ok(())
        }

        pub(super) fn comment_line<'a>(
            _stack: &mut Vec<tree_sitter::Node>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            let range = crate::util::node::range(&cursor.node());
            builder.push(range, &SemanticTokenType::COMMENT, None)?;

            goto_next(cursor, done);

            Ok(())
        }

        pub(super) fn comment_line_annot<'a>(
            _stack: &mut Vec<tree_sitter::Node>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            let range = crate::util::node::range(&cursor.node());
            builder.push(range, &SemanticTokenType::COMMENT, None)?;

            goto_next(cursor, done);

            Ok(())
        }

        pub(super) fn export<'a>(
            _stack: &mut Vec<tree_sitter::Node>,
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

            goto_next(cursor, done);

            Ok(())
        }

        pub(super) fn import<'a>(
            _stack: &mut Vec<tree_sitter::Node>,
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

            goto_next(cursor, done);

            Ok(())
        }

        pub(super) fn meta_input<'a>(
            _stack: &mut Vec<tree_sitter::Node>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            if let Some(node) = cursor.node().child(1) {
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            goto_next(cursor, done);

            Ok(())
        }

        pub(super) fn meta_output<'a>(
            _stack: &mut Vec<tree_sitter::Node>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            if let Some(node) = cursor.node().child(1) {
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            goto_next(cursor, done);

            Ok(())
        }

        pub(super) fn meta_script<'a>(
            _stack: &mut Vec<tree_sitter::Node>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            if let Some(node) = cursor.node().child(1) {
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            goto_next(cursor, done);

            Ok(())
        }

        pub(super) fn module<'a>(
            _stack: &mut Vec<tree_sitter::Node>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            if let Some(node) = cursor.node().child(1) {
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            goto_next(cursor, done);

            Ok(())
        }

        pub(super) fn module_field_data<'a>(
            _stack: &mut Vec<tree_sitter::Node>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            if let Some(node) = cursor.node().child(1) {
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            goto_next(cursor, done);

            Ok(())
        }

        pub(super) fn module_field_elem<'a>(
            _stack: &mut Vec<tree_sitter::Node>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            if let Some(node) = cursor.node().child(1) {
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            goto_next(cursor, done);

            Ok(())
        }

        pub(super) fn module_field_export<'a>(
            _stack: &mut Vec<tree_sitter::Node>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            if let Some(node) = cursor.node().child(1) {
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            goto_next(cursor, done);

            Ok(())
        }

        pub(super) fn module_field_func<'a>(
            stack: &mut Vec<tree_sitter::Node>,
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

            goto_next(cursor, done);

            Ok(())
        }

        pub(super) fn module_field_global<'a>(
            stack: &mut Vec<tree_sitter::Node>,
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

            goto_next(cursor, done);

            Ok(())
        }

        pub(super) fn module_field_import<'a>(
            _stack: &mut Vec<tree_sitter::Node>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            if let Some(node) = cursor.node().child(1) {
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            goto_next(cursor, done);

            Ok(())
        }

        pub(super) fn module_field_memory<'a>(
            _stack: &mut Vec<tree_sitter::Node>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            if let Some(node) = cursor.node().child(1) {
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            goto_next(cursor, done);

            Ok(())
        }

        pub(super) fn module_field_start<'a>(
            _stack: &mut Vec<tree_sitter::Node>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            if let Some(node) = cursor.node().child(1) {
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            goto_next(cursor, done);

            Ok(())
        }

        pub(super) fn module_field_table<'a>(
            _stack: &mut Vec<tree_sitter::Node>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            if let Some(node) = cursor.node().child(1) {
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            goto_next(cursor, done);

            Ok(())
        }

        pub(super) fn module_field_type<'a>(
            _stack: &mut Vec<tree_sitter::Node>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            if let Some(node) = cursor.node().child(1) {
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            goto_next(cursor, done);

            Ok(())
        }

        pub(super) fn register<'a>(
            _stack: &mut Vec<tree_sitter::Node>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            if let Some(node) = cursor.node().child(1) {
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            goto_next(cursor, done);

            Ok(())
        }

        pub(super) fn root<'a>(
            _stack: &mut Vec<tree_sitter::Node>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            _builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            goto_next(cursor, done);

            Ok(())
        }

        pub(super) fn script_module_binary<'a>(
            _stack: &mut Vec<tree_sitter::Node>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            if let Some(node) = cursor.node().child(1) {
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            goto_next(cursor, done);

            Ok(())
        }

        pub(super) fn script_module_quote<'a>(
            _stack: &mut Vec<tree_sitter::Node>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            if let Some(node) = cursor.node().child(1) {
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            goto_next(cursor, done);

            Ok(())
        }

        pub(super) fn type_use<'a>(
            _stack: &mut Vec<tree_sitter::Node>,
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

            goto_next(cursor, done);

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
                goto_next(&mut cursor, &mut done);
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
            _stack: &mut Vec<tree_sitter::Node>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            let range = crate::util::node::range(&cursor.node());
            builder.push(range, &SemanticTokenType::COMMENT, None)?;

            goto_next(cursor, done);

            Ok(())
        }

        pub(super) fn comment_block_annot<'a>(
            _stack: &mut Vec<tree_sitter::Node>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            let range = crate::util::node::range(&cursor.node());
            builder.push(range, &SemanticTokenType::COMMENT, None)?;

            goto_next(cursor, done);

            Ok(())
        }

        pub(super) fn comment_line<'a>(
            _stack: &mut Vec<tree_sitter::Node>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            let range = crate::util::node::range(&cursor.node());
            builder.push(range, &SemanticTokenType::COMMENT, None)?;

            goto_next(cursor, done);

            Ok(())
        }

        pub(super) fn comment_line_annot<'a>(
            _stack: &mut Vec<tree_sitter::Node>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            let range = crate::util::node::range(&cursor.node());
            builder.push(range, &SemanticTokenType::COMMENT, None)?;

            goto_next(cursor, done);

            Ok(())
        }

        pub(super) fn export<'a>(
            _stack: &mut Vec<tree_sitter::Node>,
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

            goto_next(cursor, done);

            Ok(())
        }

        pub(super) fn import<'a>(
            _stack: &mut Vec<tree_sitter::Node>,
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

            goto_next(cursor, done);

            Ok(())
        }

        pub(super) fn module<'a>(
            _stack: &mut Vec<tree_sitter::Node>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            if let Some(node) = cursor.node().child(1) {
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            goto_next(cursor, done);

            Ok(())
        }

        pub(super) fn module_field_data<'a>(
            _stack: &mut Vec<tree_sitter::Node>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            if let Some(node) = cursor.node().child(1) {
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            goto_next(cursor, done);

            Ok(())
        }

        pub(super) fn module_field_elem<'a>(
            _stack: &mut Vec<tree_sitter::Node>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            if let Some(node) = cursor.node().child(1) {
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            goto_next(cursor, done);

            Ok(())
        }

        pub(super) fn module_field_export<'a>(
            _stack: &mut Vec<tree_sitter::Node>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            if let Some(node) = cursor.node().child(1) {
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            goto_next(cursor, done);

            Ok(())
        }

        pub(super) fn module_field_func<'a>(
            stack: &mut Vec<tree_sitter::Node>,
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

            goto_next(cursor, done);

            Ok(())
        }

        pub(super) fn module_field_global<'a>(
            stack: &mut Vec<tree_sitter::Node>,
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

            goto_next(cursor, done);

            Ok(())
        }

        pub(super) fn module_field_import<'a>(
            _stack: &mut Vec<tree_sitter::Node>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            if let Some(node) = cursor.node().child(1) {
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            goto_next(cursor, done);

            Ok(())
        }

        pub(super) fn module_field_memory<'a>(
            _stack: &mut Vec<tree_sitter::Node>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            if let Some(node) = cursor.node().child(1) {
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            goto_next(cursor, done);

            Ok(())
        }

        pub(super) fn module_field_start<'a>(
            _stack: &mut Vec<tree_sitter::Node>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            if let Some(node) = cursor.node().child(1) {
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            goto_next(cursor, done);

            Ok(())
        }

        pub(super) fn module_field_table<'a>(
            _stack: &mut Vec<tree_sitter::Node>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            if let Some(node) = cursor.node().child(1) {
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            goto_next(cursor, done);

            Ok(())
        }

        pub(super) fn module_field_type<'a>(
            _stack: &mut Vec<tree_sitter::Node>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            if let Some(node) = cursor.node().child(1) {
                let range = crate::util::node::range(&node);
                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
            }

            goto_next(cursor, done);

            Ok(())
        }

        pub(super) fn root<'a>(
            _stack: &mut Vec<tree_sitter::Node>,
            cursor: &mut tree_sitter::TreeCursor<'a>,
            _builder: &mut SemanticTokensBuilder<'a>,
            done: &mut bool,
        ) -> anyhow::Result<()> {
            goto_next(cursor, done);

            Ok(())
        }

        pub(super) fn type_use<'a>(
            _stack: &mut Vec<tree_sitter::Node>,
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

            goto_next(cursor, done);

            Ok(())
        }
    }
}
