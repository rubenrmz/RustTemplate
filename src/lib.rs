// src/lib.rs
use std::sync::Arc;

use axum::Router;

mod config;
mod errors;
mod state;

pub mod dto;
pub mod middleware;
pub mod models;
pub mod routes;
pub mod services;
pub mod store;
pub mod utils;

use state::AppState;

/// Builds and returns the configured application router.
pub async fn create_app() -> Router {
    let state = Arc::new(AppState::new().await);
    routes::router(state)
}