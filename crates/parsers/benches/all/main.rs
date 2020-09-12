use criterion::{criterion_group, criterion_main, Criterion};

mod corpus {
    pub const SPEC: &str = "../../vendor/corpus/vendor/WebAssembly/spec/test/core";
}

mod parse {
    pub mod spec {
        use criterion::Criterion;
        use glob::glob;
        use std::convert::TryFrom;
        use wasm_language_server_parsers::core::language::Language;

        pub fn all(c: &mut Criterion) {
            let mut documents = vec![];

            let paths = glob(&format!("{}/*.wast", crate::corpus::SPEC)).unwrap();

            for path in paths {
                let path = path.unwrap().canonicalize().unwrap();
                let text = std::fs::read_to_string(&path).unwrap();
                documents.push((path, text));
            }

            c.bench_function("parse::spec::*.wast", |b| {
                b.iter(|| {
                    for (path, text) in &documents {
                        let language = Language::try_from(path.as_path()).unwrap();
                        let mut parser = tree_sitter::Parser::try_from(language).unwrap();
                        let old_tree = None;
                        parser.parse(&text[..], old_tree).unwrap();
                    }
                })
            });
        }

        pub fn float_exprs(c: &mut Criterion) {
            let mut documents = vec![];

            let paths = glob(&format!("{}/float_exprs.wast", crate::corpus::SPEC)).unwrap();

            for path in paths {
                let path = path.unwrap().canonicalize().unwrap();
                let text = std::fs::read_to_string(&path).unwrap();
                documents.push((path, text));
            }

            c.bench_function("parse::spec::float_exprs.wast", |b| {
                b.iter(|| {
                    for (path, text) in &documents {
                        let language = Language::try_from(path.as_path()).unwrap();
                        let mut parser = tree_sitter::Parser::try_from(language).unwrap();
                        let old_tree = None;
                        parser.parse(&text[..], old_tree).unwrap();
                    }
                })
            });
        }
    }
}

criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = parse::spec::all, parse::spec::float_exprs
}
criterion_main!(benches);
