// src/utils/mail.rs
use lettre::{
    message::{header::ContentType, Mailbox},
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};

use crate::config::Config;
use crate::errors::AppError;

pub async fn send_html(
    config: &Config,
    to: &str,
    subject: &str,
    html_body: &str,
) -> Result<(), AppError> {
    let from: Mailbox = format!("{} <{}>", config.mail_from_name, config.mail_from)
        .parse()
        .map_err(|e| AppError::Internal(anyhow::anyhow!("invalid from address: {e}")))?;

    let to_mailbox: Mailbox = to
        .parse()
        .map_err(|e| AppError::Internal(anyhow::anyhow!("invalid to address: {e}")))?;

    let email = Message::builder()
        .from(from)
        .to(to_mailbox)
        .subject(subject)
        .header(ContentType::TEXT_HTML)
        .body(html_body.to_string())
        .map_err(|e| AppError::Internal(anyhow::anyhow!("failed to build email: {e}")))?;

    let creds = Credentials::new(
        config.mail_username.clone(),
        config.mail_password.clone(),
    );

    let mailer = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&config.mail_host)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("smtp relay error: {e}")))?
        .port(config.mail_port)
        .credentials(creds)
        .build();

    mailer
        .send(email)
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("failed to send email: {e}")))?;

    Ok(())
}
