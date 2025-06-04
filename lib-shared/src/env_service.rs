use dotenvy::var;
use std::ops::Deref;
use std::sync::Arc;

#[derive(Clone)]
pub struct EnvService {
    inner: Arc<EnvServiceInner>,
}
pub struct EnvServiceInner {
    pub psql_url: String,
    pub api_port: usize,
}

impl EnvService {
    #[tracing::instrument]
    pub fn new() -> Self {
        Self {
            inner: Arc::new(EnvServiceInner {
                psql_url: var("DATABASE_URL").unwrap().to_owned(),
                api_port: var("PORT").unwrap().parse().unwrap(),
            }),
        }
    }
}

impl Default for EnvService {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for EnvService {
    type Target = EnvServiceInner;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
