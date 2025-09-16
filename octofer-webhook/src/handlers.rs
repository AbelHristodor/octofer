use axum::{
    extract::State,
    response::{IntoResponse, Response, Result},
};
use octofer_core::Context;
use tracing::{error, info};

use crate::AppState;

pub async fn handle_webhook(State(state): State<AppState>) -> Result<Response> {
    let event_name = "issue_comment";
    let ctx = Context {};

    info!("Event received: {event_name}");

    // TODO: error handling
    if let Some(event_handlers) = state.handlers.read().await.get(event_name) {
        for handler in event_handlers {
            if let Err(e) = handler(ctx.clone()).await {
                error!("I received error from handler: {:?}", e);
            } else {
                info!("All good, handler done!");
            }
        }
    } else {
        error!("No handler found!");
    }

    Ok(axum::http::StatusCode::OK.into_response())
}

pub async fn handle_health() -> Result<Response> {
    Ok(axum::http::StatusCode::OK.into_response())
}
