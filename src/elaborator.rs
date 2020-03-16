use failure::Fallible;

pub struct Elaborator;

impl Elaborator {
    pub fn new() -> Fallible<Self> {
        Ok(Elaborator)
    }
}
