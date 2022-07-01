//! Definitions related to working with textual content.

/// Convenience struct for packaging the language-id and textual content of a [`core::Document`].
pub struct Text {
    /// The language-id of the [`core::Document`].
    pub language: crate::core::Language,
    /// The textual content of the [`core::Document`].
    pub content: ropey::Rope,
}

impl Text {
    /// Create a new [`Text`] from a language-id and some textual content.
    pub fn new(
        language_id: impl TryInto<crate::core::Language, Error = anyhow::Error>,
        text: impl AsRef<str>,
    ) -> anyhow::Result<Self> {
        let text = text.as_ref();
        let language = language_id.try_into()?;
        let content = ropey::Rope::from_str(text);
        Ok(Text { language, content })
    }
}

impl From<crate::core::Document> for Text {
    fn from(value: crate::core::Document) -> Self {
        value.text()
    }
}
