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
    use super::SemanticTokensBuilder;
    use crate::core::{
        document::Document,
        language::{wast, wat, Language},
        session::Session,
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

        let wast_module_fields: &[u16] = &[
            *wast::kind::MODULE_FIELD_DATA,
            *wast::kind::MODULE_FIELD_ELEM,
            *wast::kind::MODULE_FIELD_FUNC,
            *wast::kind::MODULE_FIELD_GLOBAL,
            *wast::kind::MODULE_FIELD_MEMORY,
            *wast::kind::MODULE_FIELD_TABLE,
            *wast::kind::MODULE_FIELD_TYPE,
        ];

        let wat_module_fields: &[u16] = &[
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
                match document.language {
                    Language::Wast => {
                        if *wast::kind::ROOT == this.kind_id() {
                            cursor.reset(this);
                            let commands = this.named_children(&mut cursor).filter(|it| {
                                [&[*wast::kind::COMMAND], wast_module_fields]
                                    .concat()
                                    .contains(&it.kind_id())
                            });
                            work.extend(commands);
                            continue;
                        }

                        if *wast::kind::MODULE == this.kind_id() {
                            if let Some(node) = this.child(1) {
                                let range = crate::util::node::range(&node);
                                builder.push(range, &SemanticTokenType::KEYWORD, None)?;
                            }

                            cursor.reset(this);
                            let fields = this
                                .named_children(&mut cursor)
                                .filter(|it| wast_module_fields.contains(&it.kind_id()));
                            work.extend(fields);
                            continue;
                        }
                    },
                    Language::Wat => {
                        if *wat::kind::ROOT == this.kind_id() {
                            cursor.reset(this);
                            let commands = this.named_children(&mut cursor).filter(|it| {
                                [&[*wat::kind::MODULE], wat_module_fields]
                                    .concat()
                                    .contains(&it.kind_id())
                            });
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
                                .filter(|it| wat_module_fields.contains(&it.kind_id()));
                            work.extend(fields);
                            continue;
                        }
                    },
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
