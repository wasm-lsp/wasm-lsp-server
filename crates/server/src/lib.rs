#![deny(clippy::all)]
#![deny(unsafe_code)]

pub mod core;
pub mod package;
mod server;
pub mod service;

pub use server::*;
