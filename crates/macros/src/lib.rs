//! Macros for the WASM language server.

#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

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

            let mut exclude = vec![];
            if input.peek(keyword::exclude) {
                input.parse::<keyword::exclude>()?;
                input.parse::<syn::Token![:]>()?;
                let paths = {
                    let content;
                    syn::bracketed!(content in input);
                    content.parse_terminated::<syn::LitStr, syn::Token![,]>(|b| b.parse())?
                };
                exclude = paths.into_iter().map(|s| s.value()).collect();
                input.parse::<syn::Token![,]>()?;
            }

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
use std::convert::TryInto;
use wasm_language_server_parsers::core::language::Language;

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

            let test_name = heck::SnakeCase::to_snake_case(file_stem);
            let test_name = format!("r#{}", test_name);
            let test_name = syn::parse_str::<syn::Ident>(&test_name).unwrap();

            let language: Language = format!("wasm.{}", file_ext).as_str().try_into().unwrap();
            let language_id = language.id();

            // generate the individual test function for the given file
            let item: syn::Item = syn::parse_quote! {
                #[tokio::test]
                async fn #test_name() -> anyhow::Result<()> {
                    let uri = Url::from_file_path(&#path_name).unwrap();
                    let text = std::fs::read_to_string(#path_name).unwrap();

                    let (ref mut service, ref mut messages) = testing::service::spawn()?;

                    // send "initialize" request
                    testing::assert_status!(service, Ok(()));
                    let request = &testing::lsp::initialize::request();
                    let response = Some(testing::lsp::initialize::response());
                    testing::assert_exchange!(service, request, Ok(response));

                    // send "initialized" notification
                    testing::assert_status!(service, Ok(()));
                    let notification = &testing::lsp::initialized::notification();
                    let status = None::<Value>;
                    testing::assert_exchange!(service, notification, Ok(status));
                    // ignore the "window/logMessage" notification: "WebAssembly language server initialized!"
                    messages.next().await.unwrap();

                    // send "textDocument/didOpen" notification for `uri`
                    testing::assert_status!(service, Ok(()));
                    let notification =
                        &testing::lsp::text_document::did_open::notification(&uri, #language_id, 1, text);
                    let status = None::<Value>;
                    testing::assert_exchange!(service, notification, Ok(status));

                    // receive "textDocument/publishDiagnostics" notification for `uri`
                    let message = messages.next().await.unwrap();
                    let actual = serde_json::to_value(&message)?;
                    let expected = testing::lsp::text_document::publish_diagnostics::notification(&uri, &[]);
                    assert_eq!(actual, expected);

                    // send "shutdown" request
                    testing::assert_status!(service, Ok(()));
                    let request = &testing::lsp::shutdown::request();
                    let response = Some(testing::lsp::shutdown::response());
                    testing::assert_exchange!(service, request, Ok(response));

                    // send "exit" notification
                    testing::assert_status!(service, Ok(()));
                    let notification = &testing::lsp::exit::notification();
                    let status = None::<Value>;
                    testing::assert_exchange!(service, notification, Ok(status));

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
