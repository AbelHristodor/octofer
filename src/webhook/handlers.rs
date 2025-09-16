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
pub async fn handle_webhook(
    State(state): State<AppState>, 
    req: Request
) -> Result<Response> {
    // Extract GitHub event information from the request extensions
    let github_event_context = req.github_event();
    
    let (event_type, installation_id, payload) = if let Some(gh_ctx) = &github_event_context {
        // Convert the WebhookEvent to JSON to extract type information
        let event_json = serde_json::to_value(&gh_ctx.event).unwrap_or(serde_json::Value::Null);
        
        // Try to determine event type from the JSON structure
        let event_type = if event_json.get("issue").is_some() && event_json.get("comment").is_some() {
            "issue_comment"
        } else if event_json.get("issue").is_some() {
            "issues"
        } else if event_json.get("pull_request").is_some() {
            "pull_request"
        } else {
            "unknown"
        };
        
        let installation_id = gh_ctx.installation_id.map(|id| id as u64);
        
        (event_type.to_string(), installation_id, event_json)
    } else {
        // Fallback for when no GitHub event context is available
        info!("No GitHub event context found, using default values");
        ("issue_comment".to_string(), None, serde_json::Value::Null)
    };
    
    // Create context with GitHub client and actual event data
    let ctx = Context::with_github_client(
        payload,
        event_type.clone(),
        installation_id,
        state.github_client.clone(),
    );

    info!("Processing webhook event: {}", event_type);
    if let Some(id) = installation_id {
        info!("Installation ID: {}", id);
    }

    // Get handlers for this event type
    if let Some(event_handlers) = state.handlers.read().await.get(&event_type) {
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
        info!("No handlers registered for event: {}", event_type);
    }

    Ok(axum::http::StatusCode::OK.into_response())
}

/// Handle health check requests
pub async fn handle_health() -> Result<Response> {
    Ok(axum::http::StatusCode::OK.into_response())
}
