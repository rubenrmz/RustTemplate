// src/config.rs
use std::env;

/// Central application configuration.
#[derive(Debug, Clone)]
pub struct Config {
    // App
    pub env: String,
    pub host: String,
    pub timezone: String,

    // Security
    pub secret_key: String,
    pub jwt_secret: String,
    pub jwt_expiration_seconds: u64,

    // Database
    pub database_url: String,
    pub database_max_connections: u32,

    // Redis
    pub redis_url: String,

    // Mail
    pub mail_host: String,
    pub mail_port: u16,
    pub mail_username: String,
    pub mail_password: String,
    pub mail_from: String,

    // Workers
    pub worker_concurrency: usize,

    // WebSockets
    pub ws_max_connections: usize,
}

impl Config {
    /// Loads configuration from environment variables.
    ///
    /// # Panics
    ///
    /// Panics if any required environment variable is missing or malformed.
    pub fn from_env() -> Self {
        Self {
            env: env::var("ENV").unwrap_or_else(|_| "development".into()),
            host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0:3000".into()),
            timezone: env::var("TIMEZONE").unwrap_or_else(|_| "UTC".into()),

            secret_key: env::var("SECRET_KEY").expect("SECRET_KEY must be set"),
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
            jwt_expiration_seconds: env::var("JWT_EXPIRATION_SECONDS")
                .unwrap_or_else(|_| "3600".into())
                .parse()
                .expect("JWT_EXPIRATION_SECONDS must be a number"),

            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            database_max_connections: env::var("DATABASE_MAX_CONNECTIONS")
                .unwrap_or_else(|_| "10".into())
                .parse()
                .expect("DATABASE_MAX_CONNECTIONS must be a number"),

            redis_url: env::var("REDIS_URL").expect("REDIS_URL must be set"),

            mail_host: env::var("MAIL_HOST").expect("MAIL_HOST must be set"),
            mail_port: env::var("MAIL_PORT")
                .unwrap_or_else(|_| "587".into())
                .parse()
                .expect("MAIL_PORT must be a number"),
            mail_username: env::var("MAIL_USERNAME").expect("MAIL_USERNAME must be set"),
            mail_password: env::var("MAIL_PASSWORD").expect("MAIL_PASSWORD must be set"),
            mail_from: env::var("MAIL_FROM").expect("MAIL_FROM must be set"),

            worker_concurrency: env::var("WORKER_CONCURRENCY")
                .unwrap_or_else(|_| "4".into())
                .parse()
                .expect("WORKER_CONCURRENCY must be a number"),

            ws_max_connections: env::var("WS_MAX_CONNECTIONS")
                .unwrap_or_else(|_| "1000".into())
                .parse()
                .expect("WS_MAX_CONNECTIONS must be a number"),
        }
    }

    pub fn is_production(&self) -> bool {
        self.env == "production"
    }

    pub fn is_development(&self) -> bool {
        self.env == "development"
    }
}