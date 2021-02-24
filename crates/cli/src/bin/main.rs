//! Command-line interface for the WebAssembly Language Server

#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

use clap::App;
use lspower::{LspService, Server};

fn cli() {
    use wasm_lsp_server::metadata;
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
/// Run the server with the async-std runtime.
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
/// Run the server with the futures runtime.
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
/// Run the server with the smol runtime.
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
/// Run the server with the tokio runtime.
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
