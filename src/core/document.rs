use crate::core::language::Language;
use tokio::sync::Mutex;
use tree_sitter::{Parser, Tree};

pub(crate) struct Document {
    pub(crate) language: Language,
    pub(crate) parser: Mutex<Parser>,
    pub(crate) text: String,
    pub(crate) tree: Mutex<Tree>,
}
