#![allow(dead_code)]

mod analyzer;
mod auditor;
mod database;
mod elaborator;
mod error;
mod highlighter;
mod parser;
mod server;
mod session;
mod synchronizer;
mod synthesizer;

use crate::{
    analyzer::Analyzer,
    auditor::Auditor,
    database::Database,
    elaborator::Elaborator,
    highlighter::Highlighter,
    parser::Parser,
    session::{Session, SessionHandle},
    synchronizer::Synchronizer,
    synthesizer::Synthesizer,
};
use failure::Fallible;
use std::sync::{Arc, Weak};
use tokio::sync::RwLock;
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

    let lock = Arc::new(RwLock::new(Weak::new()));
    let handle = SessionHandle::new(lock.clone());

    let analyzer = Analyzer::new(handle.clone())?;
    let auditor = Auditor::new(handle.clone())?;
    let database = Database::new()?;
    let elaborator = Elaborator::new(handle.clone())?;
    let highlighter = Highlighter::new(handle.clone())?;
    let parser = Parser::new()?;
    let synchronizer = Synchronizer::new(parser, handle.clone())?;
    let synthesizer = Synthesizer::new(handle.clone())?;

    let session = Arc::new(Session::new(
        analyzer,
        auditor,
        database,
        elaborator,
        highlighter,
        synchronizer,
        synthesizer,
    )?);
    *lock.write().await = Arc::downgrade(&session.clone());

    let (service, messages) = LspService::new(session);
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();
    Server::new(stdin, stdout).interleave(messages).serve(service).await;

    Ok(())
}
