use lsp_types::Url;

#[derive(Clone, Debug)]
pub enum Message {
    DidOpenTree { uri: Url },
    DidChangeTree { uri: Url },
    DidCloseTree { uri: Url },
    Start,
}
