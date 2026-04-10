// src/models/rbac.rs
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct System {
    pub id: Uuid,
    pub key: String,
    pub name: String,
    pub description: Option<String>,
    pub active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Role {
    pub id: Uuid,
    pub system_id: Uuid,
    pub key: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Flat row returned when joining user_roles → roles → systems.
#[derive(Debug, sqlx::FromRow)]
pub struct UserSystemRole {
    pub system_key: String,
    pub role_key: String,
}
