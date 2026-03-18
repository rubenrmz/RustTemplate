// src/middleware/auth.rs
use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use std::sync::Arc;

use crate::{errors::AppError, middleware::claims::Claims, state::AppState};


pub async fn require_auth(
    State(state): State<Arc<AppState>>,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let token = extract_bearer_token(&req)?;
    let claims = Claims::decode(token, &state.config.jwt_secret)?;

    req.extensions_mut().insert(claims);

    Ok(next.run(req).await)
}


fn extract_bearer_token(req: &Request) -> Result<&str, AppError> {
    let header = req
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .ok_or(AppError::Unauthorized)?;

    header
        .strip_prefix("Bearer ")
        .ok_or(AppError::Unauthorized)
}