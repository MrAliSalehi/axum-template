use derived::Gtor;
use moka::future::{Cache, CacheBuilder};
use std::ops::Deref;
use std::sync::Arc;
use std::time::Duration;

#[derive(Clone)]
pub struct CacheManager {
    inner: Arc<CacheManagerInner>,
}
#[derive(Gtor)]
pub struct CacheManagerInner {
    #[allow(unused)]
    some_cache: Cache<u32, u32>,
}

impl CacheManager {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(CacheManagerInner {
                some_cache: CacheBuilder::new(2000)
                    .time_to_live(Duration::from_secs(100))
                    .build(),
            }),
        }
    }
}
impl Default for CacheManager {
    fn default() -> Self {
        Self::new()
    }
}
impl Deref for CacheManager {
    type Target = CacheManagerInner;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
