#![deny(clippy::all)]
#![deny(unsafe_code)]

use clap::App;
use lspower::{LspService, Server};

fn cli() {
    use wasm_lsp_server::package::metadata;
    App::new(metadata::PKG_NAME)
        .author(metadata::PKG_AUTHORS)
        .version(metadata::PKG_VERSION)
        .about(metadata::PKG_DESCRIPTION)
        .get_matches();
}

fn main() -> anyhow::Result<()> {
    run()?;
    Ok(())
}

#[cfg(feature = "runtime-futures")]
fn run() -> anyhow::Result<()> {
    futures::future::block_on(async {
        env_logger::try_init()?;

        cli();

        let (service, messages) = LspService::new(|client| wasm_lsp_server::Server::new(client).unwrap());
        let stdin = blocking::Unblock::new(std::io::stdin());
        let stdout = blocking::Unblock::new(std::io::stdout());
        Server::new(stdin, stdout).interleave(messages).serve(service).await;

        Ok(())
    })
}

#[cfg(feature = "runtime-smol")]
fn run() -> anyhow::Result<()> {
    smol::block_on(async {
        env_logger::try_init()?;

        cli();

        let (service, messages) = LspService::new(|client| wasm_lsp_server::Server::new(client).unwrap());
        let stdin = smol::Unblock::new(std::io::stdin());
        let stdout = smol::Unblock::new(std::io::stdout());
        Server::new(stdin, stdout).interleave(messages).serve(service).await;

        Ok(())
    })
}

#[cfg(feature = "runtime-tokio")]
fn run() -> anyhow::Result<()> {
    tokio::runtime::Runtime::new()?.block_on(async {
        env_logger::try_init()?;

        cli();

        let (service, messages) = LspService::new(|client| wasm_lsp_server::Server::new(client).unwrap());
        let stdin = tokio::io::stdin();
        let stdout = tokio::io::stdout();
        Server::new(stdin, stdout).interleave(messages).serve(service).await;

        Ok(())
    })
}
