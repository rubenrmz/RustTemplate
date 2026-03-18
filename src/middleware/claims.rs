// src/middleware/claims.rs
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::errors::AppError;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
}

impl Claims {
    pub fn new(user_id: &str, expiration_seconds: u64) -> Self {
        let now = jsonwebtoken::get_current_timestamp() as usize;

        Self {
            sub: user_id.to_string(),
            exp: now + expiration_seconds as usize,
            iat: now,
        }
    }

    pub fn encode(&self, secret: &str) -> Result<String, AppError> {
        encode(
            &Header::default(),
            self,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .map_err(|e| AppError::Internal(e.into()))
    }

    pub fn decode(token: &str, secret: &str) -> Result<Self, AppError> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )
        .map(|data| data.claims)
        .map_err(|_| AppError::Unauthorized)
    }
}