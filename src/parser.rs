use crate::error::Error;
use failure::Fallible;
use tree_sitter;

pub fn wast() -> Fallible<tree_sitter::Parser> {
    let language = unsafe { crate::tree_sitter_wast() };
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(language).map_err(Error::LanguageError)?;
    Ok(parser)
}

pub fn wat() -> Fallible<tree_sitter::Parser> {
    let language = unsafe { crate::tree_sitter_wat() };
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(language).map_err(Error::LanguageError)?;
    Ok(parser)
}

pub fn wit() -> Fallible<tree_sitter::Parser> {
    let language = unsafe { crate::tree_sitter_wit() };
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(language).map_err(Error::LanguageError)?;
    Ok(parser)
}

pub fn witx() -> Fallible<tree_sitter::Parser> {
    let language = unsafe { crate::tree_sitter_witx() };
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(language).map_err(Error::LanguageError)?;
    Ok(parser)
}
