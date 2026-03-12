// src/config.rs

// ============================================================
// config.rs — configuración de la aplicación
// Lee variables de entorno y las centraliza en un struct
// Equivalente a config.py en Flask
// ============================================================

#[derive(Debug, Clone)]
// Clone es necesario porque AppState se clona en cada request
pub struct Config {
    pub host: String,
    pub database_url: String,
    pub jwt_secret: String,
}

impl Config {
    // Construye Config leyendo el entorno
    // Se llama una sola vez al arrancar en AppState::new()
    pub fn from_env() -> Self {
        Self {
            // unwrap_or_else provee un valor por defecto si la variable no existe
            host: std::env::var("HOST")
                .unwrap_or_else(|_| "0.0.0.0:3000".into()),

            // expect() detiene la app si la variable es obligatoria y no existe
            // Mejor fallar al arrancar que fallar silencioso en runtime
            database_url: std::env::var("DATABASE_URL")
                .expect("DATABASE_URL requerido"),

            jwt_secret: std::env::var("JWT_SECRET")
                .expect("JWT_SECRET requerido"),
        }
    }
}