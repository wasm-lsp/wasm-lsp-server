//! Semantic tokens provider definitions for ".wast" files.

use crate::{
    core::{document::Document, language::wast, session::Session},
    provider::semantic_tokens::SemanticTokensBuilder,
};
use anyhow::anyhow;
use lspower::lsp_types::*;
use std::{collections::VecDeque, sync::Arc};

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
        let mut builder = SemanticTokensBuilder::new(legend.as_ref());

        let mut cursor = node.walk();
        let mut work = {
            let mut res = VecDeque::new();
            res.push_back(node);
            res
        };

        while let Some(this) = work.pop_front() {
            // handle "_action"
            if wast::kind::equals::ACTION_GET(this.kind_id()) {
                handle::action_get(&mut cursor, &mut work, this, &mut builder)?;
                continue;
            } else if wast::kind::equals::ACTION_INVOKE(this.kind_id()) {
                handle::action_invoke(&mut cursor, &mut work, this, &mut builder)?;
                continue;
            }

            // handle "_assertion"
            if wast::kind::equals::ASSERT_EXHAUSTION(this.kind_id()) {
                handle::action_get(&mut cursor, &mut work, this, &mut builder)?;
                continue;
            } else if wast::kind::equals::ASSERT_INVALID(this.kind_id()) {
                handle::action_invoke(&mut cursor, &mut work, this, &mut builder)?;
                continue;
            } else if wast::kind::equals::ASSERT_EXHAUSTION(this.kind_id()) {
                handle::assert_exhaustion(&mut cursor, &mut work, this, &mut builder)?;
                continue;
            } else if wast::kind::equals::ASSERT_INVALID(this.kind_id()) {
                handle::assert_invalid(&mut cursor, &mut work, this, &mut builder)?;
                continue;
            } else if wast::kind::equals::ASSERT_MALFORMED(this.kind_id()) {
                handle::assert_malformed(&mut cursor, &mut work, this, &mut builder)?;
                continue;
            } else if wast::kind::equals::ASSERT_RETURN(this.kind_id()) {
                handle::assert_return(&mut cursor, &mut work, this, &mut builder)?;
                continue;
            } else if wast::kind::equals::ASSERT_RETURN_ARITHMETIC_NAN(this.kind_id()) {
                handle::assert_return_arithmetic_nan(&mut cursor, &mut work, this, &mut builder)?;
                continue;
            } else if wast::kind::equals::ASSERT_RETURN_CANONICAL_NAN(this.kind_id()) {
                handle::assert_return_canonical_nan(&mut cursor, &mut work, this, &mut builder)?;
                continue;
            } else if wast::kind::equals::ASSERT_TRAP_ACTION(this.kind_id()) {
                handle::assert_trap_action(&mut cursor, &mut work, this, &mut builder)?;
                continue;
            } else if wast::kind::equals::ASSERT_TRAP_MODULE(this.kind_id()) {
                handle::assert_trap_module(&mut cursor, &mut work, this, &mut builder)?;
                continue;
            } else if wast::kind::equals::ASSERT_UNLINKABLE(this.kind_id()) {
                handle::assert_unlinkable(&mut cursor, &mut work, this, &mut builder)?;
                continue;
            }

            // handle "command"
            if wast::kind::equals::COMMAND(this.kind_id()) {
                handle::command(&mut cursor, &mut work, this, &mut builder)?;
                continue;
            }

            // handle "_meta"
            if wast::kind::equals::META_INPUT(this.kind_id()) {
                handle::meta_input(&mut cursor, &mut work, this, &mut builder)?;
                continue;
            } else if wast::kind::equals::META_OUTPUT(this.kind_id()) {
                handle::meta_output(&mut cursor, &mut work, this, &mut builder)?;
                continue;
            } else if wast::kind::equals::META_SCRIPT(this.kind_id()) {
                handle::meta_script(&mut cursor, &mut work, this, &mut builder)?;
                continue;
            }

            // handle "module"
            if wast::kind::equals::MODULE(this.kind_id()) {
                handle::module(&mut cursor, &mut work, this, &mut builder)?;
                continue;
            }

            // handle "register"
            if wast::kind::equals::REGISTER(this.kind_id()) {
                handle::register(&mut cursor, &mut work, this, &mut builder)?;
                continue;
            }

            // handle "root"
            if wast::kind::equals::ROOT(this.kind_id()) {
                handle::root(&mut cursor, &mut work, this, &mut builder)?;
                continue;
            }

            // handle "_script_module"
            if wast::kind::equals::MODULE(this.kind_id()) {
                // handled earlier
                unreachable!();
            } else if wast::kind::equals::SCRIPT_MODULE_BINARY(this.kind_id()) {
                handle::script_module_binary(&mut cursor, &mut work, this, &mut builder)?;
                continue;
            } else if wast::kind::equals::SCRIPT_MODULE_QUOTE(this.kind_id()) {
                handle::script_module_quote(&mut cursor, &mut work, this, &mut builder)?;
                continue;
            }

            cursor.reset(this);
            work.extend(this.named_children(&mut cursor))
        }

        let tokens = builder.build()?;
        let result = SemanticTokensRangeResult::Tokens(tokens);

        Ok(Some(result))
    } else {
        Err(anyhow!("Could not obtain tree node for given range"))
    }
}

mod handle {
    use crate::{core::language::wast, provider::semantic_tokens::SemanticTokensBuilder};
    use lspower::lsp_types::*;
    use std::collections::VecDeque;

    pub(super) fn action_get<'a>(
        _cursor: &mut tree_sitter::TreeCursor<'a>,
        _work: &mut VecDeque<tree_sitter::Node<'a>>,
        this: tree_sitter::Node<'a>,
        builder: &mut SemanticTokensBuilder<'a>,
    ) -> anyhow::Result<()> {
        if let Some(node) = this.child(1) {
            let range = crate::util::node::range(&node);
            builder.push(range, &SemanticTokenType::KEYWORD, None)?;
        }

        Ok(())
    }

    pub(super) fn action_invoke<'a>(
        _cursor: &mut tree_sitter::TreeCursor<'a>,
        _work: &mut VecDeque<tree_sitter::Node<'a>>,
        this: tree_sitter::Node<'a>,
        builder: &mut SemanticTokensBuilder<'a>,
    ) -> anyhow::Result<()> {
        if let Some(node) = this.child(1) {
            let range = crate::util::node::range(&node);
            builder.push(range, &SemanticTokenType::KEYWORD, None)?;
        }

        Ok(())
    }

    pub(super) fn assert_exhaustion<'a>(
        _cursor: &mut tree_sitter::TreeCursor<'a>,
        _work: &mut VecDeque<tree_sitter::Node<'a>>,
        this: tree_sitter::Node<'a>,
        builder: &mut SemanticTokensBuilder<'a>,
    ) -> anyhow::Result<()> {
        if let Some(node) = this.child(1) {
            let range = crate::util::node::range(&node);
            builder.push(range, &SemanticTokenType::KEYWORD, None)?;
        }

        Ok(())
    }

    pub(super) fn assert_invalid<'a>(
        _cursor: &mut tree_sitter::TreeCursor<'a>,
        _work: &mut VecDeque<tree_sitter::Node<'a>>,
        this: tree_sitter::Node<'a>,
        builder: &mut SemanticTokensBuilder<'a>,
    ) -> anyhow::Result<()> {
        if let Some(node) = this.child(1) {
            let range = crate::util::node::range(&node);
            builder.push(range, &SemanticTokenType::KEYWORD, None)?;
        }

        Ok(())
    }

    pub(super) fn assert_malformed<'a>(
        _cursor: &mut tree_sitter::TreeCursor<'a>,
        _work: &mut VecDeque<tree_sitter::Node<'a>>,
        this: tree_sitter::Node<'a>,
        builder: &mut SemanticTokensBuilder<'a>,
    ) -> anyhow::Result<()> {
        if let Some(node) = this.child(1) {
            let range = crate::util::node::range(&node);
            builder.push(range, &SemanticTokenType::KEYWORD, None)?;
        }

        Ok(())
    }

    pub(super) fn assert_return<'a>(
        _cursor: &mut tree_sitter::TreeCursor<'a>,
        _work: &mut VecDeque<tree_sitter::Node<'a>>,
        this: tree_sitter::Node<'a>,
        builder: &mut SemanticTokensBuilder<'a>,
    ) -> anyhow::Result<()> {
        if let Some(node) = this.child(1) {
            let range = crate::util::node::range(&node);
            builder.push(range, &SemanticTokenType::KEYWORD, None)?;
        }

        Ok(())
    }

    pub(super) fn assert_return_arithmetic_nan<'a>(
        _cursor: &mut tree_sitter::TreeCursor<'a>,
        _work: &mut VecDeque<tree_sitter::Node<'a>>,
        this: tree_sitter::Node<'a>,
        builder: &mut SemanticTokensBuilder<'a>,
    ) -> anyhow::Result<()> {
        if let Some(node) = this.child(1) {
            let range = crate::util::node::range(&node);
            builder.push(range, &SemanticTokenType::KEYWORD, None)?;
        }

        Ok(())
    }

    pub(super) fn assert_return_canonical_nan<'a>(
        _cursor: &mut tree_sitter::TreeCursor<'a>,
        _work: &mut VecDeque<tree_sitter::Node<'a>>,
        this: tree_sitter::Node<'a>,
        builder: &mut SemanticTokensBuilder<'a>,
    ) -> anyhow::Result<()> {
        if let Some(node) = this.child(1) {
            let range = crate::util::node::range(&node);
            builder.push(range, &SemanticTokenType::KEYWORD, None)?;
        }

        Ok(())
    }

    pub(super) fn assert_trap_action<'a>(
        _cursor: &mut tree_sitter::TreeCursor<'a>,
        _work: &mut VecDeque<tree_sitter::Node<'a>>,
        this: tree_sitter::Node<'a>,
        builder: &mut SemanticTokensBuilder<'a>,
    ) -> anyhow::Result<()> {
        if let Some(node) = this.child(1) {
            let range = crate::util::node::range(&node);
            builder.push(range, &SemanticTokenType::KEYWORD, None)?;
        }

        Ok(())
    }

    pub(super) fn assert_trap_module<'a>(
        _cursor: &mut tree_sitter::TreeCursor<'a>,
        _work: &mut VecDeque<tree_sitter::Node<'a>>,
        this: tree_sitter::Node<'a>,
        builder: &mut SemanticTokensBuilder<'a>,
    ) -> anyhow::Result<()> {
        if let Some(node) = this.child(1) {
            let range = crate::util::node::range(&node);
            builder.push(range, &SemanticTokenType::KEYWORD, None)?;
        }

        Ok(())
    }

    pub(super) fn assert_unlinkable<'a>(
        _cursor: &mut tree_sitter::TreeCursor<'a>,
        _work: &mut VecDeque<tree_sitter::Node<'a>>,
        this: tree_sitter::Node<'a>,
        builder: &mut SemanticTokensBuilder<'a>,
    ) -> anyhow::Result<()> {
        if let Some(node) = this.child(1) {
            let range = crate::util::node::range(&node);
            builder.push(range, &SemanticTokenType::KEYWORD, None)?;
        }

        Ok(())
    }

    pub(super) fn command<'a>(
        _cursor: &mut tree_sitter::TreeCursor<'a>,
        work: &mut VecDeque<tree_sitter::Node<'a>>,
        this: tree_sitter::Node<'a>,
        _builder: &mut SemanticTokensBuilder<'a>,
    ) -> anyhow::Result<()> {
        if let Some(node) = this.child(0) {
            work.push_back(node);
        }

        Ok(())
    }

    pub(super) fn meta_input<'a>(
        _cursor: &mut tree_sitter::TreeCursor<'a>,
        work: &mut VecDeque<tree_sitter::Node<'a>>,
        this: tree_sitter::Node<'a>,
        _builder: &mut SemanticTokensBuilder<'a>,
    ) -> anyhow::Result<()> {
        if let Some(node) = this.child(1) {
            work.push_back(node);
        }

        Ok(())
    }

    pub(super) fn meta_output<'a>(
        _cursor: &mut tree_sitter::TreeCursor<'a>,
        work: &mut VecDeque<tree_sitter::Node<'a>>,
        this: tree_sitter::Node<'a>,
        _builder: &mut SemanticTokensBuilder<'a>,
    ) -> anyhow::Result<()> {
        if let Some(node) = this.child(1) {
            work.push_back(node);
        }

        Ok(())
    }

    pub(super) fn meta_script<'a>(
        _cursor: &mut tree_sitter::TreeCursor<'a>,
        work: &mut VecDeque<tree_sitter::Node<'a>>,
        this: tree_sitter::Node<'a>,
        _builder: &mut SemanticTokensBuilder<'a>,
    ) -> anyhow::Result<()> {
        if let Some(node) = this.child(1) {
            work.push_back(node);
        }

        Ok(())
    }

    pub(super) fn module<'a>(
        cursor: &mut tree_sitter::TreeCursor<'a>,
        work: &mut VecDeque<tree_sitter::Node<'a>>,
        this: tree_sitter::Node<'a>,
        builder: &mut SemanticTokensBuilder<'a>,
    ) -> anyhow::Result<()> {
        if let Some(node) = this.child(1) {
            let range = crate::util::node::range(&node);
            builder.push(range, &SemanticTokenType::KEYWORD, None)?;
        }

        cursor.reset(this);
        let fields = this
            .named_children(cursor)
            .filter(|it| wast::MODULE_FIELDS.contains(&it.kind_id()));
        work.extend(fields);

        Ok(())
    }

    pub(super) fn register<'a>(
        _cursor: &mut tree_sitter::TreeCursor<'a>,
        _work: &mut VecDeque<tree_sitter::Node<'a>>,
        this: tree_sitter::Node<'a>,
        builder: &mut SemanticTokensBuilder<'a>,
    ) -> anyhow::Result<()> {
        if let Some(node) = this.child(1) {
            let range = crate::util::node::range(&node);
            builder.push(range, &SemanticTokenType::KEYWORD, None)?;
        }

        Ok(())
    }

    pub(super) fn root<'a>(
        cursor: &mut tree_sitter::TreeCursor<'a>,
        work: &mut VecDeque<tree_sitter::Node<'a>>,
        this: tree_sitter::Node<'a>,
        _builder: &mut SemanticTokensBuilder<'a>,
    ) -> anyhow::Result<()> {
        cursor.reset(this);
        let commands = this.named_children(cursor).filter(|it| {
            [&[*wast::kind::COMMAND], wast::MODULE_FIELDS.as_slice()]
                .concat()
                .contains(&it.kind_id())
        });
        work.extend(commands);

        Ok(())
    }

    pub(super) fn script_module_binary<'a>(
        _cursor: &mut tree_sitter::TreeCursor<'a>,
        _work: &mut VecDeque<tree_sitter::Node<'a>>,
        this: tree_sitter::Node<'a>,
        builder: &mut SemanticTokensBuilder<'a>,
    ) -> anyhow::Result<()> {
        if let Some(node) = this.child(1) {
            let range = crate::util::node::range(&node);
            builder.push(range, &SemanticTokenType::KEYWORD, None)?;
        }

        Ok(())
    }

    pub(super) fn script_module_quote<'a>(
        _cursor: &mut tree_sitter::TreeCursor<'a>,
        _work: &mut VecDeque<tree_sitter::Node<'a>>,
        this: tree_sitter::Node<'a>,
        builder: &mut SemanticTokensBuilder<'a>,
    ) -> anyhow::Result<()> {
        if let Some(node) = this.child(1) {
            let range = crate::util::node::range(&node);
            builder.push(range, &SemanticTokenType::KEYWORD, None)?;
        }

        Ok(())
    }
}
