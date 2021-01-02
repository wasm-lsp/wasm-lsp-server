use crate::core::language::Language;
// use std::slice::SliceIndex;

/// The current node stack along with it's hash. Used for context comparison.
#[derive(Debug, Default, Clone)]
pub(super) struct NodeWalkerStack<'a> {
    nodes: Vec<tree_sitter::Node<'a>>,
}

impl<'a> NodeWalkerStack<'a> {
    fn new() -> Self {
        Self { ..Default::default() }
    }

    // fn matches<I>(&self, index: I, kinds: &[u16]) -> bool
    // where
    //     I: SliceIndex<[tree_sitter::Node<'a>], Output = [tree_sitter::Node<'a>]>,
    // {
    //     if let Some(nodes) = self.nodes.get(index) {
    //         if nodes.len() == kinds.len() {
    //             let mut result = false;
    //             for i in 0 .. kinds.len() {
    //                 result = kinds[i] == nodes[i].kind_id();
    //             }
    //             result
    //         } else {
    //             false
    //         }
    //     } else {
    //         false
    //     }
    // }

    fn pop(&mut self) -> Option<tree_sitter::Node<'a>> {
        self.nodes.pop()
    }

    fn push(&mut self, node: tree_sitter::Node<'a>) {
        self.nodes.push(node);
    }
}

// The current state of the node walking and token encoding algorithm.
pub(super) struct NodeWalker<'a> {
    language: Language,
    stack: NodeWalkerStack<'a>,
    cursor: tree_sitter::TreeCursor<'a>,
    pub(super) done: bool,
}

impl<'a> NodeWalker<'a> {
    pub(super) fn new(language: Language, node: tree_sitter::Node<'a>) -> Self {
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

    // pub(super) fn context<I>(&self, index: I, kinds: &[u16]) -> bool
    // where
    //     I: SliceIndex<[tree_sitter::Node<'a>], Output = [tree_sitter::Node<'a>]>,
    // {
    //     self.stack.matches(index, kinds)
    // }

    // pub(super) fn depth(&self) -> usize {
    //     self.stack.nodes.len()
    // }

    // Move the cursor to the first child node.
    pub(super) fn goto_first_child(&mut self) -> bool {
        let prev = self.cursor.node();
        let moved = self.cursor.goto_first_child();
        if moved {
            self.stack.push(prev);
        }
        moved
    }

    // Move the cursor to the next sibling node.
    pub(super) fn goto_next_sibling(&mut self) -> bool {
        self.cursor.goto_next_sibling()
    }

    // Move cursor to the next accessible node.
    pub(super) fn goto_next(&mut self) -> bool {
        let prev = self.cursor.node();
        let mut moved;

        // First try to descend to the first child node.
        moved = self.cursor.goto_first_child();
        if moved {
            self.stack.push(prev);
        } else {
            // Otherwise try to move to the next sibling node.
            moved = self.cursor.goto_next_sibling();
            if !moved {
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
            }
        }
        moved
    }

    // Move the cursor to the parent node.
    pub(super) fn goto_parent(&mut self) -> bool {
        let moved = self.cursor.goto_parent();
        if moved {
            self.stack.pop();
        }
        moved
    }

    // Return the current node's kind id.
    pub(super) fn kind(&self) -> u16 {
        self.cursor.node().kind_id()
    }

    // Return the current node for the cursor.
    pub(super) fn node(&self) -> tree_sitter::Node<'a> {
        self.cursor.node()
    }

    // Reconstruct the context stack from the current node position.
    fn reconstruct_stack(&mut self) {
        use crate::core::language::{wast, wat};
        use Language::{Wast, Wat};

        let language = self.language;
        let node = self.cursor.node();
        let kind = node.kind_id();

        // Reconstruct the stack by traversing upward if the current node isn't ROOT.
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
