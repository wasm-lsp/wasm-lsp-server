use lsp_types::Url;
use tower_lsp::Client;

/// Messages for synchronizing server component activity.
#[derive(Clone, Debug)]
pub enum Message {
    Start,
    TreeDidChange { client: Client, uri: Url },
    TreeDidClose { client: Client, uri: Url },
    TreeDidOpen { client: Client, uri: Url },
}
