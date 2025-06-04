use lib_shared::{Res, instrument};
use tracing::error;

#[tokio::main]
#[instrument]
async fn main() -> Res {
    lib_shared::init();

    lib_api::run()
        .await
        .inspect_err(|e| error!(error = e.to_string(), "api crashed"))
}
