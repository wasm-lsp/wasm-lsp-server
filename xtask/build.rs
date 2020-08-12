fn collect_metadata() -> anyhow::Result<()> {
    built::write_built_file()?;
    Ok(())
}

fn main() -> anyhow::Result<()> {
    collect_metadata()?;
    Ok(())
}
