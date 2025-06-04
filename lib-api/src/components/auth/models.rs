use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validify::{Payload, Validify};

#[derive(Deserialize, Validify, Payload, TS)]
#[ts(export, export_to = "models/auth/")]
pub struct LoginRequest {
    #[validate(length(min = 6, max = 200))]
    #[modify(trim)]
    pub identity: String,
    #[validate(length(min = 6, max = 200))]
    #[modify(trim)]
    pub pwd: String,
}

#[derive(Serialize, TS)]
#[ts(export, export_to = "models/auth/")]
#[serde(tag = "type", content = "value")]
pub enum LoginResponse {
    Success { token: String },
    InvalidCredentials,
}
