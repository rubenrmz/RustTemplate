// src/config.rs
use std::env;

/// Central application configuration.
#[derive(Debug, Clone)]
pub struct Config {
    // App
    pub env: String,
    pub timezone: String,
    pub frontend_url: String,

    // Security
    pub secret_key: String,
    pub jwt_issuer: String,
    pub jwt_expiration_seconds: u64,
    pub reset_token_expiration_hours: u64,

    // JWT keys — local file paths (development)
    pub jwt_private_key_path: Option<String>,
    pub jwt_public_key_path: Option<String>,

    // JWT keys — AWS Secrets Manager (production)
    pub aws_jwt_private_key_secret: Option<String>,
    pub aws_jwt_public_key_secret: Option<String>,
    pub aws_region: String,

    // Admin seed
    pub admin_email: Option<String>,
    pub admin_password: Option<String>,
    pub admin_name: Option<String>,

    // Database
    pub database_url: String,
    pub database_max_connections: u32,
    pub database_schema: String,

    // Redis
    pub redis_url: String,

    // Mail
    pub mail_host: String,
    pub mail_port: u16,
    pub mail_username: String,
    pub mail_password: String,
    pub mail_from: String,
    pub mail_from_name: String,

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
            timezone: env::var("TIMEZONE").unwrap_or_else(|_| "UTC".into()),
            frontend_url: env::var("FRONTEND_URL").expect("FRONTEND_URL must be set"),

            secret_key: env::var("SECRET_KEY").expect("SECRET_KEY must be set"),
            jwt_issuer: env::var("JWT_ISSUER").expect("JWT_ISSUER must be set"),
            jwt_expiration_seconds: env::var("JWT_EXPIRATION_SECONDS")
                .unwrap_or_else(|_| "3600".into())
                .parse()
                .expect("JWT_EXPIRATION_SECONDS must be a number"),
            reset_token_expiration_hours: env::var("RESET_TOKEN_EXPIRATION_HOURS")
                .unwrap_or_else(|_| "2".into())
                .parse()
                .expect("RESET_TOKEN_EXPIRATION_HOURS must be a number"),

            jwt_private_key_path: env::var("JWT_PRIVATE_KEY_PATH").ok(),
            jwt_public_key_path: env::var("JWT_PUBLIC_KEY_PATH").ok(),

            aws_jwt_private_key_secret: env::var("AWS_JWT_PRIVATE_KEY_SECRET").ok(),
            aws_jwt_public_key_secret: env::var("AWS_JWT_PUBLIC_KEY_SECRET").ok(),
            aws_region: env::var("AWS_REGION").unwrap_or_else(|_| "us-east-1".into()),

            admin_email: env::var("ADMIN_EMAIL").ok(),
            admin_password: env::var("ADMIN_PASSWORD").ok(),
            admin_name: env::var("ADMIN_NAME").ok(),

            database_url: Self::build_database_url(),
            database_max_connections: env::var("DATABASE_MAX_CONNECTIONS")
                .unwrap_or_else(|_| "10".into())
                .parse()
                .expect("DATABASE_MAX_CONNECTIONS must be a number"),
            database_schema: env::var("DATABASE_SCHEMA")
                .unwrap_or_else(|_| "public".into()),

            redis_url: Self::build_redis_url(),

            mail_host: env::var("MAIL_HOST").expect("MAIL_HOST must be set"),
            mail_port: env::var("MAIL_PORT")
                .unwrap_or_else(|_| "587".into())
                .parse()
                .expect("MAIL_PORT must be a number"),
            mail_username: env::var("MAIL_USERNAME").expect("MAIL_USERNAME must be set"),
            mail_password: env::var("MAIL_PASSWORD").expect("MAIL_PASSWORD must be set"),
            mail_from: env::var("MAIL_FROM").expect("MAIL_FROM must be set"),
            mail_from_name: env::var("MAIL_FROM_NAME")
                .unwrap_or_else(|_| "Rust Template".into()),

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

    fn build_database_url() -> String {
        let user = env::var("DATABASE_USER").expect("DATABASE_USER must be set");
        let password = env::var("DATABASE_PASSWORD").expect("DATABASE_PASSWORD must be set");
        let host = env::var("DATABASE_HOST").unwrap_or_else(|_| "localhost".into());
        let port = env::var("DATABASE_PORT").unwrap_or_else(|_| "5432".into());
        let name = env::var("DATABASE_NAME").expect("DATABASE_NAME must be set");
        let schema = env::var("DATABASE_SCHEMA").unwrap_or_else(|_| "public".into());

        format!(
            "postgres://{}:{}@{}:{}/{}?options=-c search_path%3D{}",
            user, password, host, port, name, schema
        )
    }

    fn build_redis_url() -> String {
        let host = env::var("REDIS_HOST").unwrap_or_else(|_| "localhost".into());
        let port = env::var("REDIS_PORT").unwrap_or_else(|_| "6379".into());
        let password = env::var("REDIS_PASSWORD").ok();
        let db = env::var("REDIS_DB").unwrap_or_else(|_| "0".into());

        match password {
            Some(pwd) => format!("redis://:{}@{}:{}/{}", pwd, host, port, db),
            None => format!("redis://{}:{}/{}", host, port, db),
        }
    }

    pub fn use_aws_secrets(&self) -> bool {
        self.aws_jwt_private_key_secret.is_some() && self.aws_jwt_public_key_secret.is_some()
    }

    pub fn is_production(&self) -> bool {
        self.env == "production"
    }

    pub fn is_development(&self) -> bool {
        self.env == "development"
    }
}
