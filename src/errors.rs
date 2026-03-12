// src/errors.rs

// ============================================================
// errors.rs — errores globales de la aplicación
// Centraliza todos los errores y su conversión a respuestas HTTP
// Equivalente a errorhandlers en Flask pero tipado y exhaustivo
// ============================================================

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

// Enum con todas las variantes de error posibles en la app
// #[derive(Error)] de thiserror genera automáticamente impl std::error::Error
#[derive(Debug, Error)]
pub enum AppError {
    // #[error("...")] define el mensaje que retorna .to_string()
    #[error("no encontrado")]
    NotFound,

    #[error("no autorizado")]
    Unauthorized,

    // {0} interpolará el String que se pase al crear el error
    #[error("error interno: {0}")]
    Internal(String),
}

// IntoResponse permite retornar AppError directamente desde los handlers
// Axum lo convierte automáticamente a una respuesta HTTP
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // Mapeamos cada variante a su status HTTP correspondiente
        let (status, message) = match &self {
            AppError::NotFound     => (StatusCode::NOT_FOUND, self.to_string()),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, self.to_string()),
            AppError::Internal(_)  => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };

        // Retornamos JSON consistente: { "error": "mensaje" }
        (status, Json(json!({ "error": message }))).into_response()
    }
}