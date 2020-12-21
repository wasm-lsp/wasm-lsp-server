//! Semantic tokens provider definitions for ".wast" files.

pub(crate) mod full {
    use crate::core::{document::Document, session::Session};
    use lspower::lsp_types::*;
    use std::sync::Arc;

    pub(crate) async fn response(
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

        let result = super::range::response(session, document, params)
            .await?
            .map(|result| match result {
                SemanticTokensRangeResult::Tokens(tokens) => SemanticTokensResult::Tokens(tokens),
                SemanticTokensRangeResult::Partial(partial) => SemanticTokensResult::Partial(partial),
            });

        Ok(result)
    }
}

pub(crate) mod range {
    use crate::{
        core::{document::Document, language::wat, session::Session},
        provider::semantic_tokens::SemanticTokensBuilder,
    };
    use anyhow::anyhow;
    use lspower::lsp_types::*;
    use std::{collections::VecDeque, sync::Arc};

    pub(crate) async fn response(
        session: Arc<Session>,
        document: &Document,
        params: SemanticTokensRangeParams,
    ) -> anyhow::Result<Option<SemanticTokensRangeResult>> {
        let legend = session.semantic_tokens_legend().await;

        let module_fields: &[u16] = &[
            *wat::kind::MODULE_FIELD_DATA,
            *wat::kind::MODULE_FIELD_ELEM,
            *wat::kind::MODULE_FIELD_FUNC,
            *wat::kind::MODULE_FIELD_GLOBAL,
            *wat::kind::MODULE_FIELD_MEMORY,
            *wat::kind::MODULE_FIELD_TABLE,
            *wat::kind::MODULE_FIELD_TYPE,
        ];

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
                if *wat::kind::ROOT == this.kind_id() {
                    cursor.reset(this);
                    let commands = this
                        .named_children(&mut cursor)
                        .filter(|it| [&[*wat::kind::MODULE], module_fields].concat().contains(&it.kind_id()));
                    work.extend(commands);
                    continue;
                }

                if *wat::kind::MODULE == this.kind_id() {
                    if let Some(node) = this.child(1) {
                        let range = crate::util::node::range(&node);
                        builder.push(range, &SemanticTokenType::KEYWORD, None)?;
                    }

                    cursor.reset(this);
                    let fields = this
                        .named_children(&mut cursor)
                        .filter(|it| module_fields.contains(&it.kind_id()));
                    work.extend(fields);
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
}
