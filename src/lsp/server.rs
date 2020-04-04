use crate::core::session::Session;
use failure::Fallible;
use std::sync::Arc;

pub(crate) struct Server {
    pub(crate) session: Arc<Session>,
}

impl Server {
    pub(crate) fn new() -> Fallible<Self> {
        let session = Arc::new(Session::new()?);
        Ok(Server { session })
    }
}
