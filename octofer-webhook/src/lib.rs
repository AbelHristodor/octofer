//! # Octofer Webhook
//!
//! Webhook handling and event routing for Octofer framework.

use anyhow::Result;
use octofer_core::{Context, EventHandlerFn, GitHubPayload};
use std::collections::HashMap;

/// Webhook server for handling GitHub events
pub struct WebhookServer {
    handlers: HashMap<String, Vec<EventHandlerFn>>,
    port: u16,
}

impl WebhookServer {
    /// Create a new webhook server
    pub fn new(port: u16) -> Self {
        Self {
            handlers: HashMap::new(),
            port,
        }
    }

    /// Add an event handler
    pub fn on<F, Fut>(&mut self, event: impl Into<String>, handler: F)
    where
        F: Fn(Context) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Result<()>> + Send + 'static,
    {
        let event = event.into();
        let boxed_handler: EventHandlerFn = Box::new(move |context| Box::pin(handler(context)));

        self.handlers.entry(event).or_default().push(boxed_handler);
    }

    /// Start the webhook server
    pub async fn start(&self) -> Result<()> {
        tracing::info!("Starting webhook server on port {}", self.port);
        // Implementation will use a web framework like axum or warp
        Ok(())
    }

    /// Handle an incoming webhook
    pub async fn handle_webhook(
        &self,
        event_name: &str,
        delivery_id: &str,
        payload: serde_json::Value,
    ) -> Result<()> {
        let payload = GitHubPayload::from_json(payload)?;
        let context = Context {
            payload,
            event_name: event_name.to_string(),
            delivery_id: delivery_id.to_string(),
        };

        if let Some(handlers) = self.handlers.get(event_name) {
            for handler in handlers {
                if let Err(e) = handler(context.clone()).await {
                    tracing::error!("Handler error for event {}: {}", event_name, e);
                }
            }
        }

        Ok(())
    }
}

impl Default for WebhookServer {
    fn default() -> Self {
        Self::new(3000)
    }
}
