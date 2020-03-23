use lsp_types::Url;
use tower_lsp::Client;

#[derive(Clone, Debug)]
pub enum Message {
    TreeDidChange { client: Client, uri: Url },
    TreeDidClose { client: Client, uri: Url },
    TreeDidOpen { client: Client, uri: Url },
    Start,
}
