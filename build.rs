use std::path::Path;

fn main() {
    let dir = Path::new("vendor/tree-sitter-webassembly/src");
    cc::Build::new()
        .include(&dir)
        .file(dir.join("parser.c"))
        .compile("tree-sitter-webassembly");
}
