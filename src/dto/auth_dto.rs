// src/dto/auth_dto.rs
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterDto {
    #[validate(length(min = 2, max = 100))]
    pub name: String,

    #[validate(email)]
    pub email: String,

    #[validate(length(min = 8, max = 128))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginDto {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 1))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RequestResetDto {
    #[validate(email)]
    pub email: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ValidateTokenDto {
    #[validate(length(min = 32, max = 64))]
    pub token: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ResetPasswordDto {
    #[validate(length(min = 32, max = 64))]
    pub token: String,

    #[validate(length(min = 8, max = 128))]
    pub password: String,

    #[validate(length(min = 8, max = 128))]
    pub password_confirm: String,
}

#[derive(Debug, Serialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: &'static str,
    pub expires_in: u64,
}

#[derive(Debug, Serialize)]
pub struct MeResponse {
    pub id: String,
    pub email: String,
    pub name: String,
    pub role: String,
}

#[derive(Debug, Serialize)]
pub struct MessageResponse {
    pub message: &'static str,
}
