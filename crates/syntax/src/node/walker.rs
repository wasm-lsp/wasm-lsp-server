use crate::language::Language;
use std::slice::SliceIndex;

/// The current node stack. Used for context comparison.
#[derive(Debug, Default, Clone)]
pub struct NodeWalkerStack<'tree> {
    nodes: Vec<tree_sitter::Node<'tree>>,
}

impl<'tree> NodeWalkerStack<'tree> {
    fn new() -> Self {
        Self { ..Default::default() }
    }

    fn matches<I>(&self, index: I, kind_ids: &[u16]) -> bool
    where
        I: SliceIndex<[tree_sitter::Node<'tree>], Output = [tree_sitter::Node<'tree>]>,
    {
        if let Some(nodes) = self.nodes.get(index) {
            if nodes.len() == kind_ids.len() {
                let mut result = false;
                for i in 0 .. kind_ids.len() {
                    result = kind_ids[i] == nodes[i].kind_id();
                }
                result
            } else {
                false
            }
        } else {
            false
        }
    }

    fn pop(&mut self) -> Option<tree_sitter::Node<'tree>> {
        self.nodes.pop()
    }

    fn push(&mut self, node: tree_sitter::Node<'tree>) {
        self.nodes.push(node);
    }
}

/// The current state of the node walking and token encoding algorithm.
pub struct NodeWalker<'tree> {
    language: Language,
    stack: NodeWalkerStack<'tree>,
    cursor: tree_sitter::TreeCursor<'tree>,
    /// Whether the [`NodeWalker`] has finished traversing the origin [`tree_sitter::Tree`].
    pub done: bool,
}

impl<'tree> NodeWalker<'tree> {
    /// Create a new [NodeWalker].
    pub fn new(language: Language, node: tree_sitter::Node<'tree>) -> Self {
        let stack = NodeWalkerStack::new();
        let cursor = node.walk();
        let done = false;
        let mut walker = Self {
            language,
            stack,
            cursor,
            done,
        };
        walker.reconstruct_stack();
        walker
    }

    /// Determine whether a given a slice of [`tree_sitter::Node`] kind ids forms the context.
    pub fn context<I>(&self, index: I, kind_ids: &[u16]) -> bool
    where
        I: SliceIndex<[tree_sitter::Node<'tree>], Output = [tree_sitter::Node<'tree>]>,
    {
        self.stack.matches(index, kind_ids)
    }

    /// Return the depth of the current context.
    pub fn depth(&self) -> usize {
        self.stack.nodes.len()
    }

    /// Move the cursor to the first child node.
    pub fn goto_first_child(&mut self) -> bool {
        let node = self.cursor.node();
        let moved = self.cursor.goto_first_child();
        if moved {
            self.stack.push(node);
        }
        moved
    }

    /// Move the cursor to the next sibling node.
    pub fn goto_next_sibling(&mut self) -> bool {
        self.cursor.goto_next_sibling()
    }

    /// Move cursor to the next accessible node.
    pub fn goto_next(&mut self) -> bool {
        let node = self.cursor.node();
        let mut moved;

        // First try to descend to the first child node.
        moved = self.cursor.goto_first_child();
        if moved {
            self.stack.push(node);
        } else {
            // Otherwise try to move to the next sibling node.
            moved = self.cursor.goto_next_sibling();
            if !moved {
                moved = self.goto_next_ancestor_sibling();
            }
        }

        moved
    }

    /// Move cursor to the next accessible node that has an error.
    pub fn goto_next_has_error(&mut self) -> bool {
        let node = self.cursor.node();
        let mut moved;

        // Only descend if the current node has an error in the subtree.
        if node.has_error() {
            moved = self.goto_next();
        } else {
            // Otherwise try to move to the next sibling node.
            moved = self.cursor.goto_next_sibling();
            if !moved {
                moved = self.goto_next_ancestor_sibling();
            }
        }

        moved
    }

    /// Move the cursor to the next ancestor sibling node.
    pub fn goto_next_ancestor_sibling(&mut self) -> bool {
        let mut moved = false;
        let mut finished = true;

        // Otherwise continue to ascend to parent nodes...
        while self.cursor.goto_parent() {
            moved = true;
            self.stack.pop();
            // ... until we can move to a sibling node.
            if self.cursor.goto_next_sibling() {
                finished = false;
                break;
            }
            // Otherwise we set `done = true` and stop the outer loop.
        }

        self.done = finished;
        moved
    }

    /// Move the cursor to the parent node.
    pub fn goto_parent(&mut self) -> bool {
        let moved = self.cursor.goto_parent();
        if moved {
            self.stack.pop();
        }
        moved
    }

    /// Return the current node's kind id.
    pub fn kind(&self) -> u16 {
        self.cursor.node().kind_id()
    }

    /// Return the current node for the cursor.
    pub fn node(&self) -> tree_sitter::Node<'tree> {
        self.cursor.node()
    }

    /// Reconstruct the context stack from the current node position.
    fn reconstruct_stack(&mut self) {
        use crate::language::{wast, wat};
        use Language::{Wast, Wat};

        let language = self.language;
        let node = self.cursor.node();
        let kind = node.kind_id();

        // Reconstruct the stack by traversing upward if the current node isn't ROOT.
        if (language == Wast && *wast::kind::ROOT != kind) || (language == Wat && *wat::kind::ROOT != kind) {
            while self.cursor.goto_parent() {
                self.stack.push(self.cursor.node());
            }
            self.stack.nodes.reverse();
            self.cursor.reset(node);
        }
    }
}
