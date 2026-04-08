// src/state.rs
use std::sync::Arc;

use jsonwebtoken::{DecodingKey, EncodingKey};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

use crate::config::Config;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub db: PgPool,
    pub jwt_encoding_key: Arc<EncodingKey>,
    pub jwt_decoding_key: Arc<DecodingKey>,
    /// Raw public key PEM bytes — used to build the JWKS response.
    pub jwt_public_key_pem: Arc<Vec<u8>>,
}

impl AppState {
    /// Initialises application state including the database pool
    /// and RSA key pair for JWT signing/verification.
    ///
    /// Keys are loaded from AWS Secrets Manager when `AWS_JWT_PRIVATE_KEY_SECRET`
    /// and `AWS_JWT_PUBLIC_KEY_SECRET` are set, otherwise from local PEM files
    /// via `JWT_PRIVATE_KEY_PATH` and `JWT_PUBLIC_KEY_PATH`.
    ///
    /// # Panics
    ///
    /// Panics if required environment variables are missing,
    /// the database connection cannot be established, or the
    /// RSA keys cannot be loaded.
    pub async fn new() -> Self {
        let config = Arc::new(Config::from_env());

        let db = PgPoolOptions::new()
            .max_connections(config.database_max_connections)
            .acquire_timeout(std::time::Duration::from_secs(5))
            .idle_timeout(std::time::Duration::from_secs(600))
            .max_lifetime(std::time::Duration::from_secs(1800))
            .connect(&config.database_url)
            .await
            .expect("failed to connect to database");

        tracing::info!("database connection pool established");

        sqlx::migrate!("./migrations")
            .run(&db)
            .await
            .expect("failed to run database migrations");

        tracing::info!("database migrations applied");

        crate::store::seeder::seed_admin(&db, &config).await;

        let (private_pem, public_pem) = load_rsa_keys(&config).await;

        let jwt_encoding_key = EncodingKey::from_rsa_pem(&private_pem)
            .expect("invalid RSA private key PEM");
        let jwt_decoding_key = DecodingKey::from_rsa_pem(&public_pem)
            .expect("invalid RSA public key PEM");

        tracing::info!("RSA key pair loaded for JWT RS256");

        Self {
            config,
            db,
            jwt_encoding_key: Arc::new(jwt_encoding_key),
            jwt_decoding_key: Arc::new(jwt_decoding_key),
            jwt_public_key_pem: Arc::new(public_pem),
        }
    }
}

/// Loads RSA key pair from AWS Secrets Manager or local files.
async fn load_rsa_keys(config: &Config) -> (Vec<u8>, Vec<u8>) {
    if config.use_aws_secrets() {
        tracing::info!("loading RSA keys from AWS Secrets Manager");
        load_from_aws(config).await
    } else {
        tracing::info!("loading RSA keys from local files");
        load_from_files(config)
    }
}

fn load_from_files(config: &Config) -> (Vec<u8>, Vec<u8>) {
    let private_path = config
        .jwt_private_key_path
        .as_deref()
        .unwrap_or("private.pem");
    let public_path = config
        .jwt_public_key_path
        .as_deref()
        .unwrap_or("public.pem");

    let private_pem =
        std::fs::read(private_path).expect("failed to read JWT private key file");
    let public_pem =
        std::fs::read(public_path).expect("failed to read JWT public key file");

    (private_pem, public_pem)
}

async fn load_from_aws(config: &Config) -> (Vec<u8>, Vec<u8>) {
    let aws_config = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .region(aws_config::Region::new(config.aws_region.clone()))
        .load()
        .await;

    let client = aws_sdk_secretsmanager::Client::new(&aws_config);

    let private_secret_name = config
        .aws_jwt_private_key_secret
        .as_deref()
        .expect("AWS_JWT_PRIVATE_KEY_SECRET must be set");
    let public_secret_name = config
        .aws_jwt_public_key_secret
        .as_deref()
        .expect("AWS_JWT_PUBLIC_KEY_SECRET must be set");

    let private_pem = fetch_secret(&client, private_secret_name).await;
    let public_pem = fetch_secret(&client, public_secret_name).await;

    (private_pem, public_pem)
}

async fn fetch_secret(
    client: &aws_sdk_secretsmanager::Client,
    name: &str,
) -> Vec<u8> {
    let output = client
        .get_secret_value()
        .secret_id(name)
        .send()
        .await
        .unwrap_or_else(|e| panic!("failed to fetch secret '{name}' from AWS: {e}"));

    output
        .secret_string()
        .map(|s| s.as_bytes().to_vec())
        .or_else(|| output.secret_binary().map(|b| b.as_ref().to_vec()))
        .unwrap_or_else(|| panic!("secret '{name}' has no value"))
}
