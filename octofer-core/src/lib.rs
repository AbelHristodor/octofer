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

    /// Check if this is an issue comment event
    pub fn is_issue_comment(&self) -> bool {
        self.event_name == "issue_comment" || self.payload.is_issue_comment()
    }

    /// Parse the payload as an issue comment event
    pub fn as_issue_comment(&self) -> anyhow::Result<IssueCommentEvent> {
        self.payload.as_issue_comment()
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

    /// Parse the payload as an issue comment event
    /// Returns the action, issue, comment, and repository information
    pub fn as_issue_comment(&self) -> anyhow::Result<IssueCommentEvent> {
        let action = self
            .get("action")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing or invalid 'action' field"))?;

        let issue = self
            .get("issue")
            .ok_or_else(|| anyhow::anyhow!("Missing 'issue' field"))?;

        let comment = self
            .get("comment")
            .ok_or_else(|| anyhow::anyhow!("Missing 'comment' field"))?;

        let repository = self
            .get("repository")
            .ok_or_else(|| anyhow::anyhow!("Missing 'repository' field"))?;

        Ok(IssueCommentEvent {
            action: action.to_string(),
            issue: issue.clone(),
            comment: comment.clone(),
            repository: repository.clone(),
        })
    }

    /// Check if this payload is an issue comment event
    pub fn is_issue_comment(&self) -> bool {
        self.get("comment").is_some() && self.get("issue").is_some()
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

/// Issue comment event data extracted from webhook payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueCommentEvent {
    pub action: String,
    pub issue: serde_json::Value,
    pub comment: serde_json::Value,
    pub repository: serde_json::Value,
}

impl IssueCommentEvent {
    /// Get the issue number
    pub fn issue_number(&self) -> Option<u64> {
        self.issue.get("number")?.as_u64()
    }

    /// Get the issue title
    pub fn issue_title(&self) -> Option<&str> {
        self.issue.get("title")?.as_str()
    }

    /// Get the comment body
    pub fn comment_body(&self) -> Option<&str> {
        self.comment.get("body")?.as_str()
    }

    /// Get the comment author login
    pub fn comment_author(&self) -> Option<&str> {
        self.comment.get("user")?.get("login")?.as_str()
    }

    /// Get the repository owner
    pub fn repository_owner(&self) -> Option<&str> {
        self.repository.get("owner")?.get("login")?.as_str()
    }

    /// Get the repository name
    pub fn repository_name(&self) -> Option<&str> {
        self.repository.get("name")?.as_str()
    }

    /// Get the full repository name (owner/repo)
    pub fn repository_full_name(&self) -> Option<&str> {
        self.repository.get("full_name")?.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use serde_json::json;

    #[tokio::test]
    async fn test_issue_comment_event_parsing() -> Result<()> {
        // Create a sample issue comment payload
        let payload_json = json!({
            "action": "created",
            "issue": {
                "number": 42,
                "title": "Test Issue for Comment",
                "body": "This is a test issue",
                "user": {
                    "login": "issue_author",
                    "id": 12345
                }
            },
            "comment": {
                "id": 98765,
                "body": "This is a test comment mentioning @bot",
                "user": {
                    "login": "comment_author",
                    "id": 67890
                }
            },
            "repository": {
                "name": "test-repo",
                "full_name": "owner/test-repo",
                "owner": {
                    "login": "owner",
                    "id": 111
                }
            }
        });

        // Create a GitHubPayload from the JSON
        let payload = GitHubPayload::from_json(payload_json)?;

        // Test that it's correctly identified as an issue comment
        assert!(payload.is_issue_comment());

        // Test parsing as issue comment event
        let event = payload.as_issue_comment()?;

        assert_eq!(event.action, "created");
        assert_eq!(event.issue_number(), Some(42));
        assert_eq!(event.issue_title(), Some("Test Issue for Comment"));
        assert_eq!(
            event.comment_body(),
            Some("This is a test comment mentioning @bot")
        );
        assert_eq!(event.comment_author(), Some("comment_author"));
        assert_eq!(event.repository_owner(), Some("owner"));
        assert_eq!(event.repository_name(), Some("test-repo"));
        assert_eq!(event.repository_full_name(), Some("owner/test-repo"));

        // Test Context helper methods
        let context = Context {
            payload,
            event_name: "issue_comment".to_string(),
            delivery_id: "test-delivery-123".to_string(),
        };

        assert!(context.is_issue_comment());
        let context_event = context.as_issue_comment()?;
        assert_eq!(context_event.action, "created");

        Ok(())
    }

    #[tokio::test]
    async fn test_non_issue_comment_payload() -> Result<()> {
        // Create a non-issue comment payload (just an issue event)
        let payload_json = json!({
            "action": "opened",
            "issue": {
                "number": 1,
                "title": "Test Issue",
                "body": "This is just an issue event"
            },
            "repository": {
                "name": "test-repo",
                "full_name": "owner/test-repo"
            }
        });

        let payload = GitHubPayload::from_json(payload_json)?;

        // This should not be identified as an issue comment (missing comment field)
        assert!(!payload.is_issue_comment());

        // Attempting to parse as issue comment should fail
        assert!(payload.as_issue_comment().is_err());

        Ok(())
    }
}
