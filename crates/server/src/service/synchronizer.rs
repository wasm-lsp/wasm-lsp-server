pub(crate) mod document {
    use crate::{core, core::RopeExt};
    use ropey::Rope;
    use std::{convert::TryFrom, sync::Arc};

    pub(crate) async fn change(
        _session: Arc<core::Session>,
        _params: lsp::DidChangeTextDocumentParams,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    pub(crate) async fn close(
        _session: Arc<core::Session>,
        _params: lsp::DidCloseTextDocumentParams,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    pub(crate) async fn open(
        session: Arc<core::Session>,
        params: lsp::DidOpenTextDocumentParams,
    ) -> anyhow::Result<()> {
        let language = core::language::Language::try_from(params.text_document.language_id.as_str())?;
        let mut parser = tree_sitter::Parser::try_from(language)?;

        let content = Rope::from(params.text_document.text);

        let tree = {
            let content = content.clone();
            let callback = content.chunk_walker(0).callback_adapter();
            let old_tree = None;
            parser.parse_with(callback, old_tree)?
        };

        if let Some(tree) = tree {
            let document = core::Document { language, content };
            log::info!("{:#?}", tree.root_node().to_sexp());
            session.insert_document(params.text_document.uri, document, parser, tree)?;
        }

        Ok(())
    }
}
