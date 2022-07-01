use wasm_lsp_languages::language::Language;

#[allow(missing_docs)]
pub mod context {
    use tree_sitter::Node;

    pub trait Context<'tree> {
        type Level;

        fn new() -> Self;

        fn pop(&mut self) -> Option<Self::Level>;

        fn push(&mut self, level: Self::Level);

        fn push_ancestor(&mut self, ancestor: Node<'tree>, prefixed: Vec<Node<'tree>>);

        fn push_prefix(&mut self, prefix: Node<'tree>);

        fn reverse(&mut self);
    }

    pub mod basic {
        use std::convert::Infallible;
        use tree_sitter::Node;

        #[derive(Debug, Clone, Eq, Hash, PartialEq)]
        pub struct Level<'tree> {
            phantom: core::marker::PhantomData<&'tree Infallible>,
        }

        #[derive(Debug, Default, Clone, Eq, Hash, PartialEq)]
        pub struct Context<'tree> {
            phantom: core::marker::PhantomData<&'tree Infallible>,
        }

        impl<'tree> super::Context<'tree> for Context<'tree> {
            type Level = Level<'tree>;

            fn new() -> Self {
                Self::default()
            }

            fn pop(&mut self) -> Option<Self::Level> {
                None
            }

            fn push(&mut self, _: Self::Level) {
            }

            fn push_ancestor(&mut self, _: Node<'tree>, _: Vec<Node<'tree>>) {
            }

            fn push_prefix(&mut self, _: Node<'tree>) {
            }

            fn reverse(&mut self) {
            }
        }
    }

    pub mod trace {
        use tree_sitter::Node;

        #[derive(Debug, Clone, Eq, Hash, PartialEq)]
        pub struct Level<'tree> {
            ancestor: Node<'tree>,
            prefixed: Vec<Node<'tree>>,
        }

        /// The current node context.
        #[derive(Debug, Default, Clone, Eq, Hash, PartialEq)]
        pub struct Context<'tree> {
            stack: Vec<Level<'tree>>,
        }

        impl<'tree> super::Context<'tree> for Context<'tree> {
            type Level = Level<'tree>;

            fn new() -> Self {
                Self::default()
            }

            fn pop(&mut self) -> Option<Self::Level> {
                self.stack.pop()
            }

            fn push(&mut self, level: Self::Level) {
                self.stack.push(level);
            }

            fn push_ancestor(&mut self, ancestor: Node<'tree>, prefixed: Vec<Node<'tree>>) {
                let level = Level { ancestor, prefixed };
                self.stack.push(level);
            }

            fn push_prefix(&mut self, prefix: Node<'tree>) {
                if let Some(level) = self.stack.last_mut() {
                    level.prefixed.push(prefix);
                } else {
                    unreachable!("NodeWalkerContext::push_prefix should never be callable wihout an active level");
                }
            }

            fn reverse(&mut self) {
                self.stack.reverse();
            }
        }
    }
}

pub use context::Context;

#[allow(missing_docs)]
pub struct NodeWalker<'tree, C> {
    language: Language,
    pub context: C,
    cursor: tree_sitter::TreeCursor<'tree>,
    pub done: bool,
}

impl<'tree, C: Context<'tree>> NodeWalker<'tree, C> {
    /// Create a new [NodeWalker].
    #[inline]
    pub fn new(language: Language, node: tree_sitter::Node<'tree>) -> Self {
        let context = C::new();
        let cursor = node.walk();
        let done = false;
        let mut walker = Self {
            language,
            context,
            cursor,
            done,
        };
        walker.reconstruct_stack();
        walker
    }

    /// Move the cursor to the first child node.
    #[inline]
    pub fn goto_first_child(&mut self) -> bool {
        let ancestor = self.cursor.node();
        let moved = self.cursor.goto_first_child();
        if moved {
            let prefixed = Default::default();
            self.context.push_ancestor(ancestor, prefixed);
        }
        moved
    }

    /// Move the cursor to the next sibling node.
    #[inline]
    pub fn goto_next_sibling(&mut self) -> bool {
        let prefix = self.cursor.node();
        let moved = self.cursor.goto_next_sibling();
        if moved {
            self.context.push_prefix(prefix);
        }
        moved
    }

    /// Move cursor to the next accessible node.
    #[inline]
    #[allow(clippy::needless_late_init)]
    pub fn goto_next(&mut self) -> bool {
        let mut moved;

        // First try to descend to the first child node.
        moved = self.goto_first_child();
        if !moved {
            // Otherwise try to move to the next sibling node.
            moved = self.goto_next_sibling();
            if !moved {
                moved = self.goto_next_ancestor_sibling();
            }
        }

        moved
    }

    /// Move cursor to the next accessible node that has an error.
    #[inline]
    pub fn goto_next_has_error(&mut self) -> bool {
        let node = self.cursor.node();
        let mut moved;

        // Only descend if the current node has an error in the subtree.
        if node.has_error() && !crate::language::COMMENT_NODES.contains(&node.kind_id()) {
            moved = self.goto_next();
        } else {
            // Otherwise try to move to the next sibling node.
            moved = self.goto_next_sibling();
            if !moved {
                moved = self.goto_next_ancestor_sibling();
            }
        }

        moved
    }

    /// Move the cursor to the next ancestor sibling node.
    #[inline]
    pub fn goto_next_ancestor_sibling(&mut self) -> bool {
        let mut moved;
        let mut finished = true;

        // Otherwise continue to ascend to parent nodes...
        loop {
            moved = self.goto_parent();
            if moved {
                // ... until we can move to a sibling node.
                if self.goto_next_sibling() {
                    finished = false;
                    break;
                }
            } else {
                break;
            }
        }

        self.done = finished;
        moved
    }

    /// Move the cursor to the parent node.
    #[inline]
    pub fn goto_parent(&mut self) -> bool {
        let moved = self.cursor.goto_parent();
        if moved {
            self.context.pop();
        }
        moved
    }

    /// Return the current node's kind id.
    #[inline]
    pub fn kind(&self) -> u16 {
        self.cursor.node().kind_id()
    }

    /// Return the current node for the cursor.
    #[inline]
    pub fn node(&self) -> tree_sitter::Node<'tree> {
        self.cursor.node()
    }

    /// Reconstruct the context stack from the current node position.
    #[inline]
    fn reconstruct_stack(&mut self) {
        use crate::language::{wast, wat};
        use Language::{Wast, Wat};

        let language = self.language;
        let node = self.node();
        let kind = node.kind_id();

        // Reconstruct the stack by traversing upward if the current node isn't ROOT.
        if (language == Wast && wast::kind::ROOT != kind) || (language == Wat && wat::kind::ROOT != kind) {
            let cursor = &mut node.walk();
            loop {
                let previous = self.node();
                if self.goto_parent() {
                    let ancestor = self.node();
                    let prefixed = ancestor
                        .children(cursor)
                        .take_while(|node| node.id() != previous.id())
                        .collect();
                    self.context.push_ancestor(ancestor, prefixed)
                } else {
                    break;
                }
            }

            self.context.reverse();
            self.cursor.reset(node);
        }
    }
}

#[allow(missing_docs)]
pub type BasicNodeWalker<'tree> = NodeWalker<'tree, context::basic::Context<'tree>>;

#[allow(missing_docs)]
pub type TraceNodeWalker<'tree> = NodeWalker<'tree, context::trace::Context<'tree>>;
