//! Service-related definitions.

use lspower::{ExitedError, LspService, MessageStream};
use serde_json::Value;
use tower_test::mock::Spawn;

/// Send a `request` via the given `service` and return the response if possible.
pub async fn send(service: &mut Spawn<LspService>, request: &Value) -> Result<Option<Value>, ExitedError> {
    let request = serde_json::from_value(request.clone()).unwrap();
    let response = service.call(request).await?;
    let response = response.and_then(|x| serde_json::to_value(x).ok());
    Ok(response)
}

/// Spawn an LSP service and return it along with its message stream.
pub fn spawn() -> anyhow::Result<(Spawn<LspService>, MessageStream)> {
    let (service, messages) = LspService::new(|client| {
        let server = wasm_language_server::lsp::server::Server::new(client);
        server.unwrap()
    });
    Ok((Spawn::new(service), messages))
}

/// Convenience macro for service-related status assertions.
#[macro_export]
macro_rules! assert_status {
    ($service:expr, $status:expr) => {
        assert_eq!($service.poll_ready(), std::task::Poll::Ready($status));
    };
}

/// Convenience macro for service-related message exchange assertions.
#[macro_export]
macro_rules! assert_exchange {
    ($service:expr, $request:expr, $response:expr) => {
        assert_eq!(
            wasm_language_server_testing::service::send($service, $request).await,
            $response
        );
    };
}
