// src/routes/health.rs
use axum::{routing::get, Router};
use std::sync::Arc;
use crate::state::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/health", get(health_check))
}

async fn health_check() -> &'static str {
    "ok"
}