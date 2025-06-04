use crate::services::auth_service::token_claims::TokenClaims;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use std::ops::Deref;
use std::sync::LazyLock;

pub mod token_claims;

//store it somewhere else
const SECRET: &str = "RdtcDAYypWNCrbvsDHZa1PaiZ727GAKr8WfVvDGtQj7nPAh9Bi1YjtTQL4IEMQ0E2r9uIGcsePMJM6hp1z8VNZ7k0k2qQHjKfxZQ";
static D: LazyLock<DecodingKey> = LazyLock::new(|| DecodingKey::from_secret(SECRET.as_bytes()));
static E: LazyLock<EncodingKey> = LazyLock::new(|| EncodingKey::from_secret(SECRET.as_bytes()));
static V: LazyLock<Validation> = LazyLock::new(Validation::default);

pub fn extract_claims(token: String) -> Option<TokenClaims> {
    let claims = decode::<TokenClaims>(&token, &D, V.deref()).ok()?;

    if claims.claims.expired() {
        return None;
    }
    Some(claims.claims)
}

pub fn create_jwt_token(sub: String) -> Option<String> {
    let now = chrono::Utc::now();
    let exp = (now + chrono::Duration::hours(10)).timestamp() as usize;
    let claims = TokenClaims {
        sub,
        exp,
        iat: now.timestamp() as usize,
    };
    Some(encode(&Header::default(), &claims, &E).unwrap_or_default())
}
