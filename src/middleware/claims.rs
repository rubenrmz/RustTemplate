// src/middleware/claims.rs
use std::collections::HashMap;

use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::errors::AppError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub iss: String,
    /// Systems the user has access to (derived from roles).
    pub aud: Vec<String>,
    pub sub: String,
    /// `{ "system_a": ["admin","editor"], "system_b": ["viewer"] }`
    pub roles: HashMap<String, Vec<String>>,
    pub exp: usize,
    pub iat: usize,
}

impl Claims {
    pub fn new(
        issuer: &str,
        audience: Vec<String>,
        user_id: &str,
        roles: HashMap<String, Vec<String>>,
        expiration_seconds: u64,
    ) -> Self {
        let now = jsonwebtoken::get_current_timestamp() as usize;

        Self {
            iss: issuer.to_string(),
            aud: audience,
            sub: user_id.to_string(),
            roles,
            exp: now + expiration_seconds as usize,
            iat: now,
        }
    }

    pub fn encode(&self, key: &EncodingKey) -> Result<String, AppError> {
        let header = Header::new(Algorithm::RS256);
        encode(&header, self, key)
            .map_err(|e| AppError::Internal(e.into()))
    }

    /// Decodes and validates a JWT.
    /// Used by the auth server itself — validates issuer + signature, no audience check.
    /// Microservices should add their own audience validation against their system key.
    pub fn decode(token: &str, key: &DecodingKey, issuer: &str) -> Result<Self, AppError> {
        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_issuer(&[issuer]);
        validation.validate_aud = false;

        decode::<Claims>(token, key, &validation)
            .map(|data| data.claims)
            .map_err(|_| AppError::Unauthorized)
    }

    /// Returns the roles for a given system, or an empty slice.
    pub fn roles_for(&self, system_key: &str) -> &[String] {
        self.roles.get(system_key).map_or(&[], |v| v.as_slice())
    }

    /// Checks if the user has a specific role in a specific system.
    pub fn has_role(&self, system_key: &str, role_key: &str) -> bool {
        self.roles_for(system_key).iter().any(|r| r == role_key)
    }

    /// Checks if the user has access to a specific system (has any role in it).
    pub fn has_system_access(&self, system_key: &str) -> bool {
        self.aud.iter().any(|s| s == system_key)
    }
}
