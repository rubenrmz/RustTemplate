// src/routes/auth/auth_routes.rs
use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Extension, Json, Router,
};
use validator::Validate;

use crate::dto::auth_dto::{LoginDto, MeResponse, RegisterDto};
use crate::errors::AppError;
use crate::middleware::claims::Claims;
use crate::services::auth::auth_service;
use crate::state::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/auth/register", post(register_handler))
        .route("/auth/login", post(login_handler))
}

pub fn protected_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/auth/refresh", post(refresh_handler))
        .route("/auth/me", post(me_handler))
}

#[tracing::instrument(skip(state, dto), fields(email = %dto.email))]
async fn register_handler(
    State(state): State<Arc<AppState>>,
    Json(dto): Json<RegisterDto>,
) -> Result<impl IntoResponse, AppError> {
    dto.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    let user = auth_service::register(&state, dto).await?;

    Ok((
        StatusCode::CREATED,
        Json(serde_json::json!({
            "id": user.id,
            "email": user.email,
            "name": user.name,
        })),
    ))
}

#[tracing::instrument(skip(state, dto), fields(email = %dto.email))]
async fn login_handler(
    State(state): State<Arc<AppState>>,
    Json(dto): Json<LoginDto>,
) -> Result<impl IntoResponse, AppError> {
    dto.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    let response = auth_service::login(&state, dto).await?;
    Ok((StatusCode::OK, Json(response)))
}

async fn refresh_handler(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
) -> Result<impl IntoResponse, AppError> {
    let response = auth_service::refresh(&state, &claims).await?;
    Ok((StatusCode::OK, Json(response)))
}

async fn me_handler(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<MeResponse>, AppError> {
    let user_id: uuid::Uuid = claims.sub.parse().map_err(|_| AppError::Unauthorized)?;

    let user = crate::store::user_store::find_by_id(&state.db, user_id)
        .await?
        .ok_or(AppError::NotFound)?;

    Ok(Json(MeResponse {
        id: user.id.to_string(),
        email: user.email,
        name: user.name,
        roles: claims.roles,
    }))
}
