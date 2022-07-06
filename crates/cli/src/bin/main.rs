//! Command-line interface for the WebAssembly Language Server

#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

use clap::Command;
use tower_lsp::{LspService, Server};

fn cli() {
    use wasm_lsp_server::metadata;
    Command::new(metadata::PKG_NAME)
        .author(metadata::PKG_AUTHORS)
        .version(metadata::PKG_VERSION)
        .about(metadata::PKG_DESCRIPTION)
        .get_matches();
}

fn main() -> anyhow::Result<()> {
    run()?;
    Ok(())
}

/// Run the server with the futures runtime.
fn run() -> anyhow::Result<()> {
    env_logger::try_init()?;
    cli();
    futures::executor::block_on(async {
        #[rustfmt::skip]
        let languages = wasm_lsp_server::core::SessionLanguages {
            wast: wasm_lsp_languages::language::wast(),
            wat : wasm_lsp_languages::language::wat (),
        };
        let (service, socket) = LspService::new(|client| wasm_lsp_server::Server::new(languages, client).unwrap());
        let stdin = blocking::Unblock::new(std::io::stdin());
        let stdout = blocking::Unblock::new(std::io::stdout());
        Server::new(stdin, stdout, socket).serve(service).await;
        Ok(())
    })
}
