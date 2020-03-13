use crate::error::Error;
use failure::Fallible;
use tree_sitter::Parser;

pub struct Factory;

impl Factory {
    pub fn parser() -> Fallible<Parser> {
        let language = unsafe { crate::tree_sitter_webassembly() };
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(language).map_err(Error::LanguageError)?;
        Ok(parser)
    }
}
