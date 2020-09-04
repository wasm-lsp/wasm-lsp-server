#![allow(non_snake_case)]
#![no_main]

#[cfg(feature = "fuzz")]
#[macro_use]
extern crate wasm_language_server_testing;

#[cfg(feature = "fuzz")]
#[doc(hidden)]
pub mod test {
    use futures::stream::StreamExt;
    use libfuzzer_sys::fuzz_target;
    use serde_json::Value;
    use std::task::Poll;
    use tokio::runtime::Runtime;
    use tower_lsp::lsp_types::*;
    use wasm_language_server_shared as shared;
    use wasm_language_server_testing::test;
    use wasm_smith::Module;

    fuzz_target!(|module: Module| {
        let future = async {
            let (mut service, mut messages) = test::service::spawn().unwrap();
            let service = &mut service;

            let wasm = module.to_bytes();
            let uri = Url::parse("inmemory:///test").unwrap();
            let language_id = "wasm.wast";
            let text = wabt::wasm2wat_with_features(wasm, {
                let mut features = wabt::Features::new();
                features.enable_all();
                features
            })
            .unwrap();

            println!("{}", text);

            assert_ready!(service, Ok(()));
            let request = &shared::lsp::initialize::request();
            let response = Some(shared::lsp::initialize::response());
            assert_exchange!(service, request, Ok(response));

            assert_ready!(service, Ok(()));
            let notification = &shared::lsp::text_document::did_open::notification(&uri, language_id, 1, text);
            let status = None::<Value>;
            assert_exchange!(service, notification, Ok(status));

            let message = messages.next().await.unwrap();
            let actual = serde_json::to_value(&message).unwrap();
            let expected = shared::lsp::text_document::publish_diagnostics::notification(&uri, &[]);
            assert_eq!(actual, expected);
        };
        let mut runtime = Runtime::new().unwrap();
        runtime.block_on(future);
    });
}
