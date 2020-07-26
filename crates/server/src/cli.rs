//! Command-line interface for the WASM language server.

use crate::package::metadata;
use clap::App;

/// Invokes the command-line interface for the WASM language server.
pub fn cli() {
    App::new(metadata::PKG_NAME)
        .author(metadata::PKG_AUTHORS)
        .version(metadata::PKG_VERSION)
        .about(metadata::PKG_DESCRIPTION)
        .get_matches();
}
