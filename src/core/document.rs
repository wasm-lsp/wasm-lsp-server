use tokio::sync::Mutex;

pub(crate) struct Document {
    pub(crate) text: String,
    pub(crate) tree: Mutex<tree_sitter::Tree>,
}
