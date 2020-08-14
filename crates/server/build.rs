use std::path::Path;

type Error = Box<dyn std::error::Error + 'static>;

fn compile_tree_sitter_grammars() -> Result<(), Error> {
    let dir = Path::new("../../vendor/tree-sitter-wasm");

    println!("cargo:rerun-if-changed={:?}", dir.join("wast/src/parser.c"));
    let mut cc = cc::Build::new();
    cc.include(dir.join("wast/src"));
    cc.file(dir.join("wast/src/parser.c"));
    cc.flag_if_supported("-Wno-overflow");
    cc.compile("tree-sitter-wast");

    println!("cargo:rerun-if-changed={:?}", dir.join("wat/src/parser.c"));
    let mut cc = cc::Build::new();
    cc.include(dir.join("wat/src"));
    cc.file(dir.join("wat/src/parser.c"));
    cc.flag_if_supported("-Wno-overflow");
    cc.compile("tree-sitter-wat");

    println!("cargo:rerun-if-changed={:?}", dir.join("wit/src/parser.c"));
    let mut cc = cc::Build::new();
    cc.include(dir.join("wit/src"));
    cc.file(dir.join("wit/src/parser.c"));
    cc.flag_if_supported("-Wno-overflow");
    cc.compile("tree-sitter-wit");

    println!("cargo:rerun-if-changed={:?}", dir.join("witx/src/parser.c"));
    let mut cc = cc::Build::new();
    cc.include(dir.join("witx/src"));
    cc.file(dir.join("witx/src/parser.c"));
    cc.flag_if_supported("-Wno-overflow");
    cc.compile("tree-sitter-witx");

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
