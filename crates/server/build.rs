use std::path::Path;

type Error = Box<dyn std::error::Error + 'static>;

fn compile_tree_sitter_grammars() -> Result<(), Error> {
    let dir = Path::new("../../vendor/tree-sitter-wasm");
    cc::Build::new()
        .include(dir.join("wast/src"))
        .file(dir.join("wast/src/parser.c"))
        .flag("-Wno-overflow")
        .compile("tree-sitter-wast");
    cc::Build::new()
        .include(dir.join("wat/src"))
        .file(dir.join("wat/src/parser.c"))
        .flag("-Wno-overflow")
        .compile("tree-sitter-wat");
    cc::Build::new()
        .include(dir.join("wit/src"))
        .file(dir.join("wit/src/parser.c"))
        .flag("-Wno-overflow")
        .compile("tree-sitter-wit");
    cc::Build::new()
        .include(dir.join("witx/src"))
        .file(dir.join("witx/src/parser.c"))
        .flag("-Wno-overflow")
        .compile("tree-sitter-witx");
    Ok(())
}

fn collect_metadata() -> Result<(), Error> {
    built::write_built_file()?;
    Ok(())
}

fn main() -> Result<(), Error> {
    compile_tree_sitter_grammars()?;
    collect_metadata()?;
    Ok(())
}
