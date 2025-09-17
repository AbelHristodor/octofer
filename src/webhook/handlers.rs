//! Webhook request handlers

use crate::core::Context;
use crate::github::middlewares::GitHubEventExt;
use crate::webhook::AppState;
use axum::{
    extract::{Request, State},
    response::{IntoResponse, Response, Result},
};
use tracing::{error, info};

/// Handle incoming webhook requests
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
pub async fn handle_health() -> Result<Response> {
    Ok(axum::http::StatusCode::OK.into_response())
}
