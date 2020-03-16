#![allow(dead_code)]

mod analyzer;
mod database;
mod error;
mod parser;
mod server;
mod session;

use crate::session::Session;
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
    let session = Session::new()?;
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();
    let (service, messages) = LspService::new(session);
    Server::new(stdin, stdout).interleave(messages).serve(service).await;
    Ok(())
}
