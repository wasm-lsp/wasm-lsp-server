use lsp_types::Url;
use tower_lsp::Client;

#[derive(Clone, Debug)]
pub enum Message {
    DidChangeTree { client: Client, uri: Url },
    DidCloseTree { client: Client, uri: Url },
    DidOpenTree { client: Client, uri: Url },
    Start,
}
