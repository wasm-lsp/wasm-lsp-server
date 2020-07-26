use glob::glob;
use proc_macro::TokenStream;
use quote::ToTokens;

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
            let content;

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
            syn::bracketed!(content in input);
            let exclude = content.parse_terminated::<syn::LitStr, syn::Token![,]>(|b| b.parse())?;
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

#[proc_macro]
pub fn corpus_tests(input: TokenStream) -> TokenStream {
    let corpus::TestsMacroInput {
        corpus,
        include,
        exclude,
    } = syn::parse_macro_input!(input as corpus::TestsMacroInput);
    let entries = glob(&include).unwrap();
    let mut content = Vec::<syn::Item>::new();
    for entry in entries {
        let path = entry.unwrap().canonicalize().unwrap();
        let path_name = path.to_str().unwrap();
        let file_name = path.file_name().unwrap().to_str().unwrap();
        if !exclude.contains(&String::from(file_name)) {
            let file_stem = path.file_stem().unwrap().to_str().unwrap();
            let extension = path.extension().and_then(std::ffi::OsStr::to_str);
            if let Some("wast") = extension {
                let test_name = str::replace(file_stem, "-", "_");
                let test_name = format!("_{}", test_name);
                let test_name = syn::parse_str::<syn::Ident>(&test_name).unwrap();
                let item: syn::Item = syn::parse_quote! {
                    #[tokio::test]
                    async fn #test_name() -> anyhow::Result<()> {
                        let uri = Url::from_file_path(&#path_name).unwrap();
                        let text = std::fs::read_to_string(#path_name).unwrap();

                        let (mut service, mut messages) = test::service::spawn()?;
                        let service = &mut service;

                        assert_ready!(service, Ok(()));
                        let request = &json!({
                            "jsonrpc": "2.0",
                            "method": "initialize",
                            "params": {
                                "capabilities":{},
                            },
                            "id": 1,
                        });
                        let response = Some(json!({
                            "jsonrpc": "2.0",
                            "result": {
                                "capabilities": {
                                    "documentSymbolProvider": true,
                                    "textDocumentSync": {
                                        "change": TextDocumentSyncKind::Full,
                                        "openClose": true,
                                    },
                                },
                            },
                            "id": 1,
                        }));
                        assert_exchange!(service, request, Ok(response));

                        assert_ready!(service, Ok(()));
                        let request = &json!({
                            "jsonrpc": "2.0",
                            "method": "textDocument/didOpen",
                            "params": {
                                "textDocument": {
                                    "uri": uri,
                                    "languageId": "wasm.wast",
                                    "version": 1,
                                    "text": text,
                                },
                            },
                        });
                        let response = None::<Value>;
                        assert_exchange!(service, request, Ok(response));

                        let message = messages.next().await.unwrap();
                        let actual = serde_json::from_str::<Value>(&message)?;
                        let expected = json!({
                            "jsonrpc": "2.0",
                            "method": "textDocument/publishDiagnostics",
                            "params": {
                                "uri": uri,
                                "diagnostics": [],
                            },
                        });
                        assert_eq!(actual, expected);

                        Ok(())
                    }
                };
                content.push(item);
            }
        }
    }

    let module: syn::ItemMod = syn::parse_quote! {
        mod #corpus {
            use futures::stream::StreamExt;
            use serde_json::{json, Value};
            use std::task::Poll;
            use tower_lsp::lsp_types::*;
            use wasm_language_server::test;

            #(#content)*
        }
    };
    module.to_token_stream().into()
}
