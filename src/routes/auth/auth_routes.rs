// src/routes/auth_routes.rs
use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use validator::Validate;

use crate::dto::auth_dto::LoginDto;
use crate::errors::AppError;
use crate::services::auth_service;
use crate::state::AppState;


/// Ensambla el router de autenticación.
pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/auth/login", post(login_handler))
}


/// `POST /auth/login` — Autentica un usuario y devuelve un JWT.
#[tracing::instrument(skip(state, dto), fields(email = %dto.email))]
async fn login_handler(
    State(state): State<Arc<AppState>>,
    Json(dto): Json<LoginDto>,
) -> Result<impl IntoResponse, AppError> {
    dto.validate().map_err(AppError::Validation)?;
    let response = auth_service::login(&state, dto).await?;
    Ok((StatusCode::OK, Json(response)))
}