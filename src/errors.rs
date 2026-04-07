// src/errors.rs
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("not found")]
    NotFound,

    #[error("unauthorized")]
    Unauthorized,

    #[error("forbidden")]
    Forbidden,

    #[error("conflict")]
    Conflict,

    #[error("validation error")]
    Validation(String),

    #[error("service unavailable")]
    ServiceUnavailable,

    #[error("internal server error")]
    Internal(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match &self {
            AppError::NotFound =>
                StatusCode::NOT_FOUND,
            AppError::Unauthorized =>
                StatusCode::UNAUTHORIZED,
            AppError::Forbidden =>
                StatusCode::FORBIDDEN,
            AppError::Conflict =>
                StatusCode::CONFLICT,
            AppError::Validation(_) =>
                StatusCode::BAD_REQUEST,
            AppError::ServiceUnavailable =>
                StatusCode::SERVICE_UNAVAILABLE,
            AppError::Internal(e) => {
                tracing::error!("internal error: {e:?}");
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };

        let body = match &self {
            AppError::Validation(msg) =>
                json!({ "error": msg }),
            AppError::Internal(_) =>
                json!({ "error": "internal server error" }),
            _ => json!({ "error": self.to_string() }),
        };

        (status, Json(body)).into_response()
    }
}