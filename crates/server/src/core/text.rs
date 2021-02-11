use crate::core;
use std::convert::TryInto;

pub struct Text {
    pub language: core::Language,
    pub content: ropey::Rope,
}

impl Text {
    pub fn new(
        language_id: impl TryInto<core::Language, Error = anyhow::Error>,
        text: impl AsRef<str>,
    ) -> anyhow::Result<Self> {
        let text = text.as_ref();
        let language = language_id.try_into()?;
        let content = ropey::Rope::from_str(text);
        Ok(Text { language, content })
    }
}
