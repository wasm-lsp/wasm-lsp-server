use std::path::Path;

fn compile_tree_sitter_grammars() {
    let dir = Path::new("../../vendor/tree-sitter-wasm");

    println!("cargo:rerun-if-changed={:?}", dir.join("wast/src/parser.c"));
    let mut cc = cc::Build::new();
    cc.include(dir.join("wast/src"));
    cc.file(dir.join("wast/src/parser.c"));
    cc.compile("tree-sitter-wast");

    println!("cargo:rerun-if-changed={:?}", dir.join("wat/src/parser.c"));
    let mut cc = cc::Build::new();
    cc.include(dir.join("wat/src"));
    cc.file(dir.join("wat/src/parser.c"));
    cc.compile("tree-sitter-wat");
}

fn main() {
    compile_tree_sitter_grammars();
}
