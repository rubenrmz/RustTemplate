// src/routes/mod.rs
use axum::{middleware, Router};
use std::sync::Arc;

use crate::{
    middleware::{auth::require_auth, role::require_admin},
    state::AppState,
};

pub mod auth;
pub mod health;
pub mod users;

pub fn router(state: Arc<AppState>) -> Router {
    let public_routes = Router::new()
        .merge(health::router())
        .merge(auth::jwks::router())
        .merge(auth::auth_routes::router())
        .merge(auth::recovery_routes::router());

    let protected_routes = Router::new()
        .merge(health::protected_router())
        .merge(auth::auth_routes::protected_router())
        .layer(middleware::from_fn_with_state(
            state.clone(),
            require_auth,
        ));

    let admin_routes = Router::new()
        .merge(users::router())
        .layer(middleware::from_fn(require_admin))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            require_auth,
        ));

    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .merge(admin_routes)
        .with_state(state)
}
