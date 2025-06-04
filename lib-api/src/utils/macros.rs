#[macro_export]
macro_rules! data {
    ($i:expr) => {
        $crate::models::api_response::ApiResponse::data(serde_json::json!($i))
    };
}
#[macro_export]
macro_rules! internal {
    ($i:expr) => {
        $crate::models::api_response::ApiResponse::internal($i)
    };
}
#[macro_export]
macro_rules! data_or_internal {
    ($i:expr) => {
        $i.map(|f| data!(f))
            .map_err(|_| $crate::models::api_response::ApiResponse::internal("internal error"))
    };
}
#[macro_export]
macro_rules! get_or_return_err {
    ($i:expr) => {
        $i.inspect_err(|e| tracing::error!("api level error: {e:?}"))
            .map_err(|_| $crate::models::api_response::ApiResponse::internal("internal error"))?
    };
}
