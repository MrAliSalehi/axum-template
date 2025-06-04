use axum::Json;
use axum::response::{IntoResponse, Response};
use http::StatusCode;
use serde_json::{Value, json};
use ts_rs::TS;

#[derive(TS)]
#[ts(export, export_to = "models/rest/")]
pub struct ApiResponse {
    pub message: Option<String>,
    #[ts(type = "Object | null")]
    pub data: Option<Value>,
    #[ts(type = "number")]
    pub status: StatusCode,
}

impl ApiResponse {
    pub(crate) fn data(data: Value) -> Self {
        Self {
            message: None,
            status: StatusCode::OK,
            data: Some(data),
        }
    }
    #[allow(unused)]
    pub(crate) fn ok<M: Into<String>>(message: M, data: Option<Value>) -> Self {
        Self {
            message: Some(message.into()),
            data,
            status: StatusCode::OK,
        }
    }
    #[allow(unused)]
    pub fn bad_request<M: Into<String>>(message: M) -> Self {
        Self {
            data: None,
            message: Some(message.into()),
            status: StatusCode::BAD_REQUEST,
        }
    }

    pub(crate) fn internal(msg: &str) -> Self {
        Self {
            message: Some(msg.into()),
            data: None,
            status: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn unauthorized(message: &str) -> Self {
        Self {
            data: None,
            message: Some(message.into()),
            status: StatusCode::UNAUTHORIZED,
        }
    }
    #[allow(unused)]
    pub fn conflict(message: &str) -> Self {
        Self {
            data: None,
            message: Some(message.into()),
            status: StatusCode::CONFLICT,
        }
    }
}

impl IntoResponse for ApiResponse {
    fn into_response(self) -> Response {
        (
            self.status,
            Json(json!({
                "message":self.message,
                "data":self.data,
            })),
        )
            .into_response()
    }
}

impl From<eyre::Result<ApiResponse, ApiResponse>> for ApiResponse {
    fn from(value: eyre::Result<ApiResponse, ApiResponse>) -> Self {
        value.unwrap_or_else(|f| f)
    }
}
