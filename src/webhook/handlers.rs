//! Webhook request handlers

use crate::core::Context;
use crate::webhook::AppState;
use axum::{
    extract::State,
    response::{IntoResponse, Response, Result},
};
use tracing::{error, info};

/// Handle incoming webhook requests
pub async fn handle_webhook(State(state): State<AppState>) -> Result<Response> {
    let event_name = "issue_comment";
    let ctx = Context::default();

    info!("Processing webhook event: {}", event_name);

    // Get handlers for this event type
    if let Some(event_handlers) = state.handlers.read().await.get(event_name) {
        for handler in event_handlers {
            match handler(ctx.clone()).await {
                Ok(_) => {
                    info!("Handler executed successfully");
                }
                Err(e) => {
                    error!("Handler failed with error: {:?}", e);
                }
            }
        }
    } else {
        info!("No handlers registered for event: {}", event_name);
    }

    Ok(axum::http::StatusCode::OK.into_response())
}

/// Handle health check requests
pub async fn handle_health() -> Result<Response> {
    Ok(axum::http::StatusCode::OK.into_response())
}
