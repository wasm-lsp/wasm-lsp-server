#![deny(clippy::all)]
#![deny(unsafe_code)]

use futures::stream::TryStreamExt;
use tower_lsp::{LspService, Server};
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::{stream::JsStream, JsFuture};

#[wasm_bindgen]
pub struct ServerConfig {
    into_server: js_sys::AsyncIterator,
    from_server: web_sys::WritableStream,
}

#[wasm_bindgen]
impl ServerConfig {
    #[wasm_bindgen(constructor)]
    pub fn new(into_server: js_sys::AsyncIterator, from_server: web_sys::WritableStream) -> Self {
        Self {
            into_server,
            from_server,
        }
    }
}

// NOTE: we don't use web_sys::ReadableStream for input here because on the
// browser side we need to use a ReadableByteStreamController to construct it
// and so far only Chromium-based browsers support that functionality.

// NOTE: input needs to be an AsyncIterator<Uint8Array, never, void> specifically
#[wasm_bindgen]
pub async fn serve(config: ServerConfig) -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let ServerConfig {
        into_server,
        from_server,
    } = config;

    JsFuture::from(web_tree_sitter_sys::Parser::init())
        .await
        .expect("failed to initialize tree-sitter");

    #[rustfmt::skip]
    #[cfg(target_arch = "wasm32")]
    let languages = wasm_lsp_server::core::SessionLanguages {
        wast: wasm_lsp_languages::language::wast().await.unwrap(),
        wat : wasm_lsp_languages::language::wat ().await.unwrap(),
    };

    #[rustfmt::skip]
    #[cfg(not(target_arch = "wasm32"))]
    let languages = wasm_lsp_server::core::SessionLanguages {
        wast: wasm_lsp_languages::language::wast(),
        wat : wasm_lsp_languages::language::wat (),
    };

    let stdin = JsStream::from(into_server);
    let stdin = stdin
        .map_ok(|value| {
            value
                .dyn_into::<js_sys::Uint8Array>()
                .expect("could not cast stream item to Uint8Array")
                .to_vec()
        })
        .map_err(|_err| std::io::Error::from(std::io::ErrorKind::Other))
        .into_async_read();

    let stdout = JsCast::unchecked_into::<wasm_streams::writable::sys::WritableStream>(from_server);
    let stdout = wasm_streams::WritableStream::from_raw(stdout);
    let stdout = stdout.try_into_async_write().map_err(|err| err.0)?;

    let (service, socket) = LspService::new(|client| wasm_lsp_server::Server::new(languages, client).unwrap());
    Server::new(stdin, stdout, socket).serve(service).await;

    Ok(())
}
