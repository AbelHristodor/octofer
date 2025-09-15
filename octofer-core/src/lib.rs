//! # Octofer Core
//!
//! Core types, traits, and utilities for the Octofer framework.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Context provides access to the current GitHub event and API
#[derive(Clone)]
pub struct Context {
    pub payload: GitHubPayload,
    pub event_name: String,
    pub delivery_id: String,
}

impl Context {
    /// Get the event payload
    pub fn payload(&self) -> &GitHubPayload {
        &self.payload
    }

    /// Get the event name
    pub fn event_name(&self) -> &str {
        &self.event_name
    }

    /// Get the delivery ID
    pub fn delivery_id(&self) -> &str {
        &self.delivery_id
    }
}

/// GitHub webhook payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubPayload {
    #[serde(flatten)]
    pub data: HashMap<String, serde_json::Value>,
}

impl GitHubPayload {
    /// Create a new payload from raw JSON
    pub fn from_json(json: serde_json::Value) -> anyhow::Result<Self> {
        let data = match json {
            serde_json::Value::Object(map) => map.into_iter().collect(),
            _ => return Err(anyhow::anyhow!("Payload must be a JSON object")),
        };

        Ok(Self { data })
    }

    /// Get a field from the payload
    pub fn get(&self, key: &str) -> Option<&serde_json::Value> {
        self.data.get(key)
    }
}

/// Event handler trait
pub trait EventHandler: Send + Sync {
    fn handle(
        &self,
        context: Context,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<()>> + Send>>;
}

/// Event handler function type
pub type EventHandlerFn = Box<
    dyn Fn(
            Context,
        )
            -> std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<()>> + Send>>
        + Send
        + Sync,
>;
