//! GitHub event processing middleware

use axum::{
    body::{Body, Bytes},
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use octocrab::models::webhook_events::WebhookEvent;
use std::sync::Arc;
use tracing::debug;

const GITHUB_EVENT_HEADER: &str = "X-GitHub-Event";

/// Context containing GitHub event information
pub struct GitHubEventContext {
    /// The parsed webhook event
    pub event: WebhookEvent,
    /// Installation ID if available
    pub installation_id: Option<i64>,
}

/// Extension trait for extracting GitHub event context from requests
pub trait GitHubEventExt {
    /// Get the GitHub event context from the request
    fn github_event(&self) -> Option<Arc<GitHubEventContext>>;
}

impl GitHubEventExt for Request {
    fn github_event(&self) -> Option<Arc<GitHubEventContext>> {
        self.extensions().get::<Arc<GitHubEventContext>>().cloned()
    }
}

/// Middleware to extract and parse GitHub webhook events
pub async fn github_event_middleware(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    debug!("Processing GitHub webhook event");

    let event_type = extract_event_type(&req)?;
    let body = extract_request_body(&mut req).await?;
    let event = parse_webhook_event(&event_type, &body)?;

    let installation_id = event.installation.as_ref().map(|i| i.id().0 as i64);
    debug!("Extracted installation ID: {:?}", installation_id);

    // Store event context in request extensions
    let context = GitHubEventContext {
        event,
        installation_id,
    };
    req.extensions_mut().insert(Arc::new(context));

    // Restore the request body for downstream handlers
    restore_request_body(&mut req, body);

    debug!("GitHub event processed successfully");
    Ok(next.run(req).await)
}

/// Extract the GitHub event type from request headers
fn extract_event_type(req: &Request) -> Result<String, StatusCode> {
    req.headers()
        .get(GITHUB_EVENT_HEADER)
        .ok_or_else(|| {
            tracing::error!("Missing required header: {}", GITHUB_EVENT_HEADER);
            StatusCode::BAD_REQUEST
        })?
        .to_str()
        .map_err(|e| {
            tracing::error!("Invalid header value for {}: {}", GITHUB_EVENT_HEADER, e);
            StatusCode::BAD_REQUEST
        })
        .map(|s| s.to_string())
}

/// Extract and consume the request body
async fn extract_request_body(req: &mut Request) -> Result<Bytes, StatusCode> {
    let body = std::mem::replace(req.body_mut(), Body::empty());

    axum::body::to_bytes(body, usize::MAX).await.map_err(|e| {
        tracing::error!("Failed to read request body: {}", e);
        StatusCode::BAD_REQUEST
    })
}

/// Parse the webhook event from the event type and body
fn parse_webhook_event(event_type: &str, body: &Bytes) -> Result<WebhookEvent, StatusCode> {
    WebhookEvent::try_from_header_and_body(event_type, body).map_err(|e| {
        tracing::error!("Failed to parse webhook event: {}", e);
        StatusCode::BAD_REQUEST
    })
}

/// Restore the request body for downstream processing
fn restore_request_body(req: &mut Request, body: Bytes) {
    *req.body_mut() = Body::from(body);
}
