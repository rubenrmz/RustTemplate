// src/middleware/claims.rs
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::errors::AppError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub iss: String,
    pub sub: String,
    pub role: String,
    pub exp: usize,
    pub iat: usize,
}

impl Claims {
    pub fn new(issuer: &str, user_id: &str, role: &str, expiration_seconds: u64) -> Self {
        let now = jsonwebtoken::get_current_timestamp() as usize;

        Self {
            iss: issuer.to_string(),
            sub: user_id.to_string(),
            role: role.to_string(),
            exp: now + expiration_seconds as usize,
            iat: now,
        }
    }

    pub fn encode(&self, key: &EncodingKey) -> Result<String, AppError> {
        let header = Header::new(Algorithm::RS256);
        encode(&header, self, key)
            .map_err(|e| AppError::Internal(e.into()))
    }

    pub fn decode(token: &str, key: &DecodingKey, issuer: &str) -> Result<Self, AppError> {
        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_issuer(&[issuer]);

        decode::<Claims>(token, key, &validation)
            .map(|data| data.claims)
            .map_err(|_| AppError::Unauthorized)
    }
}
