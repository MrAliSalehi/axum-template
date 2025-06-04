use crate::components::ApiResult;
use crate::components::auth::models::{LoginRequest, LoginResponse};
use crate::middlewares::auth::require_authentication;
use crate::models::ValidJson;
use crate::models::api_response::ApiResponse;
use crate::{data, get_or_return_err, internal};
use axum::extract::State;
use axum::middleware::from_fn_with_state;
use axum::routing::{get, post};
use axum::{Extension, Router};
use lib_core::app_state::AppState;
use lib_core::services::auth_service::create_jwt_token;
use lib_core::services::auth_service::token_claims::TokenClaims;

mod models;
pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/info", get(info))
        .layer(from_fn_with_state(state.clone(), require_authentication))
        .route("/login", post(login))
        .with_state(state.clone())
}
async fn login(s: State<AppState>, r: ValidJson<LoginRequest>) -> ApiResult {
    let is_valid = get_or_return_err!(
        s.psql
            .user_auth_driver
            .login(&r.0.0.identity, &r.0.0.pwd)
            .await
    );

    if is_valid {
        return Ok(data!(LoginResponse::InvalidCredentials));
    }

    let token = create_jwt_token(r.0.0.identity).ok_or(internal!("[X] internal"))?;
    Ok(data!(LoginResponse::Success { token }))
}
async fn info(user: Extension<TokenClaims>) -> ApiResponse {
    data!(user.0)
}
