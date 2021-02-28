use crate::language::{wast, wat, Language};
// use std::slice::SliceIndex;

///
#[derive(Debug, Clone)]
pub struct NodeWalkerLevel<'tree> {
    ancestor: tree_sitter::Node<'tree>,
    prefixed: Vec<tree_sitter::Node<'tree>>,
}

/// The current node stack. Used for context comparison.
#[derive(Debug, Default, Clone)]
pub struct NodeWalkerContext<'tree> {
    stack: Vec<NodeWalkerLevel<'tree>>,
}

impl<'tree> NodeWalkerContext<'tree> {
    fn new() -> Self {
        Self { ..Default::default() }
    }

    // fn matches<I>(&self, index: I, kind_ids: &[u16]) -> bool
    // where
    //     I: SliceIndex<[u16], Output = [u16]>,
    // {
    //     let that = kind_ids;
    //     if let Some(this) = self.kinds.get(index) {
    //         this == that
    //     } else {
    //         false
    //     }
    // }

    #[inline]
    fn pop(&mut self) -> Option<NodeWalkerLevel<'tree>> {
        self.stack.pop()
    }

    // #[inline]
    // fn pop_level(&mut self) -> Option<tree_sitter::Node<'tree>> {
    //     self.stack.pop().map(|level| level.ancestor)
    // }

    // #[inline]
    // fn pop_prefix(&mut self) -> Option<tree_sitter::Node<'tree>> {
    //     if let Some(level) = self.stack.last_mut() {
    //         level.prefixed.pop()
    //     } else {
    //         None
    //     }
    // }

    #[inline]
    fn push(&mut self, level: NodeWalkerLevel<'tree>) {
        self.stack.push(level);
    }

    #[inline]
    fn push_ancestor(&mut self, ancestor: tree_sitter::Node<'tree>) {
        let prefixed = Default::default();
        let level = NodeWalkerLevel { ancestor, prefixed };
        self.stack.push(level);
    }

    #[inline]
    fn push_prefix(&mut self, prefix: tree_sitter::Node<'tree>) {
        if let Some(level) = self.stack.last_mut() {
            level.prefixed.push(prefix);
        } else {
            unreachable!("NodeWalkerContext::push_prefix should never be callable wihout an active level");
        }
    }
}

/// The current state of the node walking and token encoding algorithm.
pub struct NodeWalker<'tree> {
    language: Language,
    context: NodeWalkerContext<'tree>,
    cursor: tree_sitter::TreeCursor<'tree>,
    /// Whether the [`NodeWalker`] has finished traversing the origin [`tree_sitter::Tree`].
    pub done: bool,
}

impl<'tree> NodeWalker<'tree> {
    /// Create a new [NodeWalker].
    #[inline]
    pub fn new(language: Language, node: tree_sitter::Node<'tree>) -> Self {
        let context = NodeWalkerContext::new();
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

    // /// Determine whether a given a slice of [`tree_sitter::Node`] kind ids forms the context.
    // pub fn context_matches<I>(&self, index: I, kind_ids: &[u16]) -> bool
    // where
    //     I: SliceIndex<[u16], Output = [u16]>,
    // {
    //     self.stack.matches(index, kind_ids)
    // }

    // /// Return the slice of [`tree_sitter::Node`] kind ids that form the context.
    // pub fn context_kinds(&self) -> &[u16] {
    //     &self.stack.kinds
    // }

    // /// Return the slice of [`tree_sitter::Node`] that form the context.
    // pub fn context_nodes(&self) -> &[tree_sitter::Node] {
    //     &self.stack.nodes
    // }

    /// Return the depth of the current context.
    #[inline]
    pub fn depth(&self) -> usize {
        self.context.stack.len()
    }

    /// Move the cursor to the first child node.
    #[inline]
    pub fn goto_first_child(&mut self) -> bool {
        let ancestor = self.cursor.node();
        let moved = self.cursor.goto_first_child();
        if moved {
            self.context.push_ancestor(ancestor);
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
        if node.has_error()
            && ![
                wast::kind::COMMENT_BLOCK_ANNOT,
                wast::kind::COMMENT_BLOCK,
                wast::kind::COMMENT_LINE_ANNOT,
                wast::kind::COMMENT_LINE,
                wat::kind::COMMENT_BLOCK_ANNOT,
                wat::kind::COMMENT_BLOCK,
                wat::kind::COMMENT_LINE_ANNOT,
                wat::kind::COMMENT_LINE,
            ]
            .contains(&node.kind_id())
        {
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
                    let level = NodeWalkerLevel { ancestor, prefixed };
                    self.context.push(level);
                } else {
                    break;
                }
            }

            self.context.stack.reverse();
            self.cursor.reset(node);
        }
    }
}
