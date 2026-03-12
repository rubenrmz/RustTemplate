// src/main.rs

// ============================================================
// main.rs — punto de entrada de la aplicación
// Inicializa logging, carga .env y arranca el servidor HTTP
// ============================================================

// Importamos la función que construye nuestra app (definida en lib.rs)
use rust_template::create_app;

// TcpListener es el socket que escucha conexiones entrantes
use tokio::net::TcpListener;

// Piezas para configurar el sistema de logging
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// #[tokio::main] convierte main() en async — arranca el runtime de tokio
#[tokio::main]
async fn main() {
    // Carga las variables del archivo .env al entorno del proceso
    // .ok() ignora el error si .env no existe (útil en producción)
    dotenvy::dotenv().ok();

    // Configura el sistema de logging
    // registry() es el punto central que conecta capas de logging
    tracing_subscriber::registry()
        // EnvFilter lee la variable RUST_LOG para filtrar niveles
        // Si RUST_LOG no existe, usa "rust_template=debug" por defecto
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "rust_template=debug".into()),
        ))
        // fmt::layer() formatea los logs para la terminal
        .with(tracing_subscriber::fmt::layer())
        // init() registra esta configuración globalmente
        .init();

    // Construye el Router con todas las rutas y middlewares
    let app = create_app().await;

    // Lee HOST desde .env o usa el valor por defecto
    let addr = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0:3000".into());

    // Abre el socket TCP en la dirección configurada
    let listener = TcpListener::bind(&addr).await.unwrap();

    tracing::info!("servidor escuchando en {}", addr);

    // Arranca el servidor — bloquea hasta que el proceso termine
    axum::serve(listener, app).await.unwrap();
}