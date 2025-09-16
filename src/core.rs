//! Core types and traits for Octofer
//!
//! This module contains the fundamental types and traits used throughout the framework,
//! including the Context type and event handler definitions.

use octocrab::models::webhook_events::WebhookEvent;

use crate::{github::GitHubClient, webhook::WebhookEventKind};
use std::sync::Arc;

/// Context passed to event handlers containing event information and utilities
#[derive(Clone, Debug, Default)]
pub struct Context {
    /// Event payload data
    pub event: Option<WebhookEvent>,
    /// Installation ID for GitHub App authentication
    pub installation_id: Option<u64>,
    /// GitHub client for API operations (if available)
    pub github_client: Option<Arc<GitHubClient>>,
}

impl Context {
    /// Create a new context
    pub fn new(event: Option<WebhookEvent>, installation_id: Option<u64>) -> Self {
        Self {
            event,
            installation_id,
            github_client: None,
        }
    }

    /// Create a new context with GitHub client
    pub fn with_github_client(
        event: Option<WebhookEvent>,
        installation_id: Option<u64>,
        github_client: Option<Arc<GitHubClient>>,
    ) -> Self {
        Self {
            event,
            installation_id,
            github_client,
        }
    }

    pub fn kind(&self) -> WebhookEventKind {
        match &self.event {
            Some(e) => serde_json::to_value(e.kind.clone()).unwrap().to_string(),
            None => "Undefined".to_string(),
        }
    }

    /// Get the event payload
    pub fn event(&self) -> &Option<WebhookEvent> {
        &self.event
    }

    /// Get the installation ID
    pub fn installation_id(&self) -> Option<u64> {
        self.installation_id
    }

    /// Get access to the GitHub client
    ///
    /// Returns a reference to the GitHub client if available. The client is already
    /// authenticated and can be used to make API calls. If an installation ID is
    /// available in the context, the client will automatically use the appropriate
    /// installation token for API calls.
    pub fn github(&self) -> Option<&Arc<GitHubClient>> {
        self.github_client.as_ref()
    }

    /// Get an authenticated installation client for the current installation
    ///
    /// This is a convenience method that returns an Octocrab client authenticated
    /// as the specific installation from this event context. Returns None if no
    /// GitHub client is available or no installation ID is present.
    pub async fn installation_client(&self) -> anyhow::Result<Option<octocrab::Octocrab>> {
        match (&self.github_client, self.installation_id) {
            (Some(client), Some(installation_id)) => {
                let octocrab_client = client.installation_client(installation_id).await?;
                Ok(Some(octocrab_client))
            }
            _ => Ok(None),
        }
    }
}

/// Type alias for event handler functions
pub type EventHandlerFn = Box<
    dyn Fn(
            Context,
        )
            -> std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<()>> + Send>>
        + Send
        + Sync,
>;

/// Trait for types that can handle GitHub events
pub trait EventHandler: Send + Sync {
    /// Handle an event with the provided context
    fn handle(
        &self,
        context: Context,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<()>> + Send>>;
}
