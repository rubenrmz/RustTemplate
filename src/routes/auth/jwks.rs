// src/routes/jwks.rs
use axum::{extract::State, routing::get, Json, Router};
use base64::Engine;
use rsa::pkcs8::DecodePublicKey;
use rsa::traits::PublicKeyParts;
use rsa::RsaPublicKey;
use serde::Serialize;
use std::sync::Arc;

use crate::state::AppState;

#[derive(Serialize)]
struct Jwks {
    keys: Vec<Jwk>,
}

#[derive(Serialize)]
struct Jwk {
    kty: &'static str,
    #[serde(rename = "use")]
    use_: &'static str,
    alg: &'static str,
    kid: &'static str,
    n: String,
    e: String,
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/.well-known/jwks.json", get(jwks_handler))
}

async fn jwks_handler(State(state): State<Arc<AppState>>) -> Json<Jwks> {
    let public_key = RsaPublicKey::from_public_key_pem(
        std::str::from_utf8(&state.jwt_public_key_pem).expect("public key is not valid UTF-8"),
    )
    .expect("failed to parse RSA public key");

    let n = public_key.n().to_bytes_be();
    let e = public_key.e().to_bytes_be();

    let engine = base64::engine::general_purpose::URL_SAFE_NO_PAD;

    Json(Jwks {
        keys: vec![Jwk {
            kty: "RSA",
            use_: "sig",
            alg: "RS256",
            kid: "auth-key-1",
            n: engine.encode(n),
            e: engine.encode(e),
        }],
    })
}
