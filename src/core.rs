//! Core types and traits for Octofer
//!
//! This module contains the fundamental types and traits used throughout the framework,
//! including the Context type and event handler definitions.

use crate::github::GitHubClient;
use std::sync::Arc;

/// Context passed to event handlers containing event information and utilities
#[derive(Clone, Debug)]
pub struct Context {
    /// Event payload data
    pub payload: serde_json::Value,
    /// Event type (e.g., "issues", "issue_comment")
    pub event_type: String,
    /// Installation ID for GitHub App authentication
    pub installation_id: Option<u64>,
    /// GitHub client for API operations (if available)
    pub github_client: Option<Arc<GitHubClient>>,
}

impl Context {
    /// Create a new context
    pub fn new(
        payload: serde_json::Value,
        event_type: String,
        installation_id: Option<u64>,
    ) -> Self {
        Self {
            payload,
            event_type,
            installation_id,
            github_client: None,
        }
    }

    /// Create a new context with GitHub client
    pub fn with_github_client(
        payload: serde_json::Value,
        event_type: String,
        installation_id: Option<u64>,
        github_client: Option<Arc<GitHubClient>>,
    ) -> Self {
        Self {
            payload,
            event_type,
            installation_id,
            github_client,
        }
    }

    /// Get the event payload
    pub fn payload(&self) -> &serde_json::Value {
        &self.payload
    }

    /// Get the event type
    pub fn event_type(&self) -> &str {
        &self.event_type
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

impl Default for Context {
    fn default() -> Self {
        Self {
            payload: serde_json::Value::Null,
            event_type: String::new(),
            installation_id: None,
            github_client: None,
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
