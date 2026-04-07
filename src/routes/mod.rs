// src/routes/mod.rs
use axum::{middleware, Router};
use std::sync::Arc;

use crate::{middleware::auth::require_auth, state::AppState};
// use crate::middleware::role::require_admin;

pub mod auth;
pub mod health;

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

    // Example: admin-only routes
    // let admin_routes = Router::new()
    //     .merge(admin::router())
    //     .layer(middleware::from_fn(require_admin))
    //     .layer(middleware::from_fn_with_state(
    //         state.clone(),
    //         require_auth,
    //     ));

    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        // .merge(admin_routes)
        .with_state(state)
}
