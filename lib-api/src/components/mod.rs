use crate::models::api_response::ApiResponse;
use axum::Router;
use lib_core::app_state::AppState;

pub mod auth;

pub type ApiResult = eyre::Result<ApiResponse, ApiResponse>;

pub fn routes(state: AppState) -> Router {
    Router::new().nest("/auth", auth::routes(state.clone()))
}
