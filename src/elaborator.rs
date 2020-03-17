use failure::Fallible;

/// Elaborates a given [`Tree`] into structured data to be cached in
/// [`Database`](crate::database::Database).
///
/// [`Tree`]: https://docs.rs/tree-sitter/latest/tree_sitter/struct.Tree.html
pub struct Elaborator;

impl Elaborator {
    pub fn new() -> Fallible<Self> {
        Ok(Elaborator)
    }
}
