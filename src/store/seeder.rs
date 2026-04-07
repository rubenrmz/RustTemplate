// src/store/seeder.rs
use sqlx::PgPool;

use crate::config::Config;
use crate::store::user_store;
use crate::utils::password;

/// Seeds the default admin user if `ADMIN_EMAIL` and `ADMIN_PASSWORD`
/// are set and no user with that email exists yet.
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

    // Check if already exists
    match user_store::find_by_email(pool, email).await {
        Ok(Some(_)) => {
            tracing::debug!("admin user '{email}' already exists, skipping seed");
            return;
        }
        Ok(None) => {}
        Err(e) => {
            tracing::error!("failed to check admin user: {e}");
            return;
        }
    }

    let hash = match password::hash(pwd) {
        Ok(h) => h,
        Err(e) => {
            tracing::error!("failed to hash admin password: {e}");
            return;
        }
    };

    match user_store::create_with_role(pool, name, email, &hash, "admin").await {
        Ok(_) => tracing::info!("admin user '{email}' seeded successfully"),
        Err(e) => tracing::error!("failed to seed admin user: {e}"),
    }
}
