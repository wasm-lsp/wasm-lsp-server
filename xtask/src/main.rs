//! Cargo xtask definitions for the wasm-language-server project.

#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

type Fallible<T> = Result<T, Box<dyn std::error::Error>>;

fn main() -> Fallible<()> {
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
                clap::SubCommand::with_name("grcov").arg(rest).arg(
                    clap::Arg::with_name("rebuild-parsers")
                        .long("rebuild-parsers")
                        .help("Rebuild tree-sitter parsers if necessary"),
                ),
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
                clap::SubCommand::with_name("tarpaulin").arg(rest).arg(
                    clap::Arg::with_name("rebuild-parsers")
                        .long("rebuild-parsers")
                        .help("Rebuild tree-sitter parsers if necessary"),
                ),
                clap::SubCommand::with_name("test").arg(rest).arg(
                    clap::Arg::with_name("rebuild-parsers")
                        .long("rebuild-parsers")
                        .help("Rebuild tree-sitter parsers if necessary"),
                ),
                clap::SubCommand::with_name("udeps").arg(rest),
            ];
            subcommands
        });

    match app.get_matches().subcommand() {
        ("check", Some(sub_matches)) => subcommand::cargo::check(sub_matches)?,
        ("clippy", Some(sub_matches)) => subcommand::cargo::clippy(sub_matches)?,
        ("doc", Some(sub_matches)) => subcommand::cargo::doc(sub_matches)?,
        ("format", Some(sub_matches)) => subcommand::cargo::format(sub_matches)?,
        ("grcov", Some(sub_matches)) => subcommand::cargo::grcov(sub_matches)?,
        ("init", Some(sub_matches)) => subcommand::init(sub_matches)?,
        ("install", Some(sub_matches)) => subcommand::cargo::install(sub_matches)?,
        ("tarpaulin", Some(sub_matches)) => subcommand::cargo::tarpaulin(sub_matches)?,
        ("test", Some(sub_matches)) => subcommand::cargo::test(sub_matches)?,
        ("udeps", Some(sub_matches)) => subcommand::cargo::udeps(sub_matches)?,
        _ => {},
    }

    Ok(())
}

mod metadata {
    use std::path::{Path, PathBuf};

    pub fn cargo() -> crate::Fallible<String> {
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
        pub fn check(sub_matches: &clap::ArgMatches) -> crate::Fallible<()> {
            let cargo = metadata::cargo()?;
            let mut cmd = Command::new(cargo);
            cmd.current_dir(metadata::project_root());
            cmd.env("RUSTFLAGS", "-Dwarnings");
            cmd.args(&["check", "--all-targets"]);
            cmd.args(&["--package", "xtask"]);
            cmd.args(&["--package", "wasm-language-server"]);
            cmd.args(&["--package", "wasm-language-server-macros"]);
            cmd.args(&["--package", "wasm-language-server-parsers"]);
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
        pub fn clippy(sub_matches: &clap::ArgMatches) -> crate::Fallible<()> {
            let cargo = metadata::cargo()?;
            let mut cmd = Command::new(cargo);
            cmd.current_dir(metadata::project_root());
            cmd.args(&["clippy", "--all-targets"]);
            cmd.args(&["--package", "xtask"]);
            cmd.args(&["--package", "wasm-language-server"]);
            cmd.args(&["--package", "wasm-language-server-macros"]);
            cmd.args(&["--package", "wasm-language-server-parsers"]);
            cmd.args(&["--package", "wasm-language-server-testing"]);
            if cfg!(target_os = "linux") {
                cmd.args(&["--package", "wasm-language-server-fuzz"]);
            }
            if let Some(values) = sub_matches.values_of("rest") {
                cmd.args(values);
            }
            cmd.args(&["--", "-D", "warnings"]);
            cmd.status()?;
            Ok(())
        }

        // Run `cargo doc` with custom options.
        pub fn doc(sub_matches: &clap::ArgMatches) -> crate::Fallible<()> {
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
        pub fn format(sub_matches: &clap::ArgMatches) -> crate::Fallible<()> {
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

        // Run `cargo grcov` with custom options.
        pub fn grcov(sub_matches: &clap::ArgMatches) -> crate::Fallible<()> {
            if sub_matches.is_present("rebuild-parsers") {
                crate::util::tree_sitter::rebuild_parsers()?;
            }

            let cargo = metadata::cargo()?;
            let mut cmd = Command::new(cargo);
            cmd.current_dir(metadata::project_root());
            cmd.env("CARGO_INCREMENTAL", "0");
            #[rustfmt::skip]
            cmd.env("RUSTFLAGS", "-Dwarnings -Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort");
            cmd.env("RUSTDOCFLAGS", "-Cpanic=abort");
            cmd.args(&[
                "+nightly",
                "test",
                "--all-features",
                "--benches",
                "--examples",
                "--lib",
                "--tests",
            ]);
            cmd.args(&["--package", "wasm-language-server"]);
            cmd.args(&["--package", "wasm-language-server-parsers"]);
            if let Some(values) = sub_matches.values_of("rest") {
                cmd.args(values);
            }
            cmd.status()?;

            let mut cmd = Command::new("grcov");
            cmd.current_dir(metadata::project_root());
            cmd.arg("./target/debug/");
            cmd.args(&["--source-dir", "."]);
            cmd.args(&["--output-type", "html"]);
            cmd.args(&["--output-path", "./target/debug/coverage/"]);
            cmd.args(&["--llvm", "--branch", "--ignore-not-existing"]);
            cmd.args(&["--ignore", "crates/testing"]);
            cmd.args(&["--ignore", "xpath"]);
            cmd.status()?;

            Ok(())
        }

        // Run `cargo install` with custom options.
        pub fn install(sub_matches: &clap::ArgMatches) -> crate::Fallible<()> {
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

        // Run `cargo tarpaulin` with custom options.
        pub fn tarpaulin(sub_matches: &clap::ArgMatches) -> crate::Fallible<()> {
            if sub_matches.is_present("rebuild-parsers") {
                crate::util::tree_sitter::rebuild_parsers()?;
            }

            let cargo = metadata::cargo()?;
            let mut cmd = Command::new(cargo);
            cmd.current_dir(metadata::project_root());
            cmd.args(&["+nightly", "tarpaulin"]);
            cmd.args(&["-Zpackage-features"]);
            cmd.args(&["--out", "Xml"]);
            cmd.args(&[
                "--packages",
                "xtask",
                "wasm-language-server",
                "wasm-language-server-macros",
                "wasm-language-server-parsers",
                "wasm-language-server-testing",
            ]);
            cmd.args(&[
                "--exclude-files",
                "xtask",
                "crates/macros",
                "crates/server/src/bin",
                "crates/server/src/cli.rs",
                "crates/testing",
                "tests",
                "vendor",
                "**/stdio2.h",
                "**/string_fortified.h",
            ]);
            if let Some(values) = sub_matches.values_of("rest") {
                cmd.args(values);
            }
            cmd.status()?;

            Ok(())
        }

        // Run `cargo test` with custom options.
        pub fn test(sub_matches: &clap::ArgMatches) -> crate::Fallible<()> {
            if sub_matches.is_present("rebuild-parsers") {
                crate::util::tree_sitter::rebuild_parsers()?;
            }

            let cargo = metadata::cargo()?;
            let mut cmd = Command::new(cargo);
            cmd.current_dir(metadata::project_root());
            cmd.env("RUSTFLAGS", "-Dwarnings");
            cmd.args(&["test", "--examples", "--lib", "--tests"]);
            cmd.args(&["--package", "xtask"]);
            cmd.args(&["--package", "wasm-language-server"]);
            cmd.args(&["--package", "wasm-language-server-macros"]);
            cmd.args(&["--package", "wasm-language-server-parsers"]);
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

        // Run `cargo udeps` with custom options.
        pub fn udeps(sub_matches: &clap::ArgMatches) -> crate::Fallible<()> {
            let cargo = metadata::cargo()?;
            let mut cmd = Command::new(cargo);
            cmd.current_dir(metadata::project_root());
            cmd.args(&["+nightly", "udeps", "--all-targets", "--all-features"]);
            cmd.args(&["--package", "xtask"]);
            cmd.args(&["--package", "wasm-language-server"]);
            cmd.args(&["--package", "wasm-language-server-macros"]);
            cmd.args(&["--package", "wasm-language-server-parsers"]);
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
    pub fn init(sub_matches: &clap::ArgMatches) -> crate::Fallible<()> {
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
        pub fn rebuild_parsers() -> crate::Fallible<()> {
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
            for grammar in &["wast", "wat"] {
                // Configure the grammar directory path.
                let grammar_path = [tree_sitter_path, grammar].iter().collect::<PathBuf>();
                let grammar_path = dunce::canonicalize(grammar_path)?;
                let grammar_path = grammar_path.to_str().unwrap();

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
