//! Collects diagnostics for documents with syntax and validation errors.

/// Functions related to processing parse tree events for a document.
pub(crate) mod tree {
    use crate::core::{
        document::Document,
        error::{Error, Fallible},
        language::{wast, wat, Language},
        session::Session,
    };
    use std::sync::Arc;
    use tower_lsp::lsp_types::*;

    // Compute diagnostics for a change event given a `document` and `tree`.
    // NOTE: This function was split out from `change` below in order to avoid
    // the need to create an explicit scope around the `node` variable since it
    // must be dropped before the use of `.await`. See PR #61 for details.
    fn diagnostics_for_change(document: &Document, tree: tree_sitter::Tree) -> Fallible<Vec<Diagnostic>> {
        let mut diagnostics = vec![];
        let node = tree.root_node();

        if node.has_error() {
            // prepare a query to match tree-sitter ERROR nodes
            let language = tree.language();
            let source = "((ERROR) @error)"; // query the tree for ERROR nodes
            let query = tree_sitter::Query::new(language, source).map_err(Error::TreeSitterQueryError)?;

            // prepare a query cursor
            let mut query_cursor = tree_sitter::QueryCursor::new();
            let text_callback = |node: tree_sitter::Node| &document.text[node.byte_range()];
            let matches = query_cursor.matches(&query, node, text_callback);

            // iterate the query cursor and construct appropriate lsp diagnostics
            for tree_sitter::QueryMatch { captures, .. } in matches {
                'captures: for tree_sitter::QueryCapture { node, .. } in captures {
                    // create a cursor node starting from the capture node
                    let mut cursor = *node;

                    // traverse upward through the parent nodes
                    'cursor: while let Some(parent) = cursor.parent() {
                        cursor = parent;
                        // ignore further processing if the first non-ERROR
                        // parent node is a comment node; we do this in
                        // order to avoid syntax errors due to encoding
                        // issues (see "comments.wast" and issue #42)
                        if !cursor.is_error() {
                            match document.language {
                                Language::Wast if wast::kind::is_comment(&parent.kind_id()) => {
                                    break 'captures;
                                }
                                Language::Wat if wat::kind::is_comment(&parent.kind_id()) => {
                                    break 'captures;
                                }
                                _ => {
                                    break 'cursor;
                                }
                            }
                        }
                    }

                    let start = node.start_position();
                    let end = node.end_position();
                    diagnostics.push({
                        let range = {
                            let start = Position::new(start.row as u64, start.column as u64);
                            let end = Position::new(end.row as u64, end.column as u64);
                            Range::new(start, end)
                        };
                        let severity = Some(DiagnosticSeverity::Error);
                        let code = Default::default();
                        let source = Some(String::from("wasm-lsp"));
                        let message = String::from("syntax error");
                        let related_information = Default::default();
                        let tags = Default::default();
                        Diagnostic::new(range, severity, code, source, message, related_information, tags)
                    });
                }
            }
        } else {
            // NOTE: else let elaborator handle
        }

        Ok(diagnostics)
    }

    /// Handle a parse tree "change" event.
    pub(crate) async fn change(session: Arc<Session>, uri: Url) -> Fallible<()> {
        if let Some(document) = session.get_document(&uri).await? {
            let tree = document.tree.lock().await.clone();
            let diagnostics = diagnostics_for_change(&document, tree)?;
            let version = Default::default();
            session.client.publish_diagnostics(uri, diagnostics, version).await;
        }
        Ok(())
    }

    /// Handle a parse tree "close" event.
    pub(crate) async fn close(session: Arc<Session>, uri: Url) -> Fallible<()> {
        // clear diagnostics on tree close
        // FIXME: handle this properly
        let diagnostics = Default::default();
        let version = Default::default();
        session.client.publish_diagnostics(uri, diagnostics, version).await;
        Ok(())
    }

    /// Handle a parse tree "open" event.
    pub(crate) async fn open(session: Arc<Session>, uri: Url) -> Fallible<()> {
        self::change(session, uri).await
    }
}
