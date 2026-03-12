// src/state.rs

// ============================================================
// state.rs — estado compartido de la aplicación
// Se inyecta en cada handler via State<AppState>
// Equivalente a current_app / g en Flask pero explícito y tipado
// ============================================================

use crate::config::Config;

// Clone es obligatorio — Axum clona el estado en cada request
#[derive(Clone)]
pub struct AppState {
    pub config: Config,

    // Cuando agregues sqlx, el pool va aquí:
    // pub db: PgPool,
    //
    // PgPool ya implementa Clone internamente con Arc,
    // así que clonar AppState no copia la conexión, solo el puntero
}

impl AppState {
    // Constructor async — permite hacer await para conectar DB, etc.
    pub async fn new() -> Self {
        let config = Config::from_env();

        // Cuando tengas sqlx:
        // let db = PgPoolOptions::new()
        //     .max_connections(10)
        //     .connect(&config.database_url)
        //     .await
        //     .expect("error conectando a la DB");

        Self {
            config,
            // db,
        }
    }
}