use lib_shared::{Res, instrument, warn};
use parking_lot::RwLock;
use std::sync::Arc;
use tokio::task::JoinHandle;
use tracing::info;

#[derive(Clone, Default)]
pub struct ThreadManager {
    inner: Arc<RwLock<InnerMut>>,
}

#[derive(Default)]
struct InnerMut {
    handles: hashbrown::HashMap<String, JoinHandle<Res>>,
}

impl ThreadManager {
    pub fn new() -> Self {
        Self::default()
    }
    #[instrument(skip(self, handle))]
    pub async fn add(&self, name: &str, handle: JoinHandle<Res>) {
        let read = self.inner.read();

        if read.handles.contains_key(name) {
            warn!(name, "attempting to insert an already running task");
            return;
        }

        drop(read);
        self.inner.write().handles.insert(name.to_owned(), handle);
        info!("started tracking task");
    }
}

impl Drop for InnerMut {
    #[instrument(skip(self))]
    fn drop(&mut self) {
        self.handles.iter().for_each(|(name, h)| {
            h.abort();
            info!(name, "task closed");
        })
    }
}
