mod error;
mod parser;
mod server;
mod session;

use crate::session::Session;
use failure::Fallible;
use tower_lsp::{LspService, Server};
use tree_sitter::Language;

extern {
    fn tree_sitter_webassembly() -> Language;
}

#[tokio::main]
async fn main() -> Fallible<()> {
    let session = Session::new()?;
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();
    let (service, messages) = LspService::new(session);
    Server::new(stdin, stdout).interleave(messages).serve(service).await;
    Ok(())
}
