use crate::{core::language::Language, provider::semantic_tokens::builder::SemanticTokensBuilder};
use lspower::lsp_types::*;

/// The current node stack along with it's hash. Used for context comparison.
#[derive(Debug, Default, Clone)]
pub(super) struct NodeWalkerStack<'a> {
    nodes: Vec<tree_sitter::Node<'a>>,
    hash: u64,
}

impl<'a> NodeWalkerStack<'a> {
    fn new() -> Self {
        let mut stack = Self { ..Default::default() };
        stack.rehash();
        stack
    }

    fn pop(&mut self) -> Option<tree_sitter::Node<'a>> {
        let node = self.nodes.pop();
        self.rehash();
        node
    }

    fn push(&mut self, node: tree_sitter::Node<'a>) {
        self.nodes.push(node);
        self.rehash();
    }

    fn rehash(&mut self) {
        use std::{
            collections::hash_map::DefaultHasher,
            hash::{Hash, Hasher},
        };
        let mut hasher = DefaultHasher::new();
        Hash::hash_slice(&self.nodes, &mut hasher);
        self.hash = hasher.finish();
    }
}

// The current state of the node walking and token encoding algorithm.
pub(super) struct NodeWalker<'a> {
    language: Language,
    stack: NodeWalkerStack<'a>,
    builder: SemanticTokensBuilder<'a>,
    cursor: tree_sitter::TreeCursor<'a>,
    done: bool,
}

impl<'a> NodeWalker<'a> {
    fn new(language: Language, legend: Option<&'a SemanticTokensLegend>, node: &tree_sitter::Node<'a>) -> Self {
        let stack = NodeWalkerStack::new();
        let builder = SemanticTokensBuilder::new(legend);
        let cursor = node.walk();
        let done = false;
        let mut walker = Self {
            language,
            stack,
            builder,
            cursor,
            done,
        };
        walker.reconstruct_stack();
        walker
    }

    // Move to the next appropriate node in the syntax tree.
    fn goto_next(&mut self) {
        let prev = self.cursor.node();

        // First try to descend to the first child node.
        if self.cursor.goto_first_child() {
            self.stack.push(prev);
        } else {
            // Otherwise try to move to the next sibling node.
            if !self.cursor.goto_next_sibling() {
                let mut finished = true;
                // Otherwise continue to ascend to parent nodes...
                while self.cursor.goto_parent() {
                    self.stack.pop();
                    // ... until we can move to a sibling node.
                    if self.cursor.goto_next_sibling() {
                        finished = false;
                        break;
                    }
                    // Otherwise we set `done = true` and stop the outer loop.
                }
                self.done = finished;
            }
        }
    }

    // Reconstruct the context stack from the current node position.
    fn reconstruct_stack(&mut self) {
        use crate::core::language::{wast, wat};
        use Language::{Wast, Wat};

        let language = self.language;
        let node = self.cursor.node();
        let kind = node.kind_id();

        if (language == Wast && !wast::kind::equals::ROOT(kind)) || (language == Wat && !wat::kind::equals::ROOT(kind))
        {
            while self.cursor.goto_parent() {
                self.stack.push(self.cursor.node());
            }
            self.stack.nodes.reverse();
            self.cursor.reset(node);
        }
    }
}
