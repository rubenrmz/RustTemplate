// src/routes/mod.rs
use axum::{middleware, Router};
use std::sync::Arc;

use crate::{middleware::auth::require_auth, state::AppState};

pub mod health;
// pub mod users;

pub fn create_router(state: Arc<AppState>) -> Router {
    let public_routes = Router::new()
        .merge(health::router());
        // .merge(auth_routes::router());

    let protected_routes = Router::new()
        // .merge(users::router())
        .layer(middleware::from_fn_with_state(
            state.clone(),
            require_auth,
        ));

    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .with_state(state)
}