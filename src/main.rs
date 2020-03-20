#![allow(dead_code)]

mod analyzer;
mod database;
mod elaborator;
mod error;
mod highlighter;
mod parser;
mod server;
mod session;
mod synchronizer;

use crate::{
    analyzer::Analyzer,
    database::Database,
    elaborator::Elaborator,
    highlighter::Highlighter,
    parser::Parser,
    session::Session,
    synchronizer::Synchronizer,
};
use failure::Fallible;
use futures::future::join_all;
use std::sync::Arc;
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

    let synchronizer = Arc::new(Synchronizer::new(Arc::new(Parser::new()?))?);
    let rx = &synchronizer.rx;

    let database = Arc::new(Database::new()?);
    let analyzer = Arc::new(Analyzer::new(database.clone(), rx.clone(), synchronizer.clone())?);
    let elaborator = Arc::new(Elaborator::new(database.clone(), rx.clone(), synchronizer.clone())?);
    let highlighter = Arc::new(Highlighter::new(rx.clone(), synchronizer.clone())?);

    let session = Session::new(synchronizer.clone())?;
    let (service, messages) = LspService::new(session);
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();
    let server = Server::new(stdin, stdout).interleave(messages).serve(service);

    join_all(vec![
        tokio::spawn(async move { analyzer.init().await }),
        tokio::spawn(async move { elaborator.init().await }),
        tokio::spawn(async move { highlighter.init().await }),
        tokio::spawn(async move { server.await }),
    ])
    .await;

    Ok(())
}
