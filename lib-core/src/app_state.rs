use crate::managers::cache_manager::CacheManager;
use crate::managers::thread_manager::ThreadManager;
use lib_db::PsqlDriver;
use lib_shared::env_service::EnvService;
use lib_shared::metrics::Metrics;
use std::ops::Deref;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    inner: Arc<AppStateInner>,
}

pub struct AppStateInner {
    pub thread_manager: ThreadManager,
    pub env: EnvService,
    pub metrics: Metrics,
    pub psql: PsqlDriver,
    pub cache_manager: CacheManager,
}

impl AppState {
    pub async fn new() -> Self {
        let cache_manager = CacheManager::new();
        let env = EnvService::new();
        let metrics = Metrics::new();

        let thread_manager = ThreadManager::new();
        let psql = PsqlDriver::new(&env.psql_url, metrics.clone()).await;
        let handle = psql.connection.run_metric_provider();
        thread_manager.add("db-metric", handle).await;

        let metrics_handle = metrics.run_generic_metric_provider();
        thread_manager.add("generic-metric", metrics_handle).await;

        Self {
            inner: Arc::new(AppStateInner {
                cache_manager,
                psql,
                metrics,
                thread_manager,
                env,
            }),
        }
    }
}

impl Deref for AppState {
    type Target = AppStateInner;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
