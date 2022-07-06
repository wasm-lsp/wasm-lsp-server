#[cfg(feature = "runtime-agnostic")]
pub use async_lock::{RwLock, RwLockReadGuard, RwLockWriteGuard};
#[cfg(feature = "runtime-tokio")]
pub use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
