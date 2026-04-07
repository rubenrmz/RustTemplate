// src/routes/auth/recovery_routes.rs
use std::sync::Arc;

use axum::{extract::State, routing::post, Json, Router};
use validator::Validate;

use crate::dto::auth_dto::{MessageResponse, RequestResetDto, ResetPasswordDto, ValidateTokenDto};
use crate::errors::AppError;
use crate::services::auth::recovery_service;
use crate::state::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/auth/request-reset", post(request_reset_handler))
        .route("/auth/validate-token", post(validate_token_handler))
        .route("/auth/reset-password", post(reset_password_handler))
}

#[tracing::instrument(skip(state, dto), fields(email = %dto.email))]
async fn request_reset_handler(
    State(state): State<Arc<AppState>>,
    Json(dto): Json<RequestResetDto>,
) -> Result<Json<MessageResponse>, AppError> {
    dto.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    recovery_service::request_password_reset(&state, &dto.email).await?;

    // Siempre responder igual por seguridad
    Ok(Json(MessageResponse {
        message: "Si el email existe, recibirás un enlace de recuperación.",
    }))
}

async fn validate_token_handler(
    State(state): State<Arc<AppState>>,
    Json(dto): Json<ValidateTokenDto>,
) -> Result<Json<MessageResponse>, AppError> {
    dto.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    recovery_service::validate_reset_token(&state, &dto.token).await?;

    Ok(Json(MessageResponse {
        message: "Token válido.",
    }))
}

async fn reset_password_handler(
    State(state): State<Arc<AppState>>,
    Json(dto): Json<ResetPasswordDto>,
) -> Result<Json<MessageResponse>, AppError> {
    dto.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    recovery_service::reset_password(&state, dto).await?;

    Ok(Json(MessageResponse {
        message: "Contraseña actualizada correctamente.",
    }))
}
