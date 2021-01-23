use crate::core::language::{self, Language};
use std::convert::TryFrom;

pub fn wast() -> anyhow::Result<tree_sitter::Parser> {
    let language = language::wast::language();
    let mut parser = tree_sitter::Parser::new()?;
    parser.set_language(&language)?;
    Ok(parser)
}

pub fn wat() -> anyhow::Result<tree_sitter::Parser> {
    let language = language::wat::language();
    let mut parser = tree_sitter::Parser::new()?;
    parser.set_language(&language)?;
    Ok(parser)
}

impl TryFrom<Language> for tree_sitter::Parser {
    type Error = anyhow::Error;

    fn try_from(language: Language) -> anyhow::Result<tree_sitter::Parser> {
        match language {
            Language::Wast => wast(),
            Language::Wat => wat(),
        }
    }
}
