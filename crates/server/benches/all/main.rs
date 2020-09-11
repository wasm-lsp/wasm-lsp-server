use criterion::{criterion_group, criterion_main, Criterion};

mod corpus {
    pub const SPEC: &str = "../../vendor/corpus/vendor/WebAssembly/spec/test/core";
}

mod document_symbol {
    pub mod spec {
        use criterion::Criterion;
        use glob::glob;
        use wasm_language_server::{core::document::Document, service};

        pub fn all(c: &mut Criterion) {
            let mut documents = vec![];

            let paths = glob(&format!("{}/*.wast", crate::corpus::SPEC)).unwrap();
            let exclude = &[];

            for path in paths {
                let path = path.unwrap().canonicalize().unwrap();
                let file_name = path.file_name().unwrap().to_str().unwrap();

                if !exclude.contains(&file_name) {
                    let file_ext = path.extension().unwrap().to_str().unwrap();

                    let language_id = format!("wasm.{}", file_ext);
                    let text = std::fs::read_to_string(path).unwrap();
                    let document = Document::new(&language_id, text).unwrap().unwrap();

                    documents.push(document);
                }
            }

            let mut runtime = tokio::runtime::Builder::new().basic_scheduler().build().unwrap();

            c.bench_function("document_symbol::spec::*.wast", |b| {
                b.iter(|| {
                    for document in &documents {
                        runtime
                            .block_on(service::elaborator::wast::document_symbol(document))
                            .unwrap();
                    }
                })
            });
        }

        pub fn float_exprs(c: &mut Criterion) {
            let name = format!("{}/float_exprs.wast", crate::corpus::SPEC);
            let path = std::path::Path::new(&name);
            let path = path.canonicalize().unwrap();
            let file_ext = path.extension().unwrap().to_str().unwrap();

            let language_id = format!("wasm.{}", file_ext);
            let text = std::fs::read_to_string(path).unwrap();
            let document = Document::new(&language_id, text).unwrap().unwrap();

            let mut runtime = tokio::runtime::Builder::new().basic_scheduler().build().unwrap();

            c.bench_function("document_symbol::spec::float_exprs.wast", |b| {
                b.iter(|| {
                    runtime
                        .block_on(service::elaborator::wast::document_symbol(&document))
                        .unwrap();
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
