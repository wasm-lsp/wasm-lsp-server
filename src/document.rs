use tokio::sync::Mutex;

pub struct Document {
    pub text: String,
    pub tree: Mutex<tree_sitter::Tree>,
}
