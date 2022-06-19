#![deny(clippy::all)]
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
    test-cli
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

    let result = match args.subcommand()?.as_deref() {
        None => {
            if args.contains(["-h", "--help"]) {
                println!("{}\n", help);
            }
            Ok(())
        },
        Some("build") => subcommand::cargo::build(&mut args, cargo_args),
        Some("check") => subcommand::cargo::check(&mut args, cargo_args),
        Some("clippy") => subcommand::cargo::clippy(&mut args, cargo_args),
        Some("doc") => subcommand::cargo::doc(&mut args, cargo_args),
        Some("format") => subcommand::cargo::format(&mut args, cargo_args),
        Some("init") => subcommand::init(&mut args),
        Some("install") => subcommand::cargo::install(&mut args, cargo_args),
        Some("tarpaulin") => subcommand::cargo::tarpaulin(&mut args, cargo_args),
        Some("test") => subcommand::cargo::test(&mut args, cargo_args),
        Some("test-cli") => subcommand::cargo::test_cli(&mut args, cargo_args),
        Some("udeps") => subcommand::cargo::udeps(&mut args, cargo_args),
        Some("help") => {
            println!("{}\n", help);
            Ok(())
        },
        Some(subcommand) => Err(format!("unknown subcommand: {}", subcommand).into()),
    };
    crate::util::handle_result(result);

    let result = crate::util::handle_unused(&args);
    crate::util::handle_result(result);

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
        pub fn build(args: &mut pico_args::Arguments, mut cargo_args: Vec<std::ffi::OsString>) -> crate::Fallible<()> {
            let help = r#"
xtask-build

USAGE:
    xtask build

FLAGS:
    -h, --help          Prints help information
    --rebuild-parsers   Rebuild tree-sitter parsers
    --runtime=<arg>     Choice of runtime: agnostic, smol, tokio (default)
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

            let toolchain = crate::util::configure_runtime("build", args, &mut cargo_args)?;
            crate::util::handle_unused(args)?;

            let cargo = metadata::cargo()?;
            let mut cmd = Command::new(cargo);
            cmd.current_dir(metadata::project_root());
            cmd.args(toolchain);
            cmd.args(&["build"]);
            cmd.args(&["--package", "wasm-lsp-cli"]);
            cmd.args(cargo_args);
            cmd.status()?;

            Ok(())
        }

        // Run `cargo check` with custom options.
        pub fn check(args: &mut pico_args::Arguments, mut cargo_args: Vec<std::ffi::OsString>) -> crate::Fallible<()> {
            let help = r#"
xtask-check

USAGE:
    xtask check

FLAGS:
    -h, --help          Prints help information
    --runtime=<arg>     Choice of runtime: agnostic, smol, tokio (default)
    -- '...'            Extra arguments to pass to the cargo command
"#
            .trim();

            if args.contains(["-h", "--help"]) {
                println!("{}\n", help);
                return Ok(());
            }

            let toolchain = crate::util::configure_runtime("check", args, &mut cargo_args)?;
            crate::util::handle_unused(args)?;

            let cargo = metadata::cargo()?;
            let mut cmd = Command::new(cargo);
            cmd.current_dir(metadata::project_root());
            cmd.env("RUSTFLAGS", "-Dwarnings");
            cmd.args(toolchain);
            cmd.args(&["check"]);
            cmd.args(&["--all-targets"]);
            cmd.args(&["--package", "xtask"]);
            cmd.args(&["--package", "wasm-lsp-cli"]);
            cmd.args(&["--package", "wasm-lsp-languages"]);
            cmd.args(&["--package", "wasm-lsp-macros"]);
            cmd.args(&["--package", "wasm-lsp-server"]);
            cmd.args(&["--package", "wasm-lsp-syntax"]);
            cmd.args(&["--package", "wasm-lsp-testing"]);
            if cfg!(target_os = "linux") {
                cmd.args(&["--package", "wasm-lsp-fuzz"]);
            }
            cmd.args(cargo_args);
            cmd.status()?;

            Ok(())
        }

        // Run `cargo clippy` with custom options.
        pub fn clippy(args: &mut pico_args::Arguments, mut cargo_args: Vec<std::ffi::OsString>) -> crate::Fallible<()> {
            let help = r#"
xtask-clippy

USAGE:
    xtask clippy

FLAGS:
    -h, --help          Prints help information
    --runtime=<arg>     Choice of runtime: agnostic, smol, tokio (default)
    -- '...'            Extra arguments to pass to the cargo command
"#
            .trim();

            if args.contains(["-h", "--help"]) {
                println!("{}\n", help);
                return Ok(());
            }

            crate::util::configure_runtime("clippy", args, &mut cargo_args)?;
            crate::util::handle_unused(args)?;

            let cargo = metadata::cargo()?;
            let mut cmd = Command::new(cargo);
            cmd.current_dir(metadata::project_root());
            cmd.args(&["+nightly", "clippy"]);
            cmd.args(&["--all-targets"]);
            cmd.args(&["--package", "xtask"]);
            cmd.args(&["--package", "wasm-lsp-cli"]);
            cmd.args(&["--package", "wasm-lsp-languages"]);
            cmd.args(&["--package", "wasm-lsp-macros"]);
            cmd.args(&["--package", "wasm-lsp-server"]);
            cmd.args(&["--package", "wasm-lsp-syntax"]);
            cmd.args(&["--package", "wasm-lsp-testing"]);
            if cfg!(target_os = "linux") {
                cmd.args(&["--package", "wasm-lsp-fuzz"]);
            }
            cmd.args(cargo_args);
            cmd.args(&["--", "-D", "warnings"]);
            cmd.status()?;

            Ok(())
        }

        // Run `cargo doc` with custom options.
        pub fn doc(args: &mut pico_args::Arguments, cargo_args: Vec<std::ffi::OsString>) -> crate::Fallible<()> {
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

            crate::util::handle_unused(args)?;

            let cargo = metadata::cargo()?;
            let mut cmd = Command::new(cargo);
            cmd.current_dir(metadata::project_root());
            cmd.args(&["+nightly", "doc"]);
            cmd.args(cargo_args);
            cmd.status()?;

            Ok(())
        }

        // Run `cargo format` with custom options.
        pub fn format(args: &mut pico_args::Arguments, cargo_args: Vec<std::ffi::OsString>) -> crate::Fallible<()> {
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

            crate::util::handle_unused(args)?;

            let cargo = metadata::cargo()?;
            let mut cmd = Command::new(cargo);
            cmd.current_dir(metadata::project_root());
            cmd.args(&["+nightly", "fmt", "--all"]);
            cmd.args(cargo_args);
            cmd.status()?;

            Ok(())
        }

        // Run `cargo install` with custom options.
        pub fn install(
            args: &mut pico_args::Arguments,
            mut cargo_args: Vec<std::ffi::OsString>,
        ) -> crate::Fallible<()> {
            let help = r#"
xtask-install

USAGE:
    xtask install

FLAGS:
    -h, --help          Prints help information
    --rebuild-parsers   Rebuild tree-sitter parsers
    --runtime=<arg>     Choice of runtime: agnostic, smol, tokio (default)
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

            let toolchain = crate::util::configure_runtime("install", args, &mut cargo_args)?;
            crate::util::handle_unused(args)?;

            let cargo = metadata::cargo()?;
            let mut cmd = Command::new(cargo);
            let mut path = metadata::project_root();
            path.push("crates");
            path.push("cli");
            cmd.current_dir(path);
            cmd.args(toolchain);
            cmd.args(&["install"]);
            cmd.args(&["--path", "."]);
            cmd.args(cargo_args);
            cmd.status()?;

            Ok(())
        }

        // Run `cargo tarpaulin` with custom options.
        pub fn tarpaulin(
            args: &mut pico_args::Arguments,
            mut cargo_args: Vec<std::ffi::OsString>,
        ) -> crate::Fallible<()> {
            let help = r#"
xtask-tarpaulin

USAGE:
    xtask tarpaulin

FLAGS:
    -h, --help          Prints help information
    --rebuild-parsers   Rebuild tree-sitter parsers
    --runtime=<arg>     Choice of runtime: agnostic, smol, tokio (default)
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

            crate::util::configure_runtime("tarpaulin", args, &mut cargo_args)?;
            crate::util::handle_unused(args)?;

            let cargo = metadata::cargo()?;
            let mut cmd = Command::new(cargo);
            cmd.current_dir(metadata::project_root());
            cmd.args(&["+nightly", "tarpaulin"]);
            cmd.args(&["--out", "Xml"]);
            cmd.args(&[
                "--packages",
                "xtask",
                "wasm-lsp-cli",
                "wasm-lsp-languages",
                "wasm-lsp-macros",
                "wasm-lsp-server",
                "wasm-lsp-syntax",
                "wasm-lsp-testing",
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
        pub fn test(args: &mut pico_args::Arguments, cargo_args: Vec<std::ffi::OsString>) -> crate::Fallible<()> {
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

            crate::util::handle_unused(args)?;

            let cargo = metadata::cargo()?;
            let mut cmd = Command::new(cargo);
            cmd.current_dir(metadata::project_root());
            cmd.env("RUSTFLAGS", "-Dwarnings");
            cmd.args(&["test"]);
            cmd.args(&["--examples", "--lib", "--tests"]);
            cmd.args(&["--package", "wasm-lsp-languages"]);
            cmd.args(&["--package", "wasm-lsp-server"]);
            cmd.args(&["--package", "wasm-lsp-syntax"]);
            cmd.args(&["--package", "wasm-lsp-testing"]);
            if cfg!(target_os = "linux") {
                cmd.args(&["--package", "wasm-lsp-fuzz"]);
            }
            cmd.args(cargo_args);
            cmd.status()?;

            Ok(())
        }

        // Run `cargo test-cli` with custom options.
        pub fn test_cli(
            args: &mut pico_args::Arguments,
            mut cargo_args: Vec<std::ffi::OsString>,
        ) -> crate::Fallible<()> {
            let help = r#"
xtask-test-cli

USAGE:
    xtask test-cli

FLAGS:
    -h, --help          Prints help information
    --rebuild-parsers   Rebuild tree-sitter parsers
    --runtime=<arg>     Choice of runtime: agnostic, smol, tokio (default)
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

            let toolchain = crate::util::configure_runtime("test", args, &mut cargo_args)?;
            crate::util::handle_unused(args)?;

            let cargo = metadata::cargo()?;
            let mut cmd = Command::new(cargo);
            cmd.current_dir(metadata::project_root());
            cmd.env("RUSTFLAGS", "-Dwarnings");
            cmd.args(toolchain);
            cmd.args(&["test"]);
            cmd.args(&["--bins"]);
            cmd.args(&["--package", "wasm-lsp-cli"]);
            cmd.args(cargo_args);
            cmd.status()?;

            Ok(())
        }

        // Run `cargo udeps` with custom options.
        pub fn udeps(args: &mut pico_args::Arguments, mut cargo_args: Vec<std::ffi::OsString>) -> crate::Fallible<()> {
            let help = r#"
xtask-udep

USAGE:
    xtask udeps

FLAGS:
    -h, --help          Prints help information
    --runtime=<arg>     Choice of runtime: agnostic, smol, tokio (default)
    -- '...'            Extra arguments to pass to the cargo command
"#
            .trim();

            if args.contains(["-h", "--help"]) {
                println!("{}\n", help);
                return Ok(());
            }

            crate::util::configure_runtime("udep", args, &mut cargo_args)?;
            crate::util::handle_unused(args)?;

            let cargo = metadata::cargo()?;
            let mut cmd = Command::new(cargo);
            cmd.current_dir(metadata::project_root());
            cmd.args(&["+nightly", "udeps"]);
            cmd.args(&["--all-targets"]);
            cmd.args(&["--package", "xtask"]);
            cmd.args(&["--package", "wasm-lsp-cli"]);
            cmd.args(&["--package", "wasm-lsp-languages"]);
            cmd.args(&["--package", "wasm-lsp-macros"]);
            cmd.args(&["--package", "wasm-lsp-server"]);
            cmd.args(&["--package", "wasm-lsp-syntax"]);
            cmd.args(&["--package", "wasm-lsp-testing"]);
            if cfg!(target_os = "linux") {
                cmd.args(&["--package", "wasm-lsp-fuzz"]);
            }
            cmd.args(cargo_args);
            cmd.status()?;

            Ok(())
        }
    }

    use crate::metadata;
    use std::{path::Path, process::Command};

    // Initialize submodules (e.g., for tree-sitter grammars and test suites)
    pub fn init(args: &mut pico_args::Arguments) -> crate::Fallible<()> {
        let help = r#"
xtask-init

USAGE:
    xtask init

FLAGS:
    -h, --help          Prints help information
    --with-corpus
"#
        .trim();

        if args.contains(["-h", "--help"]) {
            println!("{}\n", help);
            return Ok(());
        }

        let with_corpus = args.contains("--with-corpus");

        crate::util::handle_unused(args)?;

        // initialize "vendor/tree-sitter-wasm" submodule
        let submodule = Path::new("vendor/tree-sitter-wasm").to_str().unwrap();
        let mut cmd = Command::new("git");
        cmd.current_dir(metadata::project_root());
        cmd.args(&["submodule", "update", "--init", "--depth", "1", "--", submodule]);
        cmd.status()?;

        if with_corpus {
            // initialize "vendor/corpus" submodule
            let submodule = Path::new("vendor/corpus").to_str().unwrap();
            let mut cmd = Command::new("git");
            cmd.current_dir(metadata::project_root());
            cmd.args(&["submodule", "update", "--init", "--depth", "1", "--", submodule]);
            cmd.status()?;

            // initialize "vendor/corpus/..." submodules
            let mut cmd = Command::new("git");
            let mut path = metadata::project_root();
            path.push("vendor");
            path.push("corpus");
            cmd.current_dir(path);
            cmd.args(&["submodule", "update", "--init", "--depth", "1"]);
            cmd.status()?;
        }

        Ok(())
    }
}

mod util {
    pub(super) fn handle_result<T>(result: crate::Fallible<T>) {
        if let Err(err) = result {
            println!("Error :: {}", err);
            std::process::exit(1);
        }
    }

    pub(super) fn handle_unused(args: &pico_args::Arguments) -> crate::Fallible<()> {
        use std::borrow::Borrow;
        let unused = args.clone().finish();
        if !unused.is_empty() {
            let mut message = String::new();
            for str in unused {
                message.push(' ');
                message.push_str(str.to_string_lossy().borrow());
            }
            Err(format!("unrecognized arguments '{}'", message).into())
        } else {
            Ok(())
        }
    }

    pub(super) fn configure_runtime(
        command_name: &str,
        args: &mut pico_args::Arguments,
        cargo_args: &mut Vec<std::ffi::OsString>,
    ) -> crate::Fallible<Vec<String>> {
        let mut toolchain = vec![];
        if let Some(arg) = args.opt_value_from_str::<_, std::ffi::OsString>("--runtime")? {
            if command_name == "install" {
                if arg == "async-std" {
                    let features = vec!["runtime-async-std"];
                    cargo_args.push("--no-default-features".into());
                    cargo_args.push(format!("--features={}", features.join(",")).into());
                } else if arg == "futures" {
                    let features = vec!["runtime-futures"];
                    cargo_args.push("--no-default-features".into());
                    cargo_args.push(format!("--features={}", features.join(",")).into());
                } else if arg == "smol" {
                    let features = vec!["runtime-smol"];
                    cargo_args.push("--no-default-features".into());
                    cargo_args.push(format!("--features={}", features.join(",")).into());
                } else if arg == "tokio" {
                    let features = vec!["runtime-tokio"];
                    cargo_args.push("--no-default-features".into());
                    cargo_args.push(format!("--features={}", features.join(",")).into());
                } else {
                    return Err(format!("unexpected runtime '{}'", arg.to_string_lossy()).into());
                }
            }
            if command_name != "install" {
                if arg == "async-std" {
                    let features = vec!["wasm-lsp-cli/runtime-async-std"];
                    cargo_args.push("--no-default-features".into());
                    cargo_args.push(format!("--features={}", features.join(",")).into());
                    toolchain.push(String::from("+nightly"));
                } else if arg == "futures" {
                    let features = vec!["wasm-lsp-cli/runtime-futures"];
                    cargo_args.push("--no-default-features".into());
                    cargo_args.push(format!("--features={}", features.join(",")).into());
                    toolchain.push(String::from("+nightly"));
                } else if arg == "smol" {
                    let features = vec!["wasm-lsp-cli/runtime-smol"];
                    cargo_args.push("--no-default-features".into());
                    cargo_args.push(format!("--features={}", features.join(",")).into());
                    toolchain.push(String::from("+nightly"));
                } else if arg == "tokio" {
                    let features = vec!["wasm-lsp-cli/runtime-tokio"];
                    cargo_args.push("--no-default-features".into());
                    cargo_args.push(format!("--features={}", features.join(",")).into());
                    toolchain.push(String::from("+nightly"));
                } else {
                    return Err(format!("unexpected runtime '{}'", arg.to_string_lossy()).into());
                }
            }
        }

        Ok(toolchain)
    }

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
