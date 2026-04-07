// src/routes/users.rs
use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Json, Router,
};
use uuid::Uuid;
use validator::Validate;

use crate::dto::user_dto::{CreateUserDto, UpdateUserDto, UserListParams};
use crate::errors::AppError;
use crate::services::user_service;
use crate::state::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/users", get(list_handler))
        .route("/users", post(create_handler))
        .route("/users/{id}", get(get_handler))
        .route("/users/{id}", put(update_handler))
        .route("/users/{id}", delete(delete_handler))
}

async fn list_handler(
    State(state): State<Arc<AppState>>,
    Query(params): Query<UserListParams>,
) -> Result<impl IntoResponse, AppError> {
    let response = user_service::list(&state, params).await?;
    Ok(Json(response))
}

async fn get_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let response = user_service::get_by_id(&state, id).await?;
    Ok(Json(response))
}

#[tracing::instrument(skip(state, dto), fields(email = %dto.email))]
async fn create_handler(
    State(state): State<Arc<AppState>>,
    Json(dto): Json<CreateUserDto>,
) -> Result<impl IntoResponse, AppError> {
    dto.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    let response = user_service::create(&state, dto).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

async fn update_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(dto): Json<UpdateUserDto>,
) -> Result<impl IntoResponse, AppError> {
    dto.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    let response = user_service::update(&state, id, dto).await?;
    Ok(Json(response))
}

async fn delete_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    user_service::delete(&state, id).await?;
    Ok(StatusCode::NO_CONTENT)
}
