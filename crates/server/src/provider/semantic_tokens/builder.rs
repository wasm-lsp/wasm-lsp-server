//! Definitions for the semantic tokens encoder.

use anyhow::anyhow;
use std::collections::HashMap;

/// The builder for the semantic tokens encoder. Encapsulates state during encoding.
#[derive(Clone, Debug, Default)]
pub(crate) struct SemanticTokensBuilder<'a> {
    prev_start: u32,
    prev_line: u32,
    data_is_sorted_and_delta_encoded: bool,
    data: Vec<lsp::SemanticToken>,
    token_modifier_map: HashMap<&'a lsp::SemanticTokenModifier, u32>,
    token_type_map: HashMap<&'a lsp::SemanticTokenType, u32>,
    has_legend: bool,
}

impl<'a> SemanticTokensBuilder<'a> {
    /// Construct a new builder given an optional tokens legend.
    pub(crate) fn new(legend: Option<&'a lsp::SemanticTokensLegend>) -> Self {
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

    /// Build and return the semantic tokenization result from the current encoder state.
    pub(crate) fn build(self) -> lsp::SemanticTokens {
        let data = if !self.data_is_sorted_and_delta_encoded {
            Self::sort_and_delta_encode(&self.data)
        } else {
            self.data
        };

        lsp::SemanticTokens {
            data,
            ..Default::default()
        }
    }

    /// Push a new semantic token onto the encoder state.
    pub(crate) fn push(
        &mut self,
        range: lsp::Range,
        token_type: &lsp::SemanticTokenType,
        token_modifiers: Option<Vec<&lsp::SemanticTokenModifier>>,
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

            self.push_encoded(line, char, length, n_token_type, n_token_modifiers);
        } else {
            return Err(anyhow!("`token_type` is not in the provided legend"));
        }

        Ok(())
    }

    /// Push a new semantic token (in encoded form) onto the encoder state.
    fn push_encoded(&mut self, line: u32, char: u32, length: u32, token_type: u32, token_modifiers_bitset: u32) {
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

        self.data.push(lsp::SemanticToken {
            delta_line,
            delta_start,
            length,
            token_type,
            token_modifiers_bitset,
        });

        self.prev_line = line;
        self.prev_start = char;
    }

    /// Sort and delta-encode a slice of semantic tokens.
    pub(crate) fn sort_and_delta_encode(data: &[lsp::SemanticToken]) -> Vec<lsp::SemanticToken> {
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

            result.push(lsp::SemanticToken {
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
