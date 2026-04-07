// src/middleware/role.rs
use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
};

use crate::errors::AppError;
use crate::middleware::claims::Claims;

/// Rejects requests whose JWT `role` is not `"admin"`.
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

    if claims.role != "admin" {
        return Err(AppError::Forbidden);
    }

    Ok(next.run(req).await)
}

/// Rejects requests whose JWT `role` is not `"admin"` or `"user"`.
///
/// Useful for routes that any authenticated & active user can access
/// but you still want to exclude other future roles (e.g. `"guest"`).
pub async fn require_user_or_admin(req: Request, next: Next) -> Result<Response, AppError> {
    let claims = req.extensions().get::<Claims>().ok_or(AppError::Unauthorized)?;

    match claims.role.as_str() {
        "admin" | "user" => Ok(next.run(req).await),
        _ => Err(AppError::Forbidden),
    }
}
