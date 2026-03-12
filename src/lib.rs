// src/lib.rs

// ============================================================
// lib.rs — raíz del crate como librería
// Declara todos los módulos y expone create_app()
// Equivalente al __init__.py raíz en Flask
// ============================================================

use axum::Router;

// Módulos privados — solo accesibles dentro del crate
// No necesitan ser pub porque nadie externo los usa directamente
mod config;
mod errors;
mod state;

// Módulos públicos — accesibles desde tests y código externo
pub mod dto;
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod repositories;
pub mod routes;
pub mod services;

// Importamos AppState para pasarlo al router
use state::AppState;

// Función principal que construye y retorna el Router configurado
// Es async porque AppState::new() puede conectar a la DB, etc.
pub async fn create_app() -> Router {
    // Inicializa el estado compartido (config, pool de DB, etc.)
    let state = AppState::new().await;

    // Construye el router con todas las rutas registradas
    routes::create_router(state)
}