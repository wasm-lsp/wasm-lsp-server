#![allow(non_snake_case)]
#![no_main]

use futures::stream::StreamExt;
use libfuzzer_sys::fuzz_target;
use serde_json::Value;
use tokio::runtime::Runtime;
use wasm_language_server_testing as testing;
use wasm_smith::Module;

fuzz_target!(|module: Module| {
    let future = async {
        let (mut service, mut messages) = testing::service::spawn().unwrap();
        let service = &mut service;

        let wasm = module.to_bytes();
        let uri = lsp::Url::parse("inmemory:///test").unwrap();
        let language_id = "wasm.wast";
        let text = wasmprinter::print_bytes(wasm).unwrap();

        println!("{}", text);

        testing::assert_status!(service, Ok(()));
        let request = &testing::lsp::initialize::request();
        let response = Some(testing::lsp::initialize::response());
        testing::assert_exchange!(service, request, Ok(response));

        testing::assert_status!(service, Ok(()));
        let notification = &testing::lsp::text_document::did_open::notification(&uri, language_id, 1, text);
        let status = None::<Value>;
        testing::assert_exchange!(service, notification, Ok(status));

        let message = messages.next().await.unwrap();
        let actual = serde_json::to_value(&message).unwrap();
        let expected = testing::lsp::text_document::publish_diagnostics::notification(&uri, &[]);
        assert_eq!(actual, expected);
    };
    let runtime = Runtime::new().unwrap();
    runtime.block_on(future);
});
