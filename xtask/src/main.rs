//! Cargo xtask definitions for the wasm-language-server project.

#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

type Fallible<T> = Result<T, Box<dyn std::error::Error>>;

fn main() -> Fallible<()> {
    let help = r#"
xtask

USAGE:
    xtask [SUBCOMMAND]

FLAGS:
    -h, --help          Prints help information

SUBCOMMANDS:
    build
    check
    clippy
    doc
    format
    help                Prints this message or the help of the subcommand(s)
    init
    install
    tarpaulin
    test
    udeps
"#
    .trim();

    let mut args: Vec<_> = std::env::args_os().collect();
    // remove "xtask" argument
    args.remove(0);

    let cargo_args = if let Some(dash_dash) = args.iter().position(|arg| arg == "--") {
        let c = args.drain(dash_dash + 1 ..).collect();
        args.pop();
        c
    } else {
        Vec::new()
    };

    let mut args = pico_args::Arguments::from_vec(args);
    match args.subcommand()?.as_deref() {
        Some("build") => {
            subcommand::cargo::build(args, &cargo_args)?;
            return Ok(());
        },
        Some("check") => {
            subcommand::cargo::check(args, &cargo_args)?;
            return Ok(());
        },
        Some("clippy") => {
            subcommand::cargo::clippy(args, &cargo_args)?;
            return Ok(());
        },
        Some("doc") => {
            subcommand::cargo::doc(args, &cargo_args)?;
            return Ok(());
        },
        Some("format") => {
            subcommand::cargo::format(args, &cargo_args)?;
            return Ok(());
        },
        Some("help") => {
            println!("{}\n", help);
            return Ok(());
        },
        Some("init") => {
            subcommand::init(args)?;
            return Ok(());
        },
        Some("install") => {
            subcommand::cargo::install(args, &cargo_args)?;
            return Ok(());
        },
        Some("tarpaulin") => {
            subcommand::cargo::tarpaulin(args, &cargo_args)?;
            return Ok(());
        },
        Some("test") => {
            subcommand::cargo::test(args, &cargo_args)?;
            return Ok(());
        },
        Some("udeps") => {
            subcommand::cargo::udeps(args, &cargo_args)?;
            return Ok(());
        },
        Some(subcommand) => {
            return Err(format!("unknown subcommand: {}", subcommand).into());
        },
        None => {
            if args.contains(["-h", "--help"]) {
                println!("{}\n", help);
                return Ok(());
            }
        },
    }

    if let Err(pico_args::Error::UnusedArgsLeft(args)) = args.finish() {
        return Err(format!("unrecognized arguments: {}", args.join(" ")).into());
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

        // Run `cargo build` with custom options.
        pub fn build(mut args: pico_args::Arguments, cargo_args: &[std::ffi::OsString]) -> crate::Fallible<()> {
            let help = r#"
xtask-build

USAGE:
    xtask build

FLAGS:
    -h, --help          Prints help information
    --rebuild-parsers   Rebuild tree-sitter parsers
    -- '...'            Extra arguments to pass to the cargo command
"#
            .trim();

            if args.contains(["-h", "--help"]) {
                println!("{}\n", help);
                return Ok(());
            }

            if args.contains("--rebuild-parsers") {
                crate::util::tree_sitter::rebuild_parsers()?;
            }

            let cargo = metadata::cargo()?;
            let mut cmd = Command::new(cargo);
            cmd.current_dir(metadata::project_root());
            cmd.args(&["build", "--package", "wasm-language-server-cli"]);
            cmd.args(cargo_args);
            cmd.status()?;

            Ok(())
        }

        // Run `cargo check` with custom options.
        pub fn check(mut args: pico_args::Arguments, cargo_args: &[std::ffi::OsString]) -> crate::Fallible<()> {
            let help = r#"
xtask-check

USAGE:
    xtask check

FLAGS:
    -h, --help          Prints help information
    -- '...'            Extra arguments to pass to the cargo command
"#
            .trim();

            if args.contains(["-h", "--help"]) {
                println!("{}\n", help);
                return Ok(());
            }

            let cargo = metadata::cargo()?;
            let mut cmd = Command::new(cargo);
            cmd.current_dir(metadata::project_root());
            cmd.env("RUSTFLAGS", "-Dwarnings");
            cmd.args(&["check", "--all-targets"]);
            cmd.args(&["--package", "xtask"]);
            cmd.args(&["--package", "wasm-language-server"]);
            cmd.args(&["--package", "wasm-language-server-cli"]);
            cmd.args(&["--package", "wasm-language-server-macros"]);
            cmd.args(&["--package", "wasm-language-server-parsers"]);
            cmd.args(&["--package", "wasm-language-server-testing"]);
            if cfg!(target_os = "linux") {
                cmd.args(&["--package", "wasm-language-server-fuzz"]);
            }
            cmd.args(cargo_args);
            cmd.status()?;
            Ok(())
        }

        // Run `cargo clippy` with custom options.
        pub fn clippy(mut args: pico_args::Arguments, cargo_args: &[std::ffi::OsString]) -> crate::Fallible<()> {
            let help = r#"
xtask-clippy

USAGE:
    xtask clippy

FLAGS:
    -h, --help          Prints help information
    -- '...'            Extra arguments to pass to the cargo command
"#
            .trim();

            if args.contains(["-h", "--help"]) {
                println!("{}\n", help);
                return Ok(());
            }

            let cargo = metadata::cargo()?;
            let mut cmd = Command::new(cargo);
            cmd.current_dir(metadata::project_root());
            cmd.args(&["clippy", "--all-targets"]);
            cmd.args(&["--package", "xtask"]);
            cmd.args(&["--package", "wasm-language-server"]);
            cmd.args(&["--package", "wasm-language-server-cli"]);
            cmd.args(&["--package", "wasm-language-server-macros"]);
            cmd.args(&["--package", "wasm-language-server-parsers"]);
            cmd.args(&["--package", "wasm-language-server-testing"]);
            if cfg!(target_os = "linux") {
                cmd.args(&["--package", "wasm-language-server-fuzz"]);
            }
            cmd.args(cargo_args);
            cmd.args(&["--", "-D", "warnings"]);
            cmd.status()?;
            Ok(())
        }

        // Run `cargo doc` with custom options.
        pub fn doc(mut args: pico_args::Arguments, cargo_args: &[std::ffi::OsString]) -> crate::Fallible<()> {
            let help = r#"
xtask-doc

USAGE:
    xtask doc

FLAGS:
    -h, --help          Prints help information
    -- '...'            Extra arguments to pass to the cargo command
"#
            .trim();

            if args.contains(["-h", "--help"]) {
                println!("{}\n", help);
                return Ok(());
            }

            let cargo = metadata::cargo()?;
            let mut cmd = Command::new(cargo);
            cmd.current_dir(metadata::project_root());
            cmd.args(&["+nightly", "doc"]);
            cmd.args(cargo_args);
            cmd.status()?;
            Ok(())
        }

        // Run `cargo format` with custom options.
        pub fn format(mut args: pico_args::Arguments, cargo_args: &[std::ffi::OsString]) -> crate::Fallible<()> {
            let help = r#"
xtask-format

USAGE:
    xtask format

FLAGS:
    -h, --help          Prints help information
    -- '...'            Extra arguments to pass to the cargo command
"#
            .trim();

            if args.contains(["-h", "--help"]) {
                println!("{}\n", help);
                return Ok(());
            }

            let cargo = metadata::cargo()?;
            let mut cmd = Command::new(cargo);
            cmd.current_dir(metadata::project_root());
            cmd.args(&["+nightly", "fmt", "--all"]);
            cmd.args(cargo_args);
            cmd.status()?;
            Ok(())
        }

        // Run `cargo install` with custom options.
        pub fn install(mut args: pico_args::Arguments, cargo_args: &[std::ffi::OsString]) -> crate::Fallible<()> {
            let help = r#"
xtask-install

USAGE:
    xtask install

FLAGS:
    -h, --help          Prints help information
    --rebuild-parsers   Rebuild tree-sitter parsers
    -- '...'            Extra arguments to pass to the cargo command
"#
            .trim();

            if args.contains(["-h", "--help"]) {
                println!("{}\n", help);
                return Ok(());
            }

            if args.contains("--rebuild-parsers") {
                crate::util::tree_sitter::rebuild_parsers()?;
            }

            let cargo = metadata::cargo()?;
            let mut cmd = Command::new(cargo);
            cmd.current_dir(metadata::project_root());
            cmd.args(&["install", "--path", "crates/cli"]);
            cmd.args(cargo_args);
            cmd.status()?;

            Ok(())
        }

        // Run `cargo tarpaulin` with custom options.
        pub fn tarpaulin(mut args: pico_args::Arguments, cargo_args: &[std::ffi::OsString]) -> crate::Fallible<()> {
            let help = r#"
xtask-tarpaulin

USAGE:
    xtask tarpaulin

FLAGS:
    -h, --help          Prints help information
    --rebuild-parsers   Rebuild tree-sitter parsers
    -- '...'            Extra arguments to pass to the cargo command
"#
            .trim();

            if args.contains(["-h", "--help"]) {
                println!("{}\n", help);
                return Ok(());
            }

            if args.contains("--rebuild-parsers") {
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
                "wasm-language-server-cli",
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
            cmd.args(cargo_args);
            cmd.status()?;

            Ok(())
        }

        // Run `cargo test` with custom options.
        pub fn test(mut args: pico_args::Arguments, cargo_args: &[std::ffi::OsString]) -> crate::Fallible<()> {
            let help = r#"
xtask-test

USAGE:
    xtask test

FLAGS:
    -h, --help          Prints help information
    --rebuild-parsers   Rebuild tree-sitter parsers
    -- '...'            Extra arguments to pass to the cargo command
"#
            .trim();

            if args.contains(["-h", "--help"]) {
                println!("{}\n", help);
                return Ok(());
            }

            if args.contains("--rebuild-parsers") {
                crate::util::tree_sitter::rebuild_parsers()?;
            }

            let cargo = metadata::cargo()?;
            let mut cmd = Command::new(cargo);
            cmd.current_dir(metadata::project_root());
            cmd.env("RUSTFLAGS", "-Dwarnings");
            cmd.args(&["test", "--examples", "--lib", "--tests"]);
            cmd.args(&["--package", "xtask"]);
            cmd.args(&["--package", "wasm-language-server"]);
            cmd.args(&["--package", "wasm-language-server-cli"]);
            cmd.args(&["--package", "wasm-language-server-macros"]);
            cmd.args(&["--package", "wasm-language-server-parsers"]);
            cmd.args(&["--package", "wasm-language-server-testing"]);
            if cfg!(target_os = "linux") {
                cmd.args(&["--package", "wasm-language-server-fuzz"]);
            }
            cmd.args(cargo_args);
            cmd.status()?;

            Ok(())
        }

        // Run `cargo udeps` with custom options.
        pub fn udeps(mut args: pico_args::Arguments, cargo_args: &[std::ffi::OsString]) -> crate::Fallible<()> {
            let help = r#"
xtask-udep

USAGE:
    xtask udep

FLAGS:
    -h, --help          Prints help information
    -- '...'            Extra arguments to pass to the cargo command
"#
            .trim();

            if args.contains(["-h", "--help"]) {
                println!("{}\n", help);
                return Ok(());
            }

            let cargo = metadata::cargo()?;
            let mut cmd = Command::new(cargo);
            cmd.current_dir(metadata::project_root());
            cmd.args(&["+nightly", "udeps", "--all-targets", "--all-features"]);
            cmd.args(&["--package", "xtask"]);
            cmd.args(&["--package", "wasm-language-server"]);
            cmd.args(&["--package", "wasm-language-server-cli"]);
            cmd.args(&["--package", "wasm-language-server-macros"]);
            cmd.args(&["--package", "wasm-language-server-parsers"]);
            cmd.args(&["--package", "wasm-language-server-testing"]);
            if cfg!(target_os = "linux") {
                cmd.args(&["--package", "wasm-language-server-fuzz"]);
            }
            cmd.args(cargo_args);
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
    pub fn init(mut args: pico_args::Arguments) -> crate::Fallible<()> {
        let help = r#"
xtask-init

USAGE:
    xtask init

FLAGS:
    -h, --help          Prints help information
"#
        .trim();

        if args.contains(["-h", "--help"]) {
            println!("{}\n", help);
            return Ok(());
        }

        // initialize "vendor/tree-sitter-wasm" submodule
        let submodule = Path::new("vendor/tree-sitter-wasm").to_str().unwrap();
        let mut cmd = Command::new("git");
        cmd.current_dir(metadata::project_root());
        cmd.args(&["submodule", "update", "--init", "--depth", "1", "--", submodule]);
        cmd.status()?;

        if args.contains("--with-corpus") {
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
