mod analyzer;
mod auditor;
mod cli;
mod database;
mod document;
mod elaborator;
mod error;
mod highlighter;
mod lsp;
mod package;
mod parser;
mod server;
mod synthesizer;

use crate::server::Session;
use failure::Fallible;
use tower_lsp::{LspService, Server};
use tree_sitter::Language;

extern {
    #[allow(dead_code)]
    #[doc(hidden)]
    fn tree_sitter_wast() -> Language;

    #[doc(hidden)]
    fn tree_sitter_wat() -> Language;

    #[allow(dead_code)]
    #[doc(hidden)]
    fn tree_sitter_wit() -> Language;

    #[allow(dead_code)]
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
