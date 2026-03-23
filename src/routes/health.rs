// src/routes/health.rs
use axum::{extract::State, routing::get, Json, Router};
use chrono::Utc;
use serde::Serialize;
use std::sync::Arc;

use crate::state::AppState;


#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    env: String,
    timestamp: String,
}


pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/health", get(health_check))
}


/// Returns the current health status of the service.
async fn health_check(State(state): State<Arc<AppState>>) -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        env: state.config.env.clone(),
        timestamp: Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string(),
    })
}