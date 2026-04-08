// src/lib.rs
use std::sync::Arc;

use axum::http::{header, HeaderValue, Method};
use axum::Router;
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};
use tower_http::cors::{AllowOrigin, CorsLayer};
use tower_http::limit::RequestBodyLimitLayer;
use tower_http::set_header::SetResponseHeaderLayer;
use tower_http::trace::TraceLayer;

mod config;
pub mod errors;
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

    let cors = if state.config.is_production() {
        let origins = state.config.cors_allowed_origins.clone();
        CorsLayer::new()
            .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
            .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE])
            .allow_credentials(true)
            .allow_origin(AllowOrigin::predicate(move |origin, _| {
                origins.iter().any(|o| o.as_bytes() == origin.as_bytes())
            }))
    } else {
        CorsLayer::permissive()
    };

    let governor_conf = GovernorConfigBuilder::default()
        .per_second(2)
        .burst_size(30)
        .finish()
        .expect("failed to build governor config");

    let app = routes::router(state);

    app.layer(TraceLayer::new_for_http())
        .layer(cors)
        .layer(GovernorLayer {
            config: governor_conf.into(),
        })
        .layer(RequestBodyLimitLayer::new(2 * 1024 * 1024)) // 2 MB
        .layer(SetResponseHeaderLayer::overriding(
            header::X_CONTENT_TYPE_OPTIONS,
            HeaderValue::from_static("nosniff"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            header::X_FRAME_OPTIONS,
            HeaderValue::from_static("DENY"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            header::STRICT_TRANSPORT_SECURITY,
            HeaderValue::from_static("max-age=63072000; includeSubDomains; preload"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            header::REFERRER_POLICY,
            HeaderValue::from_static("strict-origin-when-cross-origin"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            header::CONTENT_SECURITY_POLICY,
            HeaderValue::from_static("default-src 'none'; frame-ancestors 'none'"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            header::HeaderName::from_static("x-xss-protection"),
            HeaderValue::from_static("1; mode=block"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            header::HeaderName::from_static("permissions-policy"),
            HeaderValue::from_static("camera=(), microphone=(), geolocation=()"),
        ))
}
