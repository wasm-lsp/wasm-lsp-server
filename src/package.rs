//! Definitions related to the wasm-language-server crate.

/// Metadata for the crate computed from the Cargo manifest and repository.
pub(crate) mod metadata {
    #![allow(dead_code)]
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}
