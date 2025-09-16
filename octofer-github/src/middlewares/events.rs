use std::sync::Arc;

use anyhow::anyhow;
use axum::{
    body::{Body, Bytes},
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use octocrab::models::webhook_events::WebhookEvent;
use tracing::debug;

const GH_EVENT_HEADER: &str = "X-GitHub-Event";

// The event context that will be stored in request extensions
pub struct GitHubEventContext {
    pub event: WebhookEvent,
    pub installation_id: Option<i64>,
}
pub trait GitHubEventExt {
    fn github_event(&self) -> Option<Arc<GitHubEventContext>>;
}

impl GitHubEventExt for Request {
    fn github_event(&self) -> Option<Arc<GitHubEventContext>> {
        self.extensions().get::<Arc<GitHubEventContext>>().cloned()
    }
}

/// This middleware adds the GitHub event to the request context.
pub async fn github_event(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    debug!("Extracting github event");
    let event_header = extract_event_from_request(&req)?;
    let body = extract_body_from_request(&mut req).await?;
    let event = parse_webhook_event(&event_header, &body)?;

    debug!("Extracting Installation ID");
    let installation_id = event.installation.as_ref().map(|i| i.id().0 as i64);

    // Build the new request with the event in the context
    let ctx = GitHubEventContext {
        event,
        installation_id,
    };

    req.extensions_mut().insert(Arc::new(ctx));
    restore_request_body(&mut req, body);

    debug!("Going on...");
    Ok(next.run(req).await)
}

/// Restore the request body for downstream handlers
fn restore_request_body(req: &mut Request, body: Bytes) {
    *req.body_mut() = Body::from(body);
}

fn extract_event_from_request(req: &Request) -> Result<String, StatusCode> {
    req.headers()
        .get(GH_EVENT_HEADER)
        .ok_or(anyhow!("Missing required header: {}", GH_EVENT_HEADER))
        .map_err(|e| {
            tracing::error!("Missing header {}: {}", GH_EVENT_HEADER, e);
            axum::http::StatusCode::BAD_REQUEST
        })?
        .to_str()
        .map_err(|e| {
            tracing::error!("Invalid header value for {}: {}", GH_EVENT_HEADER, e);
            axum::http::StatusCode::BAD_REQUEST
        })
        .map(|s| s.to_string())
}

async fn extract_body_from_request(req: &mut Request) -> Result<Bytes, StatusCode> {
    let body = std::mem::replace(req.body_mut(), Body::empty());

    axum::body::to_bytes(body, usize::MAX).await.map_err(|e| {
        tracing::error!("Failed to read request body: {}", e);
        axum::http::StatusCode::BAD_REQUEST
    })
}

fn parse_webhook_event(event_type: &str, body: &Bytes) -> Result<WebhookEvent, StatusCode> {
    WebhookEvent::try_from_header_and_body(event_type, &body).map_err(|e| {
        tracing::error!("Failed to parse webhook event: {}", e);
        axum::http::StatusCode::BAD_REQUEST
    })
}
