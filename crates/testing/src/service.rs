use serde_json::Value;
use tower_lsp::{ClientSocket, LspService};
use tower_test::mock::Spawn;
use wasm_lsp_server::Server;

pub async fn send(
    service: &mut tower_test::mock::Spawn<LspService<Server>>,
    request: &serde_json::Value,
) -> Result<Option<Value>, tower_lsp::ExitedError> {
    let request = serde_json::from_value(request.clone()).unwrap();
    let response = service.call(request).await?;
    let response = response.and_then(|x| serde_json::to_value(x).ok());
    Ok(response)
}

pub fn spawn() -> anyhow::Result<(tower_test::mock::Spawn<LspService<Server>>, ClientSocket)> {
    let (service, socket) = LspService::new(|client| {
        #[rustfmt::skip]
        let languages = wasm_lsp_server::core::SessionLanguages {
            wast: wasm_lsp_languages::language::wast(),
            wat : wasm_lsp_languages::language::wat (),
        };
        let server = wasm_lsp_server::Server::new(languages, client);
        server.unwrap()
    });
    Ok((Spawn::new(service), socket))
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
