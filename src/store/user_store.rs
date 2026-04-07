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

pub async fn list(
    pool: &PgPool,
    limit: i64,
    offset: i64,
    search: Option<&str>,
) -> Result<(Vec<User>, i64), AppError> {
    let (users, count) = match search {
        Some(q) => {
            let pattern = format!("%{}%", q);
            let users = sqlx::query_as::<_, User>(
                "SELECT * FROM users WHERE name ILIKE $1 OR email ILIKE $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3",
            )
            .bind(&pattern)
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await
            .map_err(|e| AppError::Internal(e.into()))?;

            let count = sqlx::query_scalar::<_, i64>(
                "SELECT COUNT(*) FROM users WHERE name ILIKE $1 OR email ILIKE $1",
            )
            .bind(&pattern)
            .fetch_one(pool)
            .await
            .map_err(|e| AppError::Internal(e.into()))?;

            (users, count)
        }
        None => {
            let users = sqlx::query_as::<_, User>(
                "SELECT * FROM users ORDER BY created_at DESC LIMIT $1 OFFSET $2",
            )
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await
            .map_err(|e| AppError::Internal(e.into()))?;

            let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM users")
                .fetch_one(pool)
                .await
                .map_err(|e| AppError::Internal(e.into()))?;

            (users, count)
        }
    };

    Ok((users, count))
}

pub async fn create_with_role(
    pool: &PgPool,
    name: &str,
    email: &str,
    password_hash: &str,
    role: &str,
) -> Result<User, AppError> {
    sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (id, email, name, password_hash, role, active, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, true, NOW(), NOW())
        RETURNING *
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(email)
    .bind(name)
    .bind(password_hash)
    .bind(role)
    .fetch_one(pool)
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(ref db_err) if db_err.is_unique_violation() => AppError::Conflict,
        _ => AppError::Internal(e.into()),
    })
}

pub async fn update(
    pool: &PgPool,
    user_id: Uuid,
    name: Option<&str>,
    email: Option<&str>,
    password_hash: Option<&str>,
    role: Option<&str>,
    active: Option<bool>,
) -> Result<User, AppError> {
    let user = find_by_id(pool, user_id)
        .await?
        .ok_or(AppError::NotFound)?;

    let name = name.unwrap_or(&user.name);
    let email = email.unwrap_or(&user.email);
    let password_hash = password_hash.unwrap_or(&user.password_hash);
    let role = role.unwrap_or(&user.role);
    let active = active.unwrap_or(user.active);

    sqlx::query_as::<_, User>(
        r#"
        UPDATE users
        SET name = $1, email = $2, password_hash = $3, role = $4, active = $5, updated_at = NOW()
        WHERE id = $6
        RETURNING *
        "#,
    )
    .bind(name)
    .bind(email)
    .bind(password_hash)
    .bind(role)
    .bind(active)
    .bind(user_id)
    .fetch_one(pool)
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(ref db_err) if db_err.is_unique_violation() => AppError::Conflict,
        _ => AppError::Internal(e.into()),
    })
}

pub async fn delete(pool: &PgPool, user_id: Uuid) -> Result<(), AppError> {
    let result = sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(user_id)
        .execute(pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    Ok(())
}
