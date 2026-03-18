// src/main.rs
use rust_template::create_app;
use tokio::net::TcpListener;
use tracing_subscriber::{
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_|
                    "rust_template=debug,tower_http=debug"
                        .into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = create_app().await;

    let addr = std::env::var("HOST")
        .unwrap_or_else(|_| "0.0.0.0:3000".into());

    let listener = TcpListener::bind(&addr).await?;

    tracing::info!("listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}