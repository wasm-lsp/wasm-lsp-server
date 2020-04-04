//! The WASM language server.

#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

use failure::Fallible;
use tower_lsp::{LspService, Server};

#[tokio::main]
async fn main() -> Fallible<()> {
    env_logger::try_init()?;

    wasm_language_server::cli::cli();

    let server = wasm_language_server::lsp::server::Server::new()?;
    let (service, messages) = LspService::new(server);
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();
    Server::new(stdin, stdout).interleave(messages).serve(service).await;

    Ok(())
}
