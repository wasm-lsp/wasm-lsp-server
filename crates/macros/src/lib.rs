//! Macros for the WASM language server.

#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

use glob::glob;
use proc_macro::TokenStream;
use quote::quote;

mod corpus_tests {
    use syn::parse::{Parse, ParseStream};

    mod keyword {
        syn::custom_keyword!(corpus);
        syn::custom_keyword!(include);
        syn::custom_keyword!(exclude);
        syn::custom_keyword!(handler);
    }

    pub(crate) struct MacroInput {
        pub(crate) corpus: syn::Ident,
        pub(crate) include: String,
        pub(crate) exclude: Vec<String>,
        pub(crate) handler: syn::Expr,
    }

    impl Parse for MacroInput {
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

            Ok(MacroInput {
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
    let corpus_tests::MacroInput {
        corpus,
        include,
        exclude,
        handler,
    } = syn::parse_macro_input!(input as corpus_tests::MacroInput);
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

mod language {
    use std::convert::TryFrom;
    use syn::parse::{Parse, ParseStream};
    use wasm_lsp_languages::language;

    pub(crate) struct Language(pub(crate) language::Language);

    impl Parse for Language {
        fn parse(input: ParseStream) -> syn::parse::Result<Self> {
            let language = input.parse::<syn::LitStr>()?.value();
            let language = language::Language::try_from(language.as_str());
            let language = language.map_err(|_| input.error("invalid language identifier"))?;
            Ok(Language(language))
        }
    }
}

mod field_ids {
    use syn::parse::{Parse, ParseStream};

    mod keyword {
        syn::custom_keyword!(language);
        syn::custom_keyword!(fields);
    }

    pub(crate) struct Field {
        pub(crate) ident: syn::Ident,
        pub(crate) name: String,
    }

    impl Parse for Field {
        fn parse(input: ParseStream) -> syn::parse::Result<Self> {
            let content;
            syn::parenthesized!(content in input);
            let ident = content.parse()?;
            content.parse::<syn::Token![,]>()?;
            let name = content.parse::<syn::LitStr>()?.value();
            Ok(Field { ident, name })
        }
    }

    pub(crate) struct MacroInput {
        pub(crate) language: super::language::Language,
        pub(crate) fields: Vec<Field>,
    }

    impl Parse for MacroInput {
        fn parse(input: ParseStream) -> syn::parse::Result<Self> {
            input.parse::<keyword::language>()?;
            input.parse::<syn::Token![:]>()?;
            let language = input.parse()?;
            input.parse::<syn::Token![,]>()?;

            input.parse::<keyword::fields>()?;
            input.parse::<syn::Token![:]>()?;
            let fields = {
                let content;
                syn::bracketed!(content in input);
                content
                    .parse_terminated::<Field, syn::Token![,]>(|b| b.parse())?
                    .into_iter()
                    .collect()
            };
            input.parse::<syn::Token![,]>().ok();

            Ok(MacroInput { language, fields })
        }
    }
}

#[allow(missing_docs)]
#[proc_macro]
pub fn field_ids(input: TokenStream) -> TokenStream {
    use wasm_lsp_languages::language;

    let macro_input = syn::parse_macro_input!(input as field_ids::MacroInput);

    #[allow(unsafe_code)]
    let language = match macro_input.language.0 {
        language::Language::Wast => language::wast(),
        language::Language::Wat => language::wat(),
    };

    let mut content = vec![];

    for field in macro_input.fields {
        let ident = field.ident;
        let name = field.name.as_str();
        let value = language.field_id_for_name(name).unwrap();
        let item = quote! {
            pub const #ident: u16 = #value;
        };
        content.push(item);
    }

    let result = quote! {
        #(#content)*
    };

    result.into()
}

mod node_kind_ids {
    use syn::parse::{Parse, ParseStream};

    mod keyword {
        syn::custom_keyword!(language);
        syn::custom_keyword!(node_kinds);
    }

    pub(crate) struct NodeKind {
        pub(crate) ident: syn::Ident,
        pub(crate) kind: String,
        pub(crate) named: bool,
    }

    impl Parse for NodeKind {
        fn parse(input: ParseStream) -> syn::parse::Result<Self> {
            let content;
            syn::parenthesized!(content in input);
            let ident = content.parse()?;
            content.parse::<syn::Token![,]>()?;
            let kind = content.parse::<syn::LitStr>()?.value();
            content.parse::<syn::Token![,]>()?;
            let named = content.parse::<syn::LitBool>()?.value();
            Ok(NodeKind { ident, kind, named })
        }
    }

    pub(crate) struct MacroInput {
        pub(crate) language: super::language::Language,
        pub(crate) node_kinds: Vec<NodeKind>,
    }

    impl Parse for MacroInput {
        fn parse(input: ParseStream) -> syn::parse::Result<Self> {
            input.parse::<keyword::language>()?;
            input.parse::<syn::Token![:]>()?;
            let language = input.parse()?;
            input.parse::<syn::Token![,]>()?;

            input.parse::<keyword::node_kinds>()?;
            input.parse::<syn::Token![:]>()?;
            let node_kinds = {
                let content;
                syn::bracketed!(content in input);
                content
                    .parse_terminated::<NodeKind, syn::Token![,]>(|b| b.parse())?
                    .into_iter()
                    .collect()
            };
            input.parse::<syn::Token![,]>().ok();

            Ok(MacroInput { language, node_kinds })
        }
    }
}

#[allow(missing_docs)]
#[proc_macro]
pub fn node_kind_ids(input: TokenStream) -> TokenStream {
    use wasm_lsp_languages::language;

    let macro_input = syn::parse_macro_input!(input as node_kind_ids::MacroInput);

    #[allow(unsafe_code)]
    let language = match macro_input.language.0 {
        language::Language::Wast => language::wast(),
        language::Language::Wat => language::wat(),
    };

    let mut content = vec![];

    for node_kind in macro_input.node_kinds {
        let ident = node_kind.ident;
        let kind = node_kind.kind.as_str();
        let value = language.id_for_node_kind(kind, node_kind.named);
        let item = quote! {
            pub const #ident: u16 = #value;
        };
        content.push(item);
    }

    let result = quote! {
        #(#content)*
    };

    result.into()
}

mod syntax_utils {
    pub mod impls {
        use proc_macro2::{Ident, Span, TokenStream};
        use quote::quote;
        use syn::parse::{Parse, ParseStream};

        pub fn alphabet() -> impl Iterator<Item = String> {
            ('A' ..= 'Z').cycle().zip(0 ..).map(|(c, i)| {
                let suffix = i / 26;
                let suffix = if suffix > 0 {
                    (suffix - 1).to_string()
                } else {
                    "".to_string()
                };
                format!("{}{}", c, suffix)
            })
        }

        pub fn tuple_types_impl(depth: usize) -> impl Iterator<Item = Ident> {
            alphabet()
                .take(depth)
                .take(depth)
                .map(|x| Ident::new(x.as_str(), Span::call_site()))
        }

        fn tuple_types_for_inner(depth: usize) -> impl Iterator<Item = Ident> {
            alphabet()
                .take(depth)
                .take(depth)
                .map(|x| Ident::new(x.as_str(), Span::call_site()))
        }

        pub fn tuple_types_for(depth: usize) -> TokenStream {
            let tuple_types_for_inner = tuple_types_for_inner(depth);
            match depth {
                0 => {
                    quote! { () }
                },
                _ => {
                    quote! { (#(#tuple_types_for_inner),*,) }
                },
            }
        }

        pub fn tuple_types_where(depth: usize) -> impl Iterator<Item = TokenStream> {
            alphabet().take(depth).take(depth).map(|x| {
                let ident = Ident::new(x.as_str(), Span::call_site());
                quote! {
                    #ident: Fn(&mut Vis) -> Result<(), SyntaxErrors>
                }
            })
        }

        pub struct MacroInput {
            pub depth: usize,
        }

        impl Parse for MacroInput {
            fn parse(input: ParseStream) -> syn::parse::Result<Self> {
                let depth = input.parse::<syn::LitInt>()?.base10_parse()?;
                Ok(MacroInput { depth })
            }
        }
    }
}

#[allow(missing_docs)]
#[proc_macro]
pub fn impl_choice(input: TokenStream) -> TokenStream {
    let syntax_utils::impls::MacroInput { depth } = syn::parse_macro_input!(input as syntax_utils::impls::MacroInput);

    let tuple_types_impl = syntax_utils::impls::tuple_types_impl(depth);
    let tuple_types_for = syntax_utils::impls::tuple_types_for(depth);
    let tuple_types_where = syntax_utils::impls::tuple_types_where(depth);

    let choice_inner = match depth {
        0 => {
            quote! {
                Ok(())
            }
        },
        1 => {
            quote! {
                self.0(visitor)
            }
        },
        _ => {
            let cases = (0 .. depth).map(|n| {
                let i = syn::Index::from(n);
                quote! {
                    if let Err(mut errs) = restore(&self.#i)(visitor) {
                        errors.append(&mut errs);
                    } else {
                        return Ok(());
                    }
                }
            });
            quote! {
                let mut errors = SyntaxErrors::new();
                #(#cases)*
                Err(errors)
            }
        },
    };

    let result = quote! {
        impl<'tree, Ctx, Vis, #(#tuple_types_impl),*> Choice<'tree, Ctx, Vis> for #tuple_types_for
        where
            Ctx: Context<'tree> + 'tree,
            Vis: Visitor<'tree, Ctx> + ?Sized,
            #(#tuple_types_where),*
        {
            #[inline]
            fn choice(&self, visitor: &mut Vis) -> Result<(), SyntaxErrors> {
                #choice_inner
            }
        }
    };

    result.into()
}

#[allow(missing_docs)]
#[proc_macro]
pub fn impl_seq(input: TokenStream) -> TokenStream {
    let syntax_utils::impls::MacroInput { depth } = syn::parse_macro_input!(input as syntax_utils::impls::MacroInput);

    let tuple_types_impl = syntax_utils::impls::tuple_types_impl(depth);
    let tuple_types_for = syntax_utils::impls::tuple_types_for(depth);
    let tuple_types_where = syntax_utils::impls::tuple_types_where(depth);

    let seq_inner = match depth {
        0 => {
            quote! {}
        },
        _ => {
            let cases = (0 .. depth).map(|n| {
                let i = syn::Index::from(n);
                quote! {
                    self.#i(visitor)?;
                }
            });
            quote! {
                #(#cases)*
            }
        },
    };

    let result = quote! {
        impl<'tree, Ctx, Vis, #(#tuple_types_impl),*> Seq<'tree, Ctx, Vis> for #tuple_types_for
        where
            Ctx: Context<'tree> + 'tree,
            Vis: Visitor<'tree, Ctx> + ?Sized,
            #(#tuple_types_where),*
        {
            #[inline]
            fn seq(&self, visitor: &mut Vis) -> Result<(), SyntaxErrors> {
                #seq_inner
                Ok(())
            }
        }
    };

    result.into()
}
