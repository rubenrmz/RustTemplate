// src/store/rbac_store.rs
use std::collections::HashMap;

use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::rbac::UserSystemRole;

/// Returns a map of `{ system_key: [role_key, ...] }` for the given user.
/// Only includes systems that are active.
pub async fn roles_by_user(pool: &PgPool, user_id: Uuid) -> Result<HashMap<String, Vec<String>>, AppError> {
    let rows = sqlx::query_as::<_, UserSystemRole>(
        r#"
        SELECT s.key AS system_key, r.key AS role_key
        FROM user_roles ur
        JOIN roles   r ON r.id = ur.role_id
        JOIN systems s ON s.id = r.system_id
        WHERE ur.user_id = $1
          AND s.active = TRUE
        ORDER BY s.key, r.key
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::Internal(e.into()))?;

    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for row in rows {
        map.entry(row.system_key).or_default().push(row.role_key);
    }

    Ok(map)
}

/// Assigns a role to a user. Idempotent (ON CONFLICT DO NOTHING).
pub async fn assign_role(pool: &PgPool, user_id: Uuid, system_key: &str, role_key: &str) -> Result<(), AppError> {
    sqlx::query(
        r#"
        INSERT INTO user_roles (user_id, role_id)
        SELECT $1, r.id
        FROM roles r
        JOIN systems s ON s.id = r.system_id
        WHERE s.key = $2 AND r.key = $3
        ON CONFLICT DO NOTHING
        "#,
    )
    .bind(user_id)
    .bind(system_key)
    .bind(role_key)
    .execute(pool)
    .await
    .map_err(|e| AppError::Internal(e.into()))?;

    Ok(())
}
