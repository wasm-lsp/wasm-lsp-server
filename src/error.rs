use failure::Fail;
use tree_sitter;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "tree_sitter :: language error :: {}", 0)]
    LanguageError(tree_sitter::LanguageError),
}
