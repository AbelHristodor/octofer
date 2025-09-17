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

    /// Get the event type as a string
    /// TODO: FIx this to use serde serialization
    pub fn event_type(&self) -> &str {
        match &self.event {
            Some(e) => {
                // Convert the event kind to a string representation
                match &e.kind {
                    octocrab::models::webhook_events::WebhookEventType::Issues => "issues",
                    octocrab::models::webhook_events::WebhookEventType::IssueComment => {
                        "issue_comment"
                    }
                    octocrab::models::webhook_events::WebhookEventType::PullRequest => {
                        "pull_request"
                    }
                    octocrab::models::webhook_events::WebhookEventType::Push => "push",
                    octocrab::models::webhook_events::WebhookEventType::Create => "create",
                    octocrab::models::webhook_events::WebhookEventType::Delete => "delete",
                    octocrab::models::webhook_events::WebhookEventType::Fork => "fork",
                    octocrab::models::webhook_events::WebhookEventType::Star => "star",
                    octocrab::models::webhook_events::WebhookEventType::Watch => "watch",
                    octocrab::models::webhook_events::WebhookEventType::Release => "release",
                    _ => "unknown",
                }
            }
            None => "undefined",
        }
    }

    /// Get the event payload as a JSON value
    ///
    /// This provides access to the raw webhook payload data as a serde_json::Value,
    /// allowing handlers to extract specific fields they need.
    pub fn payload(&self) -> serde_json::Value {
        match &self.event {
            Some(event) => {
                // Convert the WebhookEvent to a JSON value
                serde_json::to_value(event).unwrap_or(serde_json::Value::Null)
            }
            None => serde_json::Value::Null,
        }
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
pub trait EventHandler<T>: Send + Sync
where
    T: Send + Sync + 'static,
{
    /// Handle an event with the provided context
    fn handle(
        &self,
        context: Context,
        extra: Arc<T>,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<()>> + Send>>;
}
