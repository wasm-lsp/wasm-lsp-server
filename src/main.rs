#![allow(dead_code)]

mod analyzer;
mod auditor;
mod cli;
mod database;
mod document;
mod elaborator;
mod error;
mod highlighter;
mod package;
mod parser;
mod server;
mod synthesizer;

use crate::server::Session;
use failure::Fallible;
use tower_lsp::{LspService, Server};
use tree_sitter::Language;

extern {
    #[doc(hidden)]
    fn tree_sitter_wast() -> Language;

    #[doc(hidden)]
    fn tree_sitter_wat() -> Language;

    #[doc(hidden)]
    fn tree_sitter_wit() -> Language;

    #[doc(hidden)]
    fn tree_sitter_witx() -> Language;
}

#[tokio::main]
async fn main() -> Fallible<()> {
    env_logger::try_init()?;

    crate::cli::cli();

    let session = Session::new()?;
    let (service, messages) = LspService::new(session);
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();
    Server::new(stdin, stdout).interleave(messages).serve(service).await;

    Ok(())
}
