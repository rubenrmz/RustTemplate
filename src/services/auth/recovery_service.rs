// src/services/recovery_service.rs
use chrono::{Duration, Utc};
use sha2::{Digest, Sha256};

use crate::dto::auth_dto::ResetPasswordDto;
use crate::errors::AppError;
use crate::state::AppState;
use crate::store::user_store;
use crate::utils::{mail, password};

pub async fn request_password_reset(state: &AppState, email: &str) -> Result<(), AppError> {
    let email = email.to_lowercase();
    let user = match user_store::find_by_email(&state.db, &email).await? {
        Some(u) => u,
        None => return Ok(()), // no revelar si el email existe
    };

    user_store::clear_reset_token(&state.db, user.id).await?;

    let token = generate_token();
    let token_hash = hash_token(&token);
    let expires_at =
        Utc::now() + Duration::hours(state.config.reset_token_expiration_hours as i64);

    user_store::set_reset_token(&state.db, user.id, &token_hash, expires_at).await?;

    // Enviar email en background para no bloquear la respuesta
    let reset_url = format!(
        "{}/auth/reset-password?token={}",
        state.config.frontend_url, token
    );
    let html = build_reset_email(
        &user.name,
        &reset_url,
        state.config.reset_token_expiration_hours,
    );
    let config = state.config.clone();
    let to = user.email.clone();

    tokio::spawn(async move {
        if let Err(e) = mail::send_html(&config, &to, "Recuperación de contraseña", &html).await {
            tracing::error!("failed to send reset email to {to}: {e}");
        }
    });

    Ok(())
}

pub async fn validate_reset_token(state: &AppState, token: &str) -> Result<(), AppError> {
    let token_hash = hash_token(token);

    let user = user_store::find_by_reset_token(&state.db, &token_hash)
        .await?
        .ok_or_else(|| AppError::Validation("Enlace inválido o ya utilizado.".into()))?;

    if user.is_token_expired() {
        return Err(AppError::Validation(
            "El enlace es inválido o ha expirado. Solicita uno nuevo.".into(),
        ));
    }

    Ok(())
}

pub async fn reset_password(state: &AppState, dto: ResetPasswordDto) -> Result<(), AppError> {
    if dto.password != dto.password_confirm {
        return Err(AppError::Validation("Las contraseñas no coinciden.".into()));
    }

    let token_hash = hash_token(&dto.token);

    let user = user_store::find_by_reset_token(&state.db, &token_hash)
        .await?
        .ok_or_else(|| AppError::Validation("Enlace inválido o ya utilizado.".into()))?;

    if user.is_token_expired() {
        return Err(AppError::Validation(
            "El enlace es inválido o ha expirado. Solicita uno nuevo.".into(),
        ));
    }

    let new_hash = password::hash(&dto.password)?;
    user_store::update_password(&state.db, user.id, &new_hash).await?;

    // Enviar email de confirmación en background
    let config = state.config.clone();
    let to = user.email.clone();
    let name = user.name.clone();

    tokio::spawn(async move {
        let html = build_confirmation_email(&name);
        if let Err(e) = mail::send_html(&config, &to, "Contraseña actualizada", &html).await {
            tracing::warn!("failed to send confirmation email to {to}: {e}");
        }
    });

    Ok(())
}

// ── Helpers ──────────────────────────────────────────────────────────

fn generate_token() -> String {
    use rand::Rng;
    let bytes: [u8; 32] = rand::rng().random();
    base64_url_encode(&bytes)
}

fn base64_url_encode(bytes: &[u8]) -> String {
    use base64::Engine;
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(bytes)
}

fn escape_html(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

fn hash_token(token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn build_reset_email(raw_name: &str, reset_url: &str, expiration_hours: u64) -> String {
    let name = escape_html(raw_name);
    format!(
        r#"<!DOCTYPE html>
<html>
<head><meta charset="UTF-8"></head>
<body style="font-family: Arial, sans-serif; line-height: 1.6; color: #333;">
<div style="max-width: 600px; margin: 0 auto; padding: 20px;">
    <h2 style="color: #2c3e50;">Recuperación de contraseña</h2>
    <p>Hola {name},</p>
    <p>Recibimos una solicitud para restablecer la contraseña de tu cuenta.</p>
    <p>Haz clic en el siguiente enlace para crear una nueva contraseña:</p>
    <p style="text-align: center; margin: 30px 0;">
        <a href="{reset_url}"
           style="background-color: #3498db; color: white; padding: 12px 30px;
                  text-decoration: none; border-radius: 5px; display: inline-block;">
            Restablecer contraseña
        </a>
    </p>
    <p>Este enlace expirará en {expiration_hours} horas.</p>
    <p>Si no solicitaste este cambio, puedes ignorar este mensaje.
       Tu contraseña permanecerá sin cambios.</p>
    <hr style="border: none; border-top: 1px solid #eee; margin: 30px 0;">
    <p style="font-size: 12px; color: #999;">
        Si el botón no funciona, copia y pega este enlace en tu navegador:<br>
        <a href="{reset_url}" style="color: #3498db;">{reset_url}</a>
    </p>
</div>
</body>
</html>"#
    )
}

fn build_confirmation_email(raw_name: &str) -> String {
    let name = escape_html(raw_name);
    format!(
        r#"<!DOCTYPE html>
<html>
<head><meta charset="UTF-8"></head>
<body style="font-family: Arial, sans-serif; line-height: 1.6; color: #333;">
<div style="max-width: 600px; margin: 0 auto; padding: 20px;">
    <h2 style="color: #2c3e50;">Contraseña actualizada</h2>
    <p>Hola {name},</p>
    <p>Tu contraseña ha sido actualizada exitosamente.</p>
    <p>Si no realizaste este cambio, contacta inmediatamente a nuestro
       equipo de soporte.</p>
    <hr style="border: none; border-top: 1px solid #eee; margin: 30px 0;">
    <p style="font-size: 12px; color: #999;">
        Este es un mensaje automático, por favor no respondas a este correo.
    </p>
</div>
</body>
</html>"#
    )
}
