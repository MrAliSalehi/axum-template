use axum::Json;
use axum::extract::Query;
use axum_valid::Validified;

pub mod api_response;
pub type ValidJson<T> = Validified<Json<T>>;
pub type ValidQuery<T> = Validified<Query<T>>;
