//! The WASM language server CLI.

#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

use clap::App;
use lspower::{LspService, Server};

fn cli() {
    use wasm_language_server::package::metadata;
    App::new(metadata::PKG_NAME)
        .author(metadata::PKG_AUTHORS)
        .version(metadata::PKG_VERSION)
        .about(metadata::PKG_DESCRIPTION)
        .get_matches();
}

#[cfg(feature = "runtime-smol")]
fn main() -> smol::io::Result<()> {
    smol::block_on(async {
        env_logger::try_init().expect("failed to initialize environment logger");

        cli();

        let (service, messages) = LspService::new(|client| wasm_language_server::server::Server::new(client).unwrap());
        let stdin = smol::Unblock::new(std::io::stdin());
        let stdout = smol::Unblock::new(std::io::stdout());
        Server::new(stdin, stdout).interleave(messages).serve(service).await;

        Ok(())
    })
}

#[cfg(feature = "runtime-tokio")]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::try_init()?;

    cli();

    let (service, messages) = LspService::new(|client| wasm_language_server::server::Server::new(client).unwrap());
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();
    Server::new(stdin, stdout).interleave(messages).serve(service).await;

    Ok(())
}
