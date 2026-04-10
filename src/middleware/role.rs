// src/middleware/role.rs
use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
};

use crate::errors::AppError;
use crate::middleware::claims::Claims;

/// Rejects requests unless the user has the `"admin"` role in **any** system.
///
/// Must be layered **after** `require_auth` so that `Claims`
/// are already present in the request extensions.
///
/// ```ignore
/// Router::new()
///     .merge(admin::router())
///     .layer(middleware::from_fn(require_admin))
///     .layer(middleware::from_fn_with_state(state.clone(), require_auth));
/// ```
pub async fn require_admin(req: Request, next: Next) -> Result<Response, AppError> {
    let claims = req.extensions().get::<Claims>().ok_or(AppError::Unauthorized)?;

    let is_admin = claims.roles.values().any(|roles| roles.iter().any(|r| r == "admin"));

    if !is_admin {
        return Err(AppError::Forbidden);
    }

    Ok(next.run(req).await)
}
