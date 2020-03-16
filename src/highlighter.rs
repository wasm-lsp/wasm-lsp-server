use failure::Fallible;

pub struct Highlighter;

impl Highlighter {
    pub fn new() -> Fallible<Self> {
        Ok(Highlighter)
    }
}
