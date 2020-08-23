//! Cargo xtask definitions for the wasm-language-server project.

#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

fn main() -> anyhow::Result<()> {
    env_logger::try_init()?;

    let app = clap::App::new("xtask")
        .setting(clap::AppSettings::TrailingVarArg)
        .subcommands({
            let rest = &clap::Arg::with_name("rest")
                .help("Extra arguments to pass to the underlying cargo command")
                .last(true)
                .multiple(true);
            let subcommands = vec![
                clap::SubCommand::with_name("check").arg(rest),
                clap::SubCommand::with_name("clippy").arg(rest),
                clap::SubCommand::with_name("doc").arg(rest),
                clap::SubCommand::with_name("format").arg(rest),
                clap::SubCommand::with_name("init").arg(
                    clap::Arg::with_name("with-corpus")
                        .long("with-corpus")
                        .help("Initialize corpus submodules"),
                ),
                clap::SubCommand::with_name("install").arg(rest).arg(
                    clap::Arg::with_name("rebuild-parsers")
                        .long("rebuild-parsers")
                        .help("Rebuild tree-sitter parsers if necessary"),
                ),
                clap::SubCommand::with_name("test").arg(rest).arg(
                    clap::Arg::with_name("rebuild-parsers")
                        .long("rebuild-parsers")
                        .help("Rebuild tree-sitter parsers if necessary"),
                ),
            ];
            subcommands
        });

    let matches = app.get_matches_safe()?;

    match matches.subcommand() {
        ("check", Some(sub_matches)) => subcommand::cargo::check(sub_matches)?,
        ("clippy", Some(sub_matches)) => subcommand::cargo::clippy(sub_matches)?,
        ("doc", Some(sub_matches)) => subcommand::cargo::doc(sub_matches)?,
        ("format", Some(sub_matches)) => subcommand::cargo::format(sub_matches)?,
        ("init", Some(sub_matches)) => subcommand::init(sub_matches)?,
        ("install", Some(sub_matches)) => subcommand::cargo::install(sub_matches)?,
        ("test", Some(sub_matches)) => subcommand::cargo::test(sub_matches)?,
        _ => {},
    }

    Ok(())
}

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
        pub fn check(sub_matches: &clap::ArgMatches) -> anyhow::Result<()> {
            let cargo = metadata::cargo()?;
            let mut cmd = Command::new(cargo);
            cmd.current_dir(metadata::project_root());
            cmd.env("RUSTFLAGS", "-Dwarnings");
            cmd.args(&["check", "--all-targets"]);
            cmd.args(&["--package", "xtask"]);
            cmd.args(&["--package", "wasm-language-server"]);
            cmd.args(&["--package", "wasm-language-server-macros"]);
            cmd.args(&["--package", "wasm-language-server-parsers"]);
            cmd.args(&["--package", "wasm-language-server-shared"]);
            cmd.args(&["--package", "wasm-language-server-testing"]);
            if cfg!(target_os = "linux") {
                cmd.args(&["--package", "wasm-language-server-fuzz"]);
            }
            if let Some(values) = sub_matches.values_of("rest") {
                cmd.args(values);
            }
            cmd.status()?;
            Ok(())
        }

        // Run `cargo clippy` with custom options.
        pub fn clippy(sub_matches: &clap::ArgMatches) -> anyhow::Result<()> {
            let cargo = metadata::cargo()?;
            let mut cmd = Command::new(cargo);
            cmd.current_dir(metadata::project_root());
            cmd.args(&["clippy", "--all-targets"]);
            cmd.args(&["--package", "xtask"]);
            cmd.args(&["--package", "wasm-language-server"]);
            cmd.args(&["--package", "wasm-language-server-macros"]);
            cmd.args(&["--package", "wasm-language-server-parsers"]);
            cmd.args(&["--package", "wasm-language-server-shared"]);
            cmd.args(&["--package", "wasm-language-server-testing"]);
            if cfg!(target_os = "linux") {
                cmd.args(&["--package", "wasm-language-server-fuzz"]);
            }
            cmd.args(&["--", "-D", "warnings"]);
            if let Some(values) = sub_matches.values_of("rest") {
                cmd.args(values);
            }
            cmd.status()?;
            Ok(())
        }

        // Run `cargo doc` with custom options.
        pub fn doc(sub_matches: &clap::ArgMatches) -> anyhow::Result<()> {
            let cargo = metadata::cargo()?;
            let mut cmd = Command::new(cargo);
            cmd.current_dir(metadata::project_root());
            cmd.args(&["+nightly", "doc"]);
            if let Some(values) = sub_matches.values_of("rest") {
                cmd.args(values);
            }
            cmd.status()?;
            Ok(())
        }

        // Run `cargo format` with custom options.
        pub fn format(sub_matches: &clap::ArgMatches) -> anyhow::Result<()> {
            let cargo = metadata::cargo()?;
            let mut cmd = Command::new(cargo);
            cmd.current_dir(metadata::project_root());
            cmd.args(&["+nightly", "fmt", "--all"]);
            if let Some(values) = sub_matches.values_of("rest") {
                cmd.args(values);
            }
            cmd.status()?;
            Ok(())
        }

        // Run `cargo install` with custom options.
        pub fn install(sub_matches: &clap::ArgMatches) -> anyhow::Result<()> {
            if sub_matches.is_present("rebuild-parsers") {
                crate::util::tree_sitter::rebuild_parsers()?;
            }

            let cargo = metadata::cargo()?;
            let mut cmd = Command::new(cargo);
            cmd.current_dir(metadata::project_root());
            cmd.args(&["install", "--path", "crates/server"]);
            if let Some(values) = sub_matches.values_of("rest") {
                cmd.args(values);
            }
            cmd.status()?;

            Ok(())
        }

        // Run `cargo test` with custom options.
        pub fn test(sub_matches: &clap::ArgMatches) -> anyhow::Result<()> {
            if sub_matches.is_present("rebuild-parsers") {
                crate::util::tree_sitter::rebuild_parsers()?;
            }

            let cargo = metadata::cargo()?;
            let mut cmd = Command::new(cargo);
            cmd.current_dir(metadata::project_root());
            cmd.env("RUSTFLAGS", "-Dwarnings");
            cmd.args(&["test", "--all-features", "--benches", "--examples", "--lib", "--tests"]);
            cmd.args(&["--package", "xtask"]);
            cmd.args(&["--package", "wasm-language-server"]);
            cmd.args(&["--package", "wasm-language-server-macros"]);
            cmd.args(&["--package", "wasm-language-server-parsers"]);
            cmd.args(&["--package", "wasm-language-server-shared"]);
            cmd.args(&["--package", "wasm-language-server-testing"]);
            if cfg!(target_os = "linux") {
                cmd.args(&["--package", "wasm-language-server-fuzz"]);
            }
            if let Some(values) = sub_matches.values_of("rest") {
                cmd.args(values);
            }
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
    pub fn init(sub_matches: &clap::ArgMatches) -> anyhow::Result<()> {
        // initialize "vendor/tree-sitter-wasm" submodule
        let submodule = Path::new("vendor/tree-sitter-wasm").to_str().unwrap();
        let mut cmd = Command::new("git");
        cmd.current_dir(metadata::project_root());
        cmd.args(&["submodule", "update", "--init", "--depth", "1", "--", submodule]);
        cmd.status()?;

        if sub_matches.is_present("with-corpus") {
            // initialize "vendor/corpus" submodule
            let submodule = Path::new("vendor/corpus").to_str().unwrap();
            let mut cmd = Command::new("git");
            cmd.current_dir(metadata::project_root());
            cmd.args(&["submodule", "update", "--init", "--depth", "1", "--", submodule]);
            cmd.status()?;

            // initialize "vendor/corpus/..." submodules
            let mut cmd = Command::new("git");
            let root = metadata::project_root();
            let root = root.to_str().unwrap();
            let path = [root, "vendor", "corpus"].iter().collect::<PathBuf>();
            cmd.current_dir(path);
            cmd.args(&["submodule", "update", "--init", "--depth", "1"]);
            cmd.status()?;
        }

        Ok(())
    }
}

mod util {
    pub mod tree_sitter {
        use crate::metadata;
        use std::{
            path::PathBuf,
            process::{Command, Stdio},
        };

        // Rebuild tree-sitter parsers if necessary.
        pub fn rebuild_parsers() -> anyhow::Result<()> {
            // Configure the project root path.
            let root_path = metadata::project_root();
            let root_path = root_path.to_str().unwrap();

            // Configure the tree-sitter directory path.
            let tree_sitter_path = [root_path, "vendor", "tree-sitter-wasm"].iter().collect::<PathBuf>();
            let tree_sitter_path = tree_sitter_path.to_str().unwrap();

            // Configure the tree-sitter cli binary path.
            let tree_sitter_cli_path = [tree_sitter_path, "node_modules", ".bin", "tree-sitter"]
                .iter()
                .collect::<PathBuf>();
            let tree_sitter_cli_path = tree_sitter_cli_path.to_str().unwrap();

            // Check if the tree-sitter cli binary is available.
            let mut cmd;
            if cfg!(target_os = "windows") {
                cmd = Command::new("cmd");
                cmd.args(&["/C", format!("{} --help", tree_sitter_cli_path).as_ref()]);
            } else {
                cmd = Command::new("sh");
                cmd.args(&["-c", format!("{} --help", tree_sitter_cli_path).as_ref()]);
            };
            cmd.stdout(Stdio::null());
            cmd.stderr(Stdio::null());

            // Run `npm ci` first if `tree-sitter` binary is not available.
            if !cmd.status()?.success() {
                log::info!("installing tree-sitter toolchain");
                let mut cmd;
                if cfg!(target_os = "windows") {
                    cmd = Command::new("cmd");
                    cmd.args(&["/C", "npm ci"]);
                } else {
                    cmd = Command::new("sh");
                    cmd.args(&["-c", "npm ci"]);
                }
                cmd.current_dir(tree_sitter_path);
                cmd.stdout(Stdio::null());
                cmd.stderr(Stdio::null());
                cmd.status()?;
            }

            // Iterate through the different grammar types.
            for grammar in &["wast", "wat", "wit", "witx"] {
                // Configure the grammar directory path.
                let grammar_path = [tree_sitter_path, grammar].iter().collect::<PathBuf>();
                let grammar_path = dunce::canonicalize(grammar_path)?;
                let grammar_path = grammar_path.to_str().unwrap();

                log::info!("regenerating parser: {}", grammar);
                let commands = format!("cd {} && {} generate", grammar_path, tree_sitter_cli_path);
                let mut cmd;
                if cfg!(target_os = "windows") {
                    cmd = Command::new("cmd");
                    cmd.args(&["/C", commands.as_ref()]);
                } else {
                    cmd = Command::new("sh");
                    cmd.args(&["-c", commands.as_ref()]);
                }
                let status = cmd.status()?;
                if !status.success() {
                    panic!("failed to regenerate parser: {}", grammar);
                }
            }

            Ok(())
        }
    }
}
