use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct TokenClaims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
}

impl TokenClaims {
    pub fn expired(&self) -> bool {
        let current_time = Utc::now().timestamp();
        self.exp < current_time as _
    }
}
