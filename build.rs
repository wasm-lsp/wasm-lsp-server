use std::path::Path;

fn main() {
    let dir = Path::new("vendor/tree-sitter-webassembly");
    cc::Build::new()
        .include(dir.join("wast/src"))
        .file(dir.join("wast/src/parser.c"))
        .compile("tree-sitter-wast");
    cc::Build::new()
        .include(dir.join("wat/src"))
        .file(dir.join("wat/src/parser.c"))
        .compile("tree-sitter-wat");
    cc::Build::new()
        .include(dir.join("wit/src"))
        .file(dir.join("wit/src/parser.c"))
        .compile("tree-sitter-wit");
    cc::Build::new()
        .include(dir.join("witx/src"))
        .file(dir.join("witx/src/parser.c"))
        .compile("tree-sitter-witx");
}
