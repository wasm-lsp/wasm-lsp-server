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

#[cfg(feature = "runtime-async-std")]
fn run() -> anyhow::Result<()> {
    env_logger::try_init()?;
    cli();
    async_std::task::block_on(async {
        let (service, messages) = LspService::new(|client| wasm_lsp_server::Server::new(client).unwrap());
        let stdin = async_std::io::stdin();
        let stdout = async_std::io::stdout();
        Server::new(stdin, stdout).interleave(messages).serve(service).await;
        Ok(())
    })
}

#[cfg(feature = "runtime-futures")]
fn run() -> anyhow::Result<()> {
    env_logger::try_init()?;
    cli();
    futures::future::block_on(async {
        let (service, messages) = LspService::new(|client| wasm_lsp_server::Server::new(client).unwrap());
        let stdin = blocking::Unblock::new(std::io::stdin());
        let stdout = blocking::Unblock::new(std::io::stdout());
        Server::new(stdin, stdout).interleave(messages).serve(service).await;
        Ok(())
    })
}

#[cfg(feature = "runtime-smol")]
fn run() -> anyhow::Result<()> {
    env_logger::try_init()?;
    cli();
    smol::block_on(async {
        let (service, messages) = LspService::new(|client| wasm_lsp_server::Server::new(client).unwrap());
        let stdin = smol::Unblock::new(std::io::stdin());
        let stdout = smol::Unblock::new(std::io::stdout());
        Server::new(stdin, stdout).interleave(messages).serve(service).await;
        Ok(())
    })
}

#[cfg(feature = "runtime-tokio")]
fn run() -> anyhow::Result<()> {
    env_logger::try_init()?;
    cli();
    tokio::runtime::Runtime::new()?.block_on(async {
        let (service, messages) = LspService::new(|client| wasm_lsp_server::Server::new(client).unwrap());
        let stdin = tokio::io::stdin();
        let stdout = tokio::io::stdout();
        Server::new(stdin, stdout).interleave(messages).serve(service).await;
        Ok(())
    })
}
