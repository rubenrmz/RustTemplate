// src/routes/health.rs
use axum::{extract::State, routing::get, Json, Router};
use chrono::Utc;
use serde::Serialize;
use std::sync::Arc;

use crate::errors::AppError;
use crate::state::AppState;

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    env: String,
    timestamp: String,
}

#[derive(Serialize)]
struct HealthDetailedResponse {
    status: &'static str,
    env: String,
    database: &'static str,
    timestamp: String,
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/health", get(health_check))
}

pub fn protected_router() -> Router<Arc<AppState>> {
    Router::new().route("/health/detail", get(health_detail))
}

async fn health_check(State(state): State<Arc<AppState>>) -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        env: state.config.env.clone(),
        timestamp: Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string(),
    })
}

async fn health_detail(
    State(state): State<Arc<AppState>>,
) -> Result<Json<HealthDetailedResponse>, AppError> {
    let db_status = sqlx::query_scalar::<_, i32>("SELECT 1")
        .fetch_one(&state.db)
        .await
        .map(|_| "ok")
        .unwrap_or("unreachable");

    let status = if db_status == "ok" { "ok" } else { "degraded" };

    let response = HealthDetailedResponse {
        status,
        env: state.config.env.clone(),
        database: db_status,
        timestamp: Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string(),
    };

    if status != "ok" {
        return Err(AppError::ServiceUnavailable);
    }

    Ok(Json(response))
}
