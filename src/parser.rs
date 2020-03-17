use crate::error::Error;
use failure::Fallible;
use std::sync::Mutex;
use tree_sitter;

/// Owns tree-sitter [`Parser`] instances for supported WebAssembly documents.
///
/// [`Parser`]: https://docs.rs/tree-sitter/latest/tree_sitter/struct.Parser.html
pub struct Parser {
    pub wast: Mutex<tree_sitter::Parser>,
    pub wat: Mutex<tree_sitter::Parser>,
    pub wit: Mutex<tree_sitter::Parser>,
    pub witx: Mutex<tree_sitter::Parser>,
}

impl Parser {
    fn wast() -> Fallible<tree_sitter::Parser> {
        let language = unsafe { crate::tree_sitter_wast() };
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(language).map_err(Error::LanguageError)?;
        Ok(parser)
    }

    fn wat() -> Fallible<tree_sitter::Parser> {
        let language = unsafe { crate::tree_sitter_wat() };
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(language).map_err(Error::LanguageError)?;
        Ok(parser)
    }

    fn wit() -> Fallible<tree_sitter::Parser> {
        let language = unsafe { crate::tree_sitter_wit() };
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(language).map_err(Error::LanguageError)?;
        Ok(parser)
    }

    fn witx() -> Fallible<tree_sitter::Parser> {
        let language = unsafe { crate::tree_sitter_witx() };
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(language).map_err(Error::LanguageError)?;
        Ok(parser)
    }

    pub fn new() -> Fallible<Self> {
        let wat = Mutex::new(Self::wat()?);
        let wast = Mutex::new(Self::wast()?);
        let wit = Mutex::new(Self::wit()?);
        let witx = Mutex::new(Self::witx()?);
        Ok(Parser { wast, wat, wit, witx })
    }
}
