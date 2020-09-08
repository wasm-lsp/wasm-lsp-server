//! Macros for the WASM language server.

#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

mod util {
    pub(crate) mod language_id {
        #[derive(thiserror::Error, Debug)]
        pub(crate) enum Error {
            #[error("Failed to compute language id for extension: {ext:?}")]
            LanguageId { ext: String },
        }

        pub(crate) fn from_ext(ext: &str) -> anyhow::Result<&str> {
            match ext {
                "wast" => Ok("wasm.wast"),
                "wat" => Ok("wasm.wat"),
                ext => Err(Error::LanguageId { ext: ext.to_owned() }.into()),
            }
        }
    }
}

mod corpus {
    mod keyword {
        syn::custom_keyword!(corpus);
        syn::custom_keyword!(include);
        syn::custom_keyword!(exclude);
    }

    use syn::parse::{Parse, ParseStream};

    pub(crate) struct TestsMacroInput {
        pub(crate) corpus: syn::Ident,
        pub(crate) include: String,
        pub(crate) exclude: Vec<String>,
    }

    impl Parse for TestsMacroInput {
        fn parse(input: ParseStream) -> syn::parse::Result<Self> {
            input.parse::<keyword::corpus>()?;
            input.parse::<syn::Token![:]>()?;
            let corpus = input.parse()?;
            input.parse::<syn::Token![,]>()?;

            input.parse::<keyword::include>()?;
            input.parse::<syn::Token![:]>()?;
            let include = input.parse::<syn::LitStr>()?.value();
            input.parse::<syn::Token![,]>()?;

            input.parse::<keyword::exclude>()?;
            input.parse::<syn::Token![:]>()?;
            let exclude = {
                let content;
                syn::bracketed!(content in input);
                content.parse_terminated::<syn::LitStr, syn::Token![,]>(|b| b.parse())?
            };
            let exclude = exclude.into_iter().map(|s| s.value()).collect();
            input.parse::<syn::Token![,]>().ok();

            Ok(TestsMacroInput {
                corpus,
                include,
                exclude,
            })
        }
    }
}

use glob::glob;
use proc_macro::TokenStream;
use quote::ToTokens;

/// Generate tests from a corpus of wasm modules on the filesystem.
///
/// # Arguments
///
/// * `corpus` - name of the generated submodule containing the individual tests
/// * `include` - glob pattern of files to include for testing
/// * `exclude` - array of file names to exclude from testing
///
/// # Examples
///
/// ```
/// corpus_tests! {
///     corpus: annotations,
///     include: "vendor/corpus/vendor/WebAssembly/annotations/test/core/*.wast",
///     exclude: ["annotations.wast"],
/// }
/// ```
#[proc_macro]
pub fn corpus_tests(input: TokenStream) -> TokenStream {
    let corpus::TestsMacroInput {
        corpus,
        include,
        exclude,
    } = syn::parse_macro_input!(input as corpus::TestsMacroInput);
    // compute the paths from the glob pattern
    let paths = glob(&include).unwrap();

    // prepare the vector of syntax items; these items are the individual test
    // functions that will be enclosed in the generated test submodule
    let mut content = Vec::<syn::Item>::new();

    for path in paths {
        // ensure the path is canonicalized and absolute
        let path = path.unwrap().canonicalize().unwrap();
        let path_name = path.to_str().unwrap();
        let file_name = path.file_name().unwrap().to_str().unwrap();

        // skip the file if contained in the exclude list; otherwise continue
        if !exclude.contains(&String::from(file_name)) {
            let file_stem = path.file_stem().unwrap().to_str().unwrap();
            let file_ext = path.extension().unwrap().to_str().unwrap();

            let test_name = str::replace(file_stem, "-", "_");
            let test_name = format!("r#{}", test_name);
            let test_name = syn::parse_str::<syn::Ident>(&test_name).unwrap();

            let language_id = crate::util::language_id::from_ext(file_ext).unwrap();

            // generate the individual test function for the given file
            let item: syn::Item = syn::parse_quote! {
                #[tokio::test]
                async fn #test_name() -> anyhow::Result<()> {
                    let uri = Url::from_file_path(&#path_name).unwrap();
                    let text = std::fs::read_to_string(#path_name).unwrap();

                    let (mut service, mut messages) = testing::service::spawn()?;
                    let service = &mut service;

                    testing::assert_status!(service, Ok(()));
                    let request = &testing::lsp::initialize::request();
                    let response = Some(testing::lsp::initialize::response());
                    testing::assert_exchange!(service, request, Ok(response));

                    testing::assert_status!(service, Ok(()));
                    let notification =
                        &testing::lsp::text_document::did_open::notification(&uri, #language_id, 1, text);
                    let status = None::<Value>;
                    testing::assert_exchange!(service, notification, Ok(status));

                    let message = messages.next().await.unwrap();
                    let actual = serde_json::to_value(&message)?;
                    let expected = testing::lsp::text_document::publish_diagnostics::notification(&uri, &[]);
                    assert_eq!(actual, expected);

                    Ok(())
                }
            };
            content.push(item);
        }
    }

    // generate the enclosing test submodule for the given corpus
    let module: syn::ItemMod = syn::parse_quote! {
        mod #corpus {
            use futures::stream::StreamExt;
            use serde_json::{json, Value};
            use std::task::Poll;
            use tower_lsp::lsp_types::*;
            use wasm_language_server_testing as testing;

            // include the test functions generated from the corpus files
            #(#content)*
        }
    };

    module.to_token_stream().into()
}
