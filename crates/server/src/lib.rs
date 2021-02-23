#![deny(clippy::all)]
#![deny(unsafe_code)]
#![allow(clippy::if_same_then_else)]
#![allow(clippy::needless_lifetimes)]

pub mod core;
pub mod handler;
pub mod package;
pub mod provider;
pub mod server;

pub use server::*;
