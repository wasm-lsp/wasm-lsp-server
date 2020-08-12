mod metadata {
    #![allow(dead_code)]
    include!(concat!(env!("OUT_DIR"), "/built.rs"));

    pub fn cargo() -> anyhow::Result<String> {
        // NOTE: we use the cargo wrapper rather than the binary reported through the "CARGO" environment
        // variable because we need to be able to invoke cargo with different toolchains (e.g., +nightly)
        Ok(String::from("cargo"))
    }

    pub fn project_root() -> std::path::PathBuf {
        std::path::Path::new(&env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .nth(1)
            .unwrap()
            .to_path_buf()
    }
}

fn main() -> anyhow::Result<()> {
    let app = clap::App::new("xtask").subcommands(vec![
        clap::SubCommand::with_name("check"),
        clap::SubCommand::with_name("clippy"),
        clap::SubCommand::with_name("doc"),
        clap::SubCommand::with_name("format"),
        clap::SubCommand::with_name("install"),
        clap::SubCommand::with_name("test"),
    ]);

    let matches = app.get_matches_safe()?;

    if matches.subcommand_matches("check").is_some() {
        let cargo = metadata::cargo()?;
        let mut cmd = std::process::Command::new(cargo);
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
    }

    if matches.subcommand_matches("clippy").is_some() {
        let cargo = metadata::cargo()?;
        let mut cmd = std::process::Command::new(cargo);
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
    }

    if matches.subcommand_matches("doc").is_some() {
        let cargo = metadata::cargo()?;
        let mut cmd = std::process::Command::new(cargo);
        cmd.current_dir(metadata::project_root());
        cmd.args(&["+nightly", "doc", "--all-features", "--no-deps"]);
        cmd.status()?;
    }

    if matches.subcommand_matches("format").is_some() {
        let cargo = metadata::cargo()?;
        let mut cmd = std::process::Command::new(cargo);
        cmd.current_dir(metadata::project_root());
        cmd.args(&["+nightly", "fmt", "--all"]);
        cmd.status()?;
    }

    if matches.subcommand_matches("install").is_some() {
        let cargo = metadata::cargo()?;
        let mut cmd = std::process::Command::new(cargo);
        cmd.current_dir(metadata::project_root());
        cmd.args(&["install", "--path", "crates/server", "--offline"]);
        cmd.status()?;
    }

    if matches.subcommand_matches("test").is_some() {
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
    }

    Ok(())
}
