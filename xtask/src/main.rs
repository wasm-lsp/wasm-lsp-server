//! Cargo xtask definitions for the wasm-language-server project.

#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

mod metadata {
    use std::path::{Path, PathBuf};

    pub fn cargo() -> anyhow::Result<String> {
        // NOTE: we use the cargo wrapper rather than the binary reported through the "CARGO" environment
        // variable because we need to be able to invoke cargo with different toolchains (e.g., +nightly)
        Ok(String::from("cargo"))
    }

    pub fn project_root() -> PathBuf {
        Path::new(&env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .nth(1)
            .unwrap()
            .to_path_buf()
    }
}

mod subcommand {
    pub mod cargo {
        use crate::metadata;
        use std::process::Command;

        // Run `cargo check` with custom options.
        pub fn check() -> anyhow::Result<()> {
            let cargo = metadata::cargo()?;
            let mut cmd = Command::new(cargo);
            cmd.current_dir(metadata::project_root());
            cmd.env("RUSTFLAGS", "-Dwarnings");
            cmd.args(&[
                "check",
                "--all-features",
                "--all-targets",
                "--benches",
                "--bins",
                "--examples",
                "--tests",
                "--workspace",
            ]);
            cmd.status()?;
            Ok(())
        }

        // Run `cargo clippy` with custom options.
        pub fn clippy() -> anyhow::Result<()> {
            let cargo = metadata::cargo()?;
            let mut cmd = Command::new(cargo);
            cmd.current_dir(metadata::project_root());
            cmd.args(&[
                "clippy",
                "--all-features",
                "--all-targets",
                "--benches",
                "--bins",
                "--examples",
                "--tests",
                "--workspace",
                "--",
                "-D",
                "warnings",
            ]);
            cmd.status()?;
            Ok(())
        }

        // Run `cargo doc` with custom options.
        pub fn doc() -> anyhow::Result<()> {
            let cargo = metadata::cargo()?;
            let mut cmd = Command::new(cargo);
            cmd.current_dir(metadata::project_root());
            cmd.args(&["+nightly", "doc", "--all-features", "--no-deps"]);
            cmd.status()?;
            Ok(())
        }

        // Run `cargo format` with custom options.
        pub fn format() -> anyhow::Result<()> {
            let cargo = metadata::cargo()?;
            let mut cmd = Command::new(cargo);
            cmd.current_dir(metadata::project_root());
            cmd.args(&["+nightly", "fmt", "--all"]);
            cmd.status()?;
            Ok(())
        }

        // Run `cargo install` with custom options.
        pub fn install() -> anyhow::Result<()> {
            let cargo = metadata::cargo()?;
            let mut cmd = std::process::Command::new(cargo);
            cmd.current_dir(metadata::project_root());
            cmd.args(&["install", "--path", "crates/server", "--offline"]);
            cmd.status()?;
            Ok(())
        }

        // Run `cargo test` with custom options.
        pub fn test() -> anyhow::Result<()> {
            let cargo = metadata::cargo()?;
            let mut cmd = std::process::Command::new(cargo);
            cmd.current_dir(metadata::project_root());
            cmd.env("RUSTFLAGS", "-Dwarnings");
            cmd.args(&[
                "test",
                "--all-features",
                "--all-targets",
                "--benches",
                "--bins",
                "--examples",
                "--tests",
                "--workspace",
            ]);
            cmd.status()?;
            Ok(())
        }
    }

    use crate::metadata;
    use std::{
        path::{Path, PathBuf},
        process::Command,
    };

    // Initialize submodules (e.g., for tree-sitter grammars and test suites)
    pub fn init(matches: &clap::ArgMatches) -> anyhow::Result<()> {
        if let Some(matches) = matches.subcommand_matches("init") {
            // initialize "vendor/tree-sitter-wasm" submodule
            let submodule = Path::new("vendor/tree-sitter-wasm").to_str().unwrap();
            let mut cmd = Command::new("git");
            cmd.current_dir(metadata::project_root());
            cmd.args(&["submodule", "update", "--init", "--depth", "1", "--", submodule]);
            cmd.status()?;

            if matches.is_present("with-corpus") {
                // initialize "vendor/corpus" submodule
                let submodule = Path::new("vendor/corpus").to_str().unwrap();
                let mut cmd = Command::new("git");
                cmd.current_dir(metadata::project_root());
                cmd.args(&["submodule", "update", "--init", "--depth", "1", "--", submodule]);
                cmd.status()?;

                // initialize "vendor/corpus/..." submodules
                let mut cmd = Command::new("git");
                let root = metadata::project_root();
                let path = [root.to_str().unwrap(), "vendor", "corpus"].iter().collect::<PathBuf>();
                cmd.current_dir(path);
                cmd.args(&["submodule", "update", "--init", "--depth", "1"]);
                cmd.status()?;
            }
        }

        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    let app = clap::App::new("xtask").subcommands(vec![
        clap::SubCommand::with_name("check"),
        clap::SubCommand::with_name("clippy"),
        clap::SubCommand::with_name("doc"),
        clap::SubCommand::with_name("format"),
        clap::SubCommand::with_name("init").arg(
            clap::Arg::with_name("with-corpus")
                .long("with-corpus")
                .help("Initialize corpus submodules"),
        ),
        clap::SubCommand::with_name("install"),
        clap::SubCommand::with_name("test"),
    ]);

    let matches = app.get_matches_safe()?;

    match matches.subcommand_name() {
        Some("check") => subcommand::cargo::check()?,
        Some("clippy") => subcommand::cargo::clippy()?,
        Some("doc") => subcommand::cargo::doc()?,
        Some("format") => subcommand::cargo::format()?,
        Some("init") => subcommand::init(&matches)?,
        Some("install") => subcommand::cargo::install()?,
        Some("test") => subcommand::cargo::test()?,
        _ => {},
    }

    Ok(())
}
