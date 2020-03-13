use failure::Fallible;

pub struct Session;

impl Session {
    pub fn new() -> Fallible<Session> {
        Ok(Session)
    }
}
