//! The WebAssembly language server.

#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

#[cfg(feature = "test")]
#[doc(hidden)]
pub mod test {
    #[macro_export]
    #[doc(hidden)]
    macro_rules! assert_ready {
        ($service:expr, $status:expr) => {
            assert_eq!($service.poll_ready(), Poll::Ready($status));
        };
    }

    #[macro_export]
    #[doc(hidden)]
    macro_rules! assert_exchange {
        ($service:expr, $request:expr, $response:expr) => {
            assert_eq!(test::service::call($service, $request).await, $response);
        };
    }

    pub mod service {
        use serde_json::Value;
        use tower_lsp::{ExitedError, LspService, MessageStream};
        use tower_test::mock::Spawn;

        pub async fn call(service: &mut Spawn<LspService>, request: &Value) -> Result<Option<Value>, ExitedError> {
            let request = serde_json::from_value(request.clone()).unwrap();
            let response = service.call(request).await?;
            let response = response.and_then(|x| serde_json::to_value(x).ok());
            Ok(response)
        }

        pub fn spawn() -> anyhow::Result<(Spawn<LspService>, MessageStream)> {
            let (service, messages) = LspService::new(|client| {
                let server = wasm_language_server::lsp::server::Server::new(client);
                server.unwrap()
            });
            Ok((Spawn::new(service), messages))
        }
    }
}
