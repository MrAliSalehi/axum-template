use crate::UserAuthDriver;
use lib_shared::instrument;
use std::fmt::Debug;
extern crate tracing;

impl UserAuthDriver {
    #[instrument(skip(self))]
    pub async fn login(&self, _id: impl Into<String> + Debug, _pwd: &str) -> eyre::Result<bool> {
        // use sqlx to write plain query
        // query!("").execute(*self.connection.db()).await?;
        // or seaorm through *self.connection.orm()
        Ok(true)
    }
}
