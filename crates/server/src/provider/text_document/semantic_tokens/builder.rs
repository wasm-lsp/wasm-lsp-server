//! Definitions for the semantic tokens builder used during tokenization.

use anyhow::anyhow;
use lsp::SemanticTokensFullDeltaResult;
use lsp_text::RopeExt;
use std::{collections::HashMap, convert::TryFrom};

/// Manages tokenization state for encoding semantic token data.
#[derive(Clone, Debug)]
pub struct SemanticTokensBuilder<'text, 'tree> {
    content: &'text ropey::Rope,
    id: u128,
    prev_row: u32,
    prev_col: u32,
    prev_data: Option<Vec<lsp::SemanticToken>>,
    data: Vec<lsp::SemanticToken>,
    token_modifier_map: HashMap<&'tree lsp::SemanticTokenModifier, u32>,
    token_type_map: HashMap<&'tree lsp::SemanticTokenType, u32>,
    has_legend: bool,
}

impl<'text, 'tree> SemanticTokensBuilder<'text, 'tree> {
    /// Create a new [`SemanticTokensBuilder`].
    pub fn new(content: &'text ropey::Rope, legend: Option<&'tree lsp::SemanticTokensLegend>) -> anyhow::Result<Self> {
        use std::time::{SystemTime, UNIX_EPOCH};

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

        Ok(Self {
            content,
            id: SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis(),
            prev_row: Default::default(),
            prev_col: Default::default(),
            prev_data: Default::default(),
            data: Default::default(),
            token_modifier_map,
            token_type_map,
            has_legend,
        })
    }

    /// Build the [`lsp::SemanticTokens`] data from the tokenization state.
    pub fn build(&mut self) -> lsp::SemanticTokens {
        self.prev_data = None;
        lsp::SemanticTokens {
            result_id: Some(self.id()),
            data: self.data.clone(),
        }
    }

    /// Build the [`lsp::SemanticTokensFullDeltaResult`] data from the current tokenization state.
    pub fn build_delta(&mut self) -> anyhow::Result<lsp::SemanticTokensFullDeltaResult> {
        if let Some(prev_data) = &self.prev_data {
            let mut start_idx = 0;
            while start_idx < self.data.len()
                && start_idx < prev_data.len()
                && prev_data[start_idx] == self.data[start_idx]
            {
                start_idx += 1;
            }

            if start_idx < self.data.len() && start_idx < prev_data.len() {
                let mut end_idx = 0;
                while end_idx < self.data.len()
                    && end_idx < prev_data.len()
                    && prev_data[prev_data.len() - 1 - end_idx] == self.data[self.data.len() - 1 - end_idx]
                {
                    end_idx += 1;
                }

                let edit = {
                    let start = u32::try_from(start_idx)?;
                    let delete_count = u32::try_from(prev_data.len() - end_idx - start_idx)?;
                    let data = Some(self.data[start_idx .. self.data.len() - end_idx].to_vec());
                    lsp::SemanticTokensEdit {
                        start,
                        delete_count,
                        data,
                    }
                };

                let tokens_delta = lsp::SemanticTokensDelta {
                    result_id: Some(self.id()),
                    edits: vec![edit],
                };

                Ok(lsp::SemanticTokensFullDeltaResult::TokensDelta(tokens_delta))
            } else if start_idx < self.data.len() {
                let edit = {
                    let start = u32::try_from(start_idx)?;
                    let delete_count = 0;
                    let data = Some(self.data[start_idx ..].to_vec());
                    lsp::SemanticTokensEdit {
                        start,
                        delete_count,
                        data,
                    }
                };

                let tokens_delta = lsp::SemanticTokensDelta {
                    result_id: Some(self.id()),
                    edits: vec![edit],
                };

                Ok(lsp::SemanticTokensFullDeltaResult::TokensDelta(tokens_delta))
            } else if start_idx < prev_data.len() {
                let edit = {
                    let start = u32::try_from(start_idx)?;
                    let delete_count = u32::try_from(prev_data.len() - start_idx)?;
                    let data = None;
                    lsp::SemanticTokensEdit {
                        start,
                        delete_count,
                        data,
                    }
                };

                let tokens_delta = lsp::SemanticTokensDelta {
                    result_id: Some(self.id()),
                    edits: vec![edit],
                };

                Ok(lsp::SemanticTokensFullDeltaResult::TokensDelta(tokens_delta))
            } else {
                let tokens_delta = lsp::SemanticTokensDelta {
                    result_id: Some(self.id()),
                    edits: vec![],
                };

                Ok(lsp::SemanticTokensFullDeltaResult::TokensDelta(tokens_delta))
            }
        } else {
            self.prev_data = None;
            let semantic_tokens = lsp::SemanticTokens {
                result_id: Some(self.id()),
                data: self.data.clone(),
            };
            Ok(SemanticTokensFullDeltaResult::Tokens(semantic_tokens))
        }
    }

    /// Return the ID for the current tokenization state.
    pub fn id(&self) -> String {
        self.id.to_string()
    }

    /// Rollback tokenization state to previous data.
    pub fn prev_result(&mut self, id: &str) -> anyhow::Result<()> {
        if self.id() == id {
            self.prev_data = Some(self.data.clone());
        }
        self.reset()
    }

    /// Push and encode a token into the tokenization state.
    pub fn push(
        &mut self,
        node: tree_sitter::Node,
        token_type: &lsp::SemanticTokenType,
        token_modifiers: Option<Vec<&lsp::SemanticTokenModifier>>,
    ) -> anyhow::Result<()> {
        if !self.has_legend {
            return Err(anyhow!("Legend must be provided in constructor"));
        }

        if node.has_error() {
            return Ok(());
        }

        let range = self.content.tree_sitter_range_to_lsp_range(node.range());

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

            self.push_encoded(line, char, length, n_token_type, n_token_modifiers);
        } else {
            return Err(anyhow!("`token_type` is not in the provided legend"));
        }

        Ok(())
    }

    /// Push a token in encoded form into the tokenization state.
    pub fn push_encoded(&mut self, row: u32, col: u32, len: u32, token_type: u32, token_mods: u32) {
        let mut push_row = row;
        let mut push_col = col;

        if !self.data.is_empty() {
            push_row -= self.prev_row;
            if push_row == 0 {
                push_col -= self.prev_col;
            }
        }

        let semantic_token = lsp::SemanticToken {
            delta_line: push_row,
            delta_start: push_col,
            length: len,
            token_type,
            token_modifiers_bitset: token_mods,
        };

        self.data.push(semantic_token);

        self.prev_row = row;
        self.prev_col = col;
    }

    /// Reset tokenization state to defaults.
    pub fn reset(&mut self) -> anyhow::Result<()> {
        use std::time::{SystemTime, UNIX_EPOCH};
        self.id = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();
        self.prev_row = Default::default();
        self.prev_col = Default::default();
        self.prev_data = Default::default();
        self.data = Default::default();
        Ok(())
    }
}
