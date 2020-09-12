#[cfg(feature = "corpus")]
mod corpus {
    use wasm_language_server_macros::corpus_tests;

    fn handler(path: &str) {
        use std::convert::TryFrom;
        use wasm_language_server_parsers::core::language::Language;

        fn handler(path: &str) -> anyhow::Result<()> {
            let path = std::path::Path::new(path);
            let text = std::fs::read_to_string(path)?;
            let language = Language::try_from(path)?;

            let mut parser = tree_sitter::Parser::try_from(language)?;
            let old_tree = None;
            let tree = parser.parse(&text[..], old_tree);

            assert!(tree.is_some());
            if let Some(tree) = tree {
                assert!(!tree.root_node().has_error());
            }

            Ok(())
        }

        handler(path).unwrap();
    }

    corpus_tests! {
        corpus: annotations,
        include: "vendor/corpus/vendor/WebAssembly/annotations/test/core/*.wast",
        exclude: ["comments.wast"],
        handler: crate::corpus::handler,
    }

    corpus_tests! {
        corpus: bulk_memory_operations,
        include: "vendor/corpus/vendor/WebAssembly/bulk-memory-operations/test/core/*.wast",
        exclude: ["comments.wast"],
        handler: crate::corpus::handler,
    }

    corpus_tests! {
        corpus: exception_handling,
        include: "vendor/corpus/vendor/WebAssembly/exception-handling/test/core/*.wast",
        exclude: ["comments.wast"],
        handler: crate::corpus::handler,
    }

    corpus_tests! {
        corpus: function_references,
        include: "vendor/corpus/vendor/WebAssembly/function-references/test/core/*.wast",
        exclude: [
            "comments.wast",
            // FIXME: determine why the parser reports errors but the server does not
            "ref_null.wast",
            // FIXME: determine why the parser reports errors but the server does not
            "select.wast",
        ],
        handler: crate::corpus::handler,
    }

    corpus_tests! {
        corpus: interface_types,
        include: "vendor/corpus/vendor/bytecodealliance/wasm-interface-types/tests/*.wat",
        exclude: [
            // FIXME: fails because language id should be wasm.wast not wasm.wat
            "bad-schema.wat",
            // FIXME: fails because language id should be wasm.wast not wasm.wat
            "bad-section.wat",
            // NOTE: true positive; fails due to syntax error
            "not-interface.wat",
            // FIXME: fails because language id should be wasm.wast not wasm.wat
            "two-sections.wat",
        ],
        handler: crate::corpus::handler,
    }

    corpus_tests! {
        corpus: multi_memory,
        include: "vendor/corpus/vendor/WebAssembly/multi-memory/test/core/*.wast",
        exclude: ["comments.wast"],
        handler: crate::corpus::handler,
    }

    corpus_tests! {
        corpus: reference_types,
        include: "vendor/corpus/vendor/WebAssembly/reference-types/test/core/*.wast",
        exclude: ["comments.wast"],
        handler: crate::corpus::handler,
    }

    corpus_tests! {
        corpus: simd,
        include: "vendor/corpus/vendor/WebAssembly/simd/test/core/**/*.wast",
        exclude: ["comments.wast"],
        handler: crate::corpus::handler,
    }

    corpus_tests! {
        corpus: spec,
        include: "vendor/corpus/vendor/WebAssembly/spec/test/core/*.wast",
        exclude: ["comments.wast"],
        handler: crate::corpus::handler,
    }

    corpus_tests! {
        corpus: threads,
        include: "vendor/corpus/vendor/WebAssembly/threads/test/core/*.wast",
        exclude: ["comments.wast"],
        handler: crate::corpus::handler,
    }
}
