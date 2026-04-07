// src/store/user_store.rs
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::user::User;

pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<Option<User>, AppError> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
        .bind(email)
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))
}

pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<User>, AppError> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))
}

pub async fn find_by_reset_token(pool: &PgPool, token_hash: &str) -> Result<Option<User>, AppError> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE reset_token = $1")
        .bind(token_hash)
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))
}

pub async fn create(
    pool: &PgPool,
    name: &str,
    email: &str,
    password_hash: &str,
) -> Result<User, AppError> {
    sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (id, email, name, password_hash, role, active, created_at, updated_at)
        VALUES ($1, $2, $3, $4, 'user', true, NOW(), NOW())
        RETURNING *
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(email)
    .bind(name)
    .bind(password_hash)
    .fetch_one(pool)
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(ref db_err) if db_err.is_unique_violation() => AppError::Conflict,
        _ => AppError::Internal(e.into()),
    })
}

pub async fn set_reset_token(
    pool: &PgPool,
    user_id: Uuid,
    token_hash: &str,
    expires_at: DateTime<Utc>,
) -> Result<(), AppError> {
    sqlx::query("UPDATE users SET reset_token = $1, tk_expires_at = $2, updated_at = NOW() WHERE id = $3")
        .bind(token_hash)
        .bind(expires_at)
        .bind(user_id)
        .execute(pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;
    Ok(())
}

pub async fn clear_reset_token(pool: &PgPool, user_id: Uuid) -> Result<(), AppError> {
    sqlx::query("UPDATE users SET reset_token = NULL, tk_expires_at = NULL, updated_at = NOW() WHERE id = $1")
        .bind(user_id)
        .execute(pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;
    Ok(())
}

pub async fn update_password(
    pool: &PgPool,
    user_id: Uuid,
    password_hash: &str,
) -> Result<(), AppError> {
    sqlx::query("UPDATE users SET password_hash = $1, reset_token = NULL, tk_expires_at = NULL, updated_at = NOW() WHERE id = $2")
        .bind(password_hash)
        .bind(user_id)
        .execute(pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;
    Ok(())
}
