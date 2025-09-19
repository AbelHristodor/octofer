//! Webhook request handlers
//!
//! This module contains the HTTP request handlers for webhook endpoints.
//! These handlers process incoming GitHub webhook events and route them
//! to registered event handlers.

use crate::core::Context;
use crate::github::middlewares::GitHubEventExt;
use crate::webhook::AppState;
use axum::{
    extract::{Request, State},
    response::{IntoResponse, Response, Result},
};
use tracing::{error, info};

/// Handle incoming webhook requests
///
/// This is the main webhook endpoint handler that processes GitHub webhook events.
/// It extracts event information from the request, creates a Context, and routes
/// the event to all registered handlers for that event type.
///
/// # Request Processing Flow
///
/// 1. **Extract Event Data** - Gets GitHub event information from request extensions
///    (populated by the github_event_middleware)
/// 2. **Create Context** - Creates a Context with event data and GitHub client
/// 3. **Find Handlers** - Looks up registered handlers for this event type
/// 4. **Execute Handlers** - Runs all handlers sequentially for this event
/// 5. **Return Response** - Returns appropriate HTTP status code
///
/// # Response Codes
///
/// - `200 OK` - Event processed successfully (even if no handlers were registered)
/// - `400 BAD REQUEST` - Request missing required GitHub event information
/// - `500 INTERNAL SERVER ERROR` - One or more handlers failed with an error
///
/// # Error Handling
///
/// If any handler returns an error, the entire request is considered failed and
/// a 500 status code is returned. This prevents GitHub from considering the
/// webhook delivery successful when there are handler errors.
///
/// # Examples
///
/// This handler is typically not called directly, but is registered with the
/// Axum router:
///
/// ```rust,ignore
/// Router::new()
///     .route("/webhook", post(handle_webhook))
/// ```
///
/// The handler expects the request to have been processed by middleware that:
/// - Verifies the HMAC signature
/// - Extracts GitHub event information
/// - Populates request extensions with event data
pub async fn handle_webhook(State(state): State<AppState>, req: Request) -> Result<Response> {
    // Extract GitHub event information from the request extensions
    let github_event_context = match req.github_event() {
        Some(g) => g,
        None => {
            error!("Request does not contain GitHub event information!");
            return Ok(axum::http::StatusCode::BAD_REQUEST.into_response());
        }
    };

    let cloned_event = github_event_context.event.clone();

    let ctx = Context::with_github_client(
        Some(cloned_event),
        github_event_context.installation_id.map(|id| id as u64),
        state.github_client,
    );

    // Get handlers for this event type
    if let Some(event_handlers) = state.handlers.read().await.get(&ctx.kind()) {
        for handler in event_handlers {
            match handler(ctx.clone()).await {
                Ok(_) => {
                    info!("Handler executed successfully");
                }
                Err(e) => {
                    error!("Handler failed with error: {:?}", e);
                    return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR.into());
                }
            }
        }
    } else {
        info!("No handlers registered for event: {}", ctx.kind());
    }

    Ok(axum::http::StatusCode::OK.into_response())
}

/// Handle health check requests
///
/// This is a simple health check endpoint that returns a 200 OK status.
/// It can be used by load balancers, monitoring systems, or deployment
/// tools to verify that the webhook server is running and responsive.
///
/// # Response
///
/// Always returns `200 OK` with an empty body.
///
/// # Examples
///
/// This handler is registered at the `/health` endpoint:
///
/// ```bash
/// curl http://localhost:8000/health
/// # Returns: 200 OK
/// ```
///
/// It can be used in Docker health checks:
///
/// ```dockerfile
/// HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
///   CMD curl -f http://localhost:8000/health || exit 1
/// ```
///
/// Or in Kubernetes liveness/readiness probes:
///
/// ```yaml
/// livenessProbe:
///   httpGet:
///     path: /health
///     port: 8000
///   initialDelaySeconds: 30
///   periodSeconds: 10
/// ```
pub async fn handle_health() -> Result<Response> {
    Ok(axum::http::StatusCode::OK.into_response())
}
