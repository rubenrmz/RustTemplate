// src/store/seeder.rs
use sqlx::PgPool;

use crate::config::Config;
use crate::store::{rbac_store, user_store};
use crate::utils::password;

/// Seeds the default admin user if `ADMIN_EMAIL` and `ADMIN_PASSWORD`
/// are set and no user with that email exists yet.
/// Also assigns the `admin` role in every active system.
///
/// Runs once at startup — idempotent.
pub async fn seed_admin(pool: &PgPool, config: &Config) {
    let (email, pwd, name) = match (&config.admin_email, &config.admin_password) {
        (Some(email), Some(pwd)) => {
            let name = config
                .admin_name
                .as_deref()
                .unwrap_or("Admin");
            (email.as_str(), pwd.as_str(), name)
        }
        _ => {
            tracing::debug!("ADMIN_EMAIL/ADMIN_PASSWORD not set, skipping admin seed");
            return;
        }
    };

    // Find or create the user
    let user = match user_store::find_by_email(pool, email).await {
        Ok(Some(u)) => {
            tracing::debug!("admin user '{email}' already exists, ensuring roles");
            u
        }
        Ok(None) => {
            let hash = match password::hash(pwd) {
                Ok(h) => h,
                Err(e) => {
                    tracing::error!("failed to hash admin password: {e}");
                    return;
                }
            };

            match user_store::create(pool, name, email, &hash).await {
                Ok(u) => {
                    tracing::info!("admin user '{email}' seeded successfully");
                    u
                }
                Err(e) => {
                    tracing::error!("failed to seed admin user: {e}");
                    return;
                }
            }
        }
        Err(e) => {
            tracing::error!("failed to check admin user: {e}");
            return;
        }
    };

    // Assign admin role in every active system
    let system_keys: Vec<String> = match sqlx::query_scalar::<_, String>(
        "SELECT key FROM systems WHERE active = TRUE",
    )
    .fetch_all(pool)
    .await
    {
        Ok(keys) => keys,
        Err(e) => {
            tracing::error!("failed to fetch systems for admin seed: {e}");
            return;
        }
    };

    for sys_key in &system_keys {
        if let Err(e) = rbac_store::assign_role(pool, user.id, sys_key, "admin").await {
            tracing::error!("failed to assign admin role in '{sys_key}': {e}");
        }
    }

    if !system_keys.is_empty() {
        tracing::info!(
            "admin roles assigned in systems: {}",
            system_keys.join(", ")
        );
    }
}
