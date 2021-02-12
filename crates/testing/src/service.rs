use lspower::{ExitedError, LspService, MessageStream};
use serde_json::Value;
use tower_test::mock::Spawn;

pub async fn send(service: &mut Spawn<LspService>, request: &Value) -> Result<Option<Value>, ExitedError> {
    let request = serde_json::from_value(request.clone()).unwrap();
    let response = service.call(request).await?;
    let response = response.and_then(|x| serde_json::to_value(x).ok());
    Ok(response)
}

pub fn spawn() -> anyhow::Result<(Spawn<LspService>, MessageStream)> {
    let (service, messages) = LspService::new(|client| {
        let server = wasm_lsp_server::Server::new(client);
        server.unwrap()
    });
    Ok((Spawn::new(service), messages))
}

#[macro_export]
macro_rules! assert_status {
    ($service:expr, $status:expr) => {
        assert_eq!($service.poll_ready(), std::task::Poll::Ready($status));
    };
}

#[macro_export]
macro_rules! assert_exchange {
    ($service:expr, $request:expr, $response:expr) => {
        assert_eq!(testing::service::send($service, $request).await, $response);
    };
}
