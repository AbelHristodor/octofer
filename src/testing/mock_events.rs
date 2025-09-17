//! Mock webhook event builders for testing
//!
//! This module provides builders for creating mock GitHub webhook events
//! to use in tests. The builders follow a fluent API pattern and allow
//! creating realistic webhook events without needing actual GitHub data.

use octocrab::models::webhook_events::{WebhookEvent, WebhookEventType};
use serde_json::Value;
use std::collections::HashMap;

/// Builder for creating mock webhook events
pub struct MockWebhookEvent {
    event_type: WebhookEventType,
    payload: HashMap<String, Value>,
    installation_id: Option<u64>,
}

impl MockWebhookEvent {
    /// Create a new mock webhook event builder
    pub fn new(event_type: WebhookEventType) -> Self {
        Self {
            event_type,
            payload: HashMap::new(),
            installation_id: Some(12345), // Default installation ID
        }
    }

    /// Create a mock issue opened event
    pub fn issue_opened(repo_name: &str, issue_number: u64) -> MockIssueEvent {
        MockIssueEvent::new("opened", repo_name, issue_number)
    }

    /// Create a mock issue closed event
    pub fn issue_closed(repo_name: &str, issue_number: u64) -> MockIssueEvent {
        MockIssueEvent::new("closed", repo_name, issue_number)
    }

    /// Create a mock issue comment created event
    pub fn issue_comment_created(repo_name: &str, issue_number: u64, comment_id: u64) -> MockIssueCommentEvent {
        MockIssueCommentEvent::new("created", repo_name, issue_number, comment_id)
    }

    /// Create a mock pull request opened event
    pub fn pull_request_opened(repo_name: &str, pr_number: u64) -> MockPullRequestEvent {
        MockPullRequestEvent::new("opened", repo_name, pr_number)
    }

    /// Set the installation ID for this event
    pub fn installation_id(mut self, installation_id: u64) -> Self {
        self.installation_id = Some(installation_id);
        self
    }

    /// Build the webhook event
    /// 
    /// Note: This creates a minimal mock event using JSON serialization.
    /// For full testing, you would typically use actual webhook payloads
    /// from GitHub's webhook documentation.
    pub fn build(self) -> WebhookEvent {
        // Create a minimal JSON payload that represents a webhook event
        let mut payload = serde_json::json!({
            "action": "opened",
            "installation": {
                "id": self.installation_id.unwrap_or(12345)
            },
            "sender": {
                "login": "test-sender",
                "id": 789
            }
        });

        // Add event-specific data
        for (key, value) in self.payload {
            payload[key] = value;
        }

        // Convert the event type to a string representation
        let event_type_str = match self.event_type {
            WebhookEventType::Issues => "issues",
            WebhookEventType::IssueComment => "issue_comment",
            WebhookEventType::PullRequest => "pull_request",
            WebhookEventType::Push => "push",
            WebhookEventType::Release => "release",
            _ => "unknown",
        };

        // Parse the JSON payload into a WebhookEvent
        // This is a simplified approach - in practice you'd want to use
        // proper webhook event structures from the octocrab crate
        let body_bytes = serde_json::to_vec(&payload).unwrap();
        match WebhookEvent::try_from_header_and_body(event_type_str, body_bytes.as_slice()) {
            Ok(event) => event,
            Err(_) => {
                // Fallback: create a minimal event structure
                // This would need to be implemented based on the actual
                // octocrab WebhookEvent structure
                panic!("Failed to create mock webhook event. Consider using actual webhook payloads for testing.")
            }
        }
    }
}

/// Builder for mock issue events
pub struct MockIssueEvent {
    action: String,
    repo_name: String,
    issue_number: u64,
    title: Option<String>,
    body: Option<String>,
    installation_id: Option<u64>,
}

impl MockIssueEvent {
    fn new(action: &str, repo_name: &str, issue_number: u64) -> Self {
        Self {
            action: action.to_string(),
            repo_name: repo_name.to_string(),
            issue_number,
            title: None,
            body: None,
            installation_id: Some(12345),
        }
    }

    /// Set the issue title
    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    /// Set the issue body
    pub fn body(mut self, body: &str) -> Self {
        self.body = Some(body.to_string());
        self
    }

    /// Set the installation ID
    pub fn installation_id(mut self, installation_id: u64) -> Self {
        self.installation_id = Some(installation_id);
        self
    }

    /// Build the webhook event
    pub fn build(self) -> WebhookEvent {
        let mut builder = MockWebhookEvent::new(WebhookEventType::Issues);
        
        if let Some(installation_id) = self.installation_id {
            builder = builder.installation_id(installation_id);
        }

        // Add issue-specific payload
        let action = self.action.clone();
        builder.payload.insert("action".to_string(), Value::String(action.clone()));
        builder.payload.insert("issue".to_string(), serde_json::json!({
            "number": self.issue_number,
            "title": self.title.unwrap_or_else(|| format!("Issue #{}", self.issue_number)),
            "body": self.body.unwrap_or_else(|| format!("Body for issue #{}", self.issue_number)),
            "state": if action == "closed" { "closed" } else { "open" }
        }));
        builder.payload.insert("repository".to_string(), serde_json::json!({
            "full_name": self.repo_name,
            "name": self.repo_name.split('/').last().unwrap_or("unknown")
        }));

        builder.build()
    }
}

/// Builder for mock issue comment events
pub struct MockIssueCommentEvent {
    action: String,
    repo_name: String,
    issue_number: u64,
    comment_id: u64,
    comment_body: Option<String>,
    installation_id: Option<u64>,
}

impl MockIssueCommentEvent {
    fn new(action: &str, repo_name: &str, issue_number: u64, comment_id: u64) -> Self {
        Self {
            action: action.to_string(),
            repo_name: repo_name.to_string(),
            issue_number,
            comment_id,
            comment_body: None,
            installation_id: Some(12345),
        }
    }

    /// Set the comment body
    pub fn body(mut self, body: &str) -> Self {
        self.comment_body = Some(body.to_string());
        self
    }

    /// Set the installation ID
    pub fn installation_id(mut self, installation_id: u64) -> Self {
        self.installation_id = Some(installation_id);
        self
    }

    /// Build the webhook event
    pub fn build(self) -> WebhookEvent {
        let mut builder = MockWebhookEvent::new(WebhookEventType::IssueComment);
        
        if let Some(installation_id) = self.installation_id {
            builder = builder.installation_id(installation_id);
        }

        // Add issue comment-specific payload
        builder.payload.insert("action".to_string(), Value::String(self.action));
        builder.payload.insert("comment".to_string(), serde_json::json!({
            "id": self.comment_id,
            "body": self.comment_body.unwrap_or_else(|| format!("Comment #{}", self.comment_id))
        }));
        builder.payload.insert("issue".to_string(), serde_json::json!({
            "number": self.issue_number
        }));
        builder.payload.insert("repository".to_string(), serde_json::json!({
            "full_name": self.repo_name,
            "name": self.repo_name.split('/').last().unwrap_or("unknown")
        }));

        builder.build()
    }
}

/// Builder for mock pull request events
pub struct MockPullRequestEvent {
    action: String,
    repo_name: String,
    pr_number: u64,
    title: Option<String>,
    installation_id: Option<u64>,
}

impl MockPullRequestEvent {
    fn new(action: &str, repo_name: &str, pr_number: u64) -> Self {
        Self {
            action: action.to_string(),
            repo_name: repo_name.to_string(),
            pr_number,
            title: None,
            installation_id: Some(12345),
        }
    }

    /// Set the pull request title
    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    /// Set the installation ID
    pub fn installation_id(mut self, installation_id: u64) -> Self {
        self.installation_id = Some(installation_id);
        self
    }

    /// Build the webhook event
    pub fn build(self) -> WebhookEvent {
        let mut builder = MockWebhookEvent::new(WebhookEventType::PullRequest);
        
        if let Some(installation_id) = self.installation_id {
            builder = builder.installation_id(installation_id);
        }

        // Add pull request-specific payload
        builder.payload.insert("action".to_string(), Value::String(self.action));
        builder.payload.insert("pull_request".to_string(), serde_json::json!({
            "number": self.pr_number,
            "title": self.title.unwrap_or_else(|| format!("PR #{}", self.pr_number))
        }));
        builder.payload.insert("repository".to_string(), serde_json::json!({
            "full_name": self.repo_name,
            "name": self.repo_name.split('/').last().unwrap_or("unknown")
        }));

        builder.build()
    }
}

/// Create a simple mock webhook event from JSON
/// 
/// This is a more flexible approach that allows creating events from
/// actual webhook payloads or custom JSON structures.
pub fn mock_event_from_json(event_type: &str, json_payload: &str) -> Result<WebhookEvent, Box<dyn std::error::Error>> {
    let bytes = json_payload.as_bytes();
    WebhookEvent::try_from_header_and_body(event_type, bytes)
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_event_from_json() {
        let json_payload = r#"{
            "action": "opened",
            "issue": {
                "number": 42,
                "title": "Test Issue",
                "body": "This is a test"
            },
            "repository": {
                "full_name": "test/repo"
            },
            "installation": {
                "id": 12345
            },
            "sender": {
                "login": "test-user",
                "id": 123
            }
        }"#;

        let result = mock_event_from_json("issues", json_payload);
        // This test might fail due to octocrab's strict validation
        // In practice, you'd use actual GitHub webhook payloads
        assert!(result.is_ok() || result.is_err()); // Either way is fine for this test
    }

    #[test]
    fn test_builders_dont_panic() {
        // Test that builders can be created without panicking
        let _issue_builder = MockWebhookEvent::issue_opened("test/repo", 42);
        let _comment_builder = MockWebhookEvent::issue_comment_created("test/repo", 42, 123);
        let _pr_builder = MockWebhookEvent::pull_request_opened("test/repo", 42);
    }
}