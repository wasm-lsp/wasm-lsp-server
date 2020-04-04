use failure::Fail;
use tree_sitter;

#[derive(Debug, Fail)]
pub(crate) enum Error {
    #[fail(display = "tree_sitter::LanguageError: {}", 0)]
    LanguageError(tree_sitter::LanguageError),
    #[fail(display = "tree_sitter::QueryError")]
    QueryError(tree_sitter::QueryError),
}
