use failure::Fail;
use tree_sitter;

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Fail)]
pub(crate) enum Error {
    #[fail(display = "core::InvalidLanguageId: {}", 0)]
    CoreInvalidLanguageId(String),
    #[fail(display = "tree_sitter::LanguageError: {}", 0)]
    TreeSitterLanguageError(tree_sitter::LanguageError),
    #[fail(display = "tree_sitter::QueryError")]
    TreeSitterQueryError(tree_sitter::QueryError),
}
