use crate::models::api_response::ApiResponse;
use axum::{
    extract::{Request, State},
    middleware::Next,
    response::{IntoResponse, Response},
};
use http::header;
use lib_core::app_state::AppState;
use lib_core::services::auth_service::extract_claims;
use lib_core::services::auth_service::token_claims::TokenClaims;

pub async fn require_authentication(
    _s: State<AppState>,
    mut req: Request,
    next: Next,
) -> eyre::Result<Response, Response> {
    let claims = get_claims(&req).map_err(|e| e.into_response())?;
    //_s: probably some db call here to get some data about the user
    req.extensions_mut().insert(claims);
    Ok(next.run(req).await)
}

fn get_claims(req: &Request) -> eyre::Result<TokenClaims, ApiResponse> {
    req.headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok().map(|f| f.replace("Bearer ", "")))
        .ok_or_else(|| ApiResponse::unauthorized("authorization header is required"))
        .map(|token| extract_claims(token).ok_or(ApiResponse::unauthorized("invalid token")))?
}
