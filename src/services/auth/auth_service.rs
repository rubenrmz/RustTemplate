// src/services/auth/auth_service.rs
use crate::dto::auth_dto::{LoginDto, RegisterDto, TokenResponse};
use crate::errors::AppError;
use crate::middleware::claims::Claims;
use crate::models::user::User;
use crate::state::AppState;
use crate::store::user_store;
use crate::utils::password;

pub async fn register(state: &AppState, dto: RegisterDto) -> Result<User, AppError> {
    if !state.config.allow_registration {
        return Err(AppError::Forbidden);
    }

    let hash = password::hash(&dto.password)?;
    let email = dto.email.to_lowercase();
    user_store::create(&state.db, &dto.name, &email, &hash).await
}

pub async fn login(state: &AppState, dto: LoginDto) -> Result<TokenResponse, AppError> {
    let email = dto.email.to_lowercase();
    let user = user_store::find_by_email(&state.db, &email)
        .await?
        .filter(|u| u.active)
        .ok_or(AppError::Unauthorized)?;

    let valid = password::verify(&dto.password, &user.password_hash)?;
    if !valid {
        return Err(AppError::Unauthorized);
    }

    issue_token(state, &user)
}

pub async fn refresh(state: &AppState, claims: &Claims) -> Result<TokenResponse, AppError> {
    let user_id: uuid::Uuid = claims.sub.parse().map_err(|_| AppError::Unauthorized)?;

    let user = user_store::find_by_id(&state.db, user_id)
        .await?
        .filter(|u| u.active)
        .ok_or(AppError::Unauthorized)?;

    issue_token(state, &user)
}

pub fn issue_token(state: &AppState, user: &User) -> Result<TokenResponse, AppError> {
    let claims = Claims::new(
        &state.config.jwt_issuer,
        &user.id.to_string(),
        &user.role,
        state.config.jwt_expiration_seconds,
    );
    let token = claims.encode(&state.jwt_encoding_key)?;

    Ok(TokenResponse {
        access_token: token,
        token_type: "Bearer",
        expires_in: state.config.jwt_expiration_seconds,
    })
}
