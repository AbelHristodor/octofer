//! Core types and traits for Octofer
//!
//! This module contains the fundamental types and traits used throughout the framework,
//! including the Context type and event handler definitions.

/// Context passed to event handlers containing event information and utilities
#[derive(Clone, Debug)]
pub struct Context {
    /// Event payload data
    pub payload: serde_json::Value,
    /// Event type (e.g., "issues", "issue_comment")
    pub event_type: String,
    /// Installation ID for GitHub App authentication
    pub installation_id: Option<u64>,
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
}

impl Default for Context {
    fn default() -> Self {
        Self {
            payload: serde_json::Value::Null,
            event_type: String::new(),
            installation_id: None,
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
