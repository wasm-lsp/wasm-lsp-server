//! Macros for the WASM language server.

#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

use glob::glob;
use proc_macro::TokenStream;
use quote::quote;

mod corpus {
    mod keyword {
        syn::custom_keyword!(corpus);
        syn::custom_keyword!(include);
        syn::custom_keyword!(exclude);
        syn::custom_keyword!(handler);
    }

    use syn::parse::{Parse, ParseStream};

    pub(crate) struct TestsMacroInput {
        pub(crate) corpus: syn::Ident,
        pub(crate) include: String,
        pub(crate) exclude: Vec<String>,
        pub(crate) handler: syn::Expr,
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

            input.parse::<keyword::handler>()?;
            input.parse::<syn::Token![:]>()?;
            let handler = input.parse()?;
            input.parse::<syn::Token![,]>().ok();

            Ok(TestsMacroInput {
                corpus,
                include,
                exclude,
                handler,
            })
        }
    }
}

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
        handler,
    } = syn::parse_macro_input!(input as corpus::TestsMacroInput);
    // Compute a string representation for the corpus name.
    let corpus_name = corpus.to_string();
    let corpus_name = corpus_name.as_str();

    // Compute the paths from the glob pattern.
    let paths = glob(&include).unwrap();

    // Prepare the vector of syntax items; these items are the individual test
    // functions that will be enclosed in the generated test submodule.
    let mut content = vec![];

    for path in paths {
        // Ensure the path is canonicalized and absolute
        let path = path.unwrap().canonicalize().unwrap();
        let path_name = path.to_str().unwrap();
        let file_name = path.file_name().unwrap().to_str().unwrap();

        // Skip the file if contained in the exclude list; otherwise continue.
        if !exclude.contains(&String::from(file_name)) {
            let file_stem = path.file_stem().unwrap().to_str().unwrap();
            let test_name = heck::SnakeCase::to_snake_case(file_stem);
            let test_name = format!("r#{}", test_name);

            // Compute the test identifier.
            let test = syn::parse_str::<syn::Ident>(&test_name).unwrap();

            // Generate the individual test function for the given file.
            let item = quote! {
                #[test]
                fn #test() {
                    #handler(#corpus_name, #path_name);
                }
            };
            content.push(item);
        }
    }

    // Generate the enclosing test submodule for the given corpus.
    let module = quote! {
        mod #corpus {
            // Include the test functions generated from the corpus files.
            #(#content)*
        }
    };

    module.into()
}
