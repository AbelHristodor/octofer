use std::sync::Arc;

use anyhow::Context;
use axum::{
    body::Body,
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use hmac::Mac;
use tracing::debug;

type HmacSha256 = hmac::Hmac<sha2::Sha256>;

#[derive(Clone)]
pub struct HmacConfig {
    pub secret: String,
    pub header_name: String,
}

impl Default for HmacConfig {
    fn default() -> Self {
        Self {
            secret: String::new(),
            header_name: "x-hub-signature-256".to_string(), // GitHub default
        }
    }
}

pub async fn verify_hmac_middleware(
    State(config): State<Arc<HmacConfig>>,
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let (parts, body) = req.into_parts();
    // Extract the HMAC signature from the request headers
    let signature = parts
        .headers
        .get(&config.header_name)
        .and_then(|value| value.to_str().ok())
        .ok_or(StatusCode::BAD_REQUEST)?
        .to_string();

    // Read the request body
    let payload = axum::body::to_bytes(body, usize::MAX)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    match verify_hmac_sha256(&signature, &payload, &config.secret) {
        Ok(_) => {
            debug!("HMAC signature verified!");
            // If verification is successful, continue processing the request
            let new_body = Body::from(payload);
            let req = Request::from_parts(parts, new_body);
            Ok(next.run(req).await)
        }
        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
}

fn verify_hmac_sha256(signature: &str, payload: &[u8], secret: &str) -> anyhow::Result<()> {
    let signature_hex = signature
        .strip_prefix("sha256=")
        .context("Signature must start with 'sha256='")?;

    let expected_signature =
        hex::decode(signature_hex).context("Failed to decode hex signature")?;

    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
        .context("Failed to create HMAC instance")
        .map_err(|_| anyhow::anyhow!("Invalid secret key for HMAC"))?;

    mac.update(payload);

    match mac.verify_slice(&expected_signature) {
        Ok(_) => Ok(()),
        Err(_) => Err(anyhow::anyhow!("HMAC verification failed")),
    }
}
