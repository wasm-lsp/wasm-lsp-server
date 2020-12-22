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
            if wast::kind::equals::COMMAND(this.kind_id()) {
                handle::command(&mut cursor, &mut work, this, &mut builder)?;
                continue;
            }

            if wast::kind::equals::MODULE(this.kind_id()) {
                handle::module(&mut cursor, &mut work, this, &mut builder)?;
                continue;
            }

            if wast::kind::equals::ROOT(this.kind_id()) {
                handle::root(&mut cursor, &mut work, this, &mut builder)?;
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
}
