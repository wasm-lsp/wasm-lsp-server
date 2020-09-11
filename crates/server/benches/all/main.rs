use criterion::{criterion_group, criterion_main, Criterion};

mod corpus {
    pub const SPEC: &str = "../../vendor/corpus/vendor/WebAssembly/spec/test/core";
}

mod document_symbol {
    pub mod spec {
        use criterion::Criterion;
        use glob::glob;
        use std::convert::TryFrom;
        use wasm_language_server::{core::document::Document, service};
        use wasm_language_server_parsers::core::language::Language;

        pub fn all(c: &mut Criterion) {
            let mut documents = vec![];

            let paths = glob(&format!("{}/*.wast", crate::corpus::SPEC)).unwrap();

            for path in paths {
                let path = path.unwrap().canonicalize().unwrap();
                let language = Language::try_from(path.as_path()).unwrap();
                let text = std::fs::read_to_string(path).unwrap();
                let document = Document::new(language.id(), text).unwrap().unwrap();

                documents.push(document);
            }

            let mut runtime = tokio::runtime::Builder::new().basic_scheduler().build().unwrap();

            c.bench_function("document_symbol::spec::*.wast", |b| {
                b.iter(|| {
                    for document in &documents {
                        runtime
                            .block_on(service::elaborator::wast::document_symbol_with_document(document))
                            .unwrap();
                    }
                })
            });
        }

        pub fn float_exprs(c: &mut Criterion) {
            let mut documents = vec![];

            let paths = glob(&format!("{}/float_exprs.wast", crate::corpus::SPEC)).unwrap();

            for path in paths {
                let path = path.unwrap().canonicalize().unwrap();
                let language = Language::try_from(path.as_path()).unwrap();
                let text = std::fs::read_to_string(path).unwrap();
                let document = Document::new(language.id(), text).unwrap().unwrap();

                documents.push(document);
            }

            let mut runtime = tokio::runtime::Builder::new().basic_scheduler().build().unwrap();

            c.bench_function("document_symbol::spec::float_exprs.wast", |b| {
                b.iter(|| {
                    for document in &documents {
                        runtime
                            .block_on(service::elaborator::wast::document_symbol_with_document(document))
                            .unwrap();
                    }
                })
            });
        }
    }
}

criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = document_symbol::spec::all, document_symbol::spec::float_exprs
}
criterion_main!(benches);
