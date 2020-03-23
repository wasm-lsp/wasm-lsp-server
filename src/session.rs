use crate::{
    analyzer::Analyzer,
    auditor::Auditor,
    database::Database,
    elaborator::Elaborator,
    highlighter::Highlighter,
    synchronizer::Synchronizer,
    synthesizer::Synthesizer,
};
use failure::Fallible;
use std::sync::{Arc, Weak};
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct SessionHandle(Arc<RwLock<Weak<Session>>>);

impl SessionHandle {
    pub fn new(session: Arc<RwLock<Weak<Session>>>) -> Self {
        SessionHandle(session)
    }

    pub async fn get(&self) -> Arc<Session> {
        self.0.read().await.upgrade().expect("session not initialized")
    }
}

/// Represents the current state of the LSP service.
pub struct Session {
    pub analyzer: Arc<Analyzer>,
    pub auditor: Arc<Auditor>,
    pub database: Arc<Database>,
    pub elaborator: Arc<Elaborator>,
    pub highlighter: Arc<Highlighter>,
    pub synchronizer: Arc<Synchronizer>,
    pub synthesizer: Arc<Synthesizer>,
}

impl Session {
    pub fn new(
        analyzer: Arc<Analyzer>,
        auditor: Arc<Auditor>,
        database: Arc<Database>,
        elaborator: Arc<Elaborator>,
        highlighter: Arc<Highlighter>,
        synchronizer: Arc<Synchronizer>,
        synthesizer: Arc<Synthesizer>,
    ) -> Fallible<Self> {
        Ok(Session {
            analyzer,
            auditor,
            database,
            elaborator,
            highlighter,
            synchronizer,
            synthesizer,
        })
    }
}
