//! Test context helpers for creating mock contexts
//!
//! This module provides utilities for creating test contexts with mock data,
//! making it easy to test event handlers in isolation.

use crate::core::Context;
use crate::github::GitHubClient;
use crate::testing::MockWebhookEvent;
use octocrab::models::webhook_events::WebhookEvent;
use std::sync::Arc;

/// Helper for creating test contexts
pub struct TestContext;

impl TestContext {
    /// Create a new empty context for testing
    pub fn new() -> Context {
        Context::new(None, None)
    }

    /// Create a context with a mock webhook event
    pub fn with_event(event: WebhookEvent) -> Context {
        let installation_id = event.installation.as_ref().map(|i| i.id().0);
        Context::new(Some(event), installation_id)
    }

    /// Create a context with a mock issue event
    pub fn with_issue_event(repo_name: &str, issue_number: u64) -> Context {
        let event = MockWebhookEvent::issue_opened(repo_name, issue_number).build();
        Self::with_event(event)
    }

    /// Create a context with a mock issue comment event
    pub fn with_issue_comment_event(repo_name: &str, issue_number: u64, comment_id: u64) -> Context {
        let event = MockWebhookEvent::issue_comment_created(repo_name, issue_number, comment_id).build();
        Self::with_event(event)
    }

    /// Create a context with a mock pull request event
    pub fn with_pull_request_event(repo_name: &str, pr_number: u64) -> Context {
        let event = MockWebhookEvent::pull_request_opened(repo_name, pr_number).build();
        Self::with_event(event)
    }

    /// Create a context with a specific installation ID
    pub fn with_installation_id(installation_id: u64) -> Context {
        Context::new(None, Some(installation_id))
    }

    /// Create a context with both event and GitHub client
    pub fn with_event_and_client(event: WebhookEvent, github_client: Arc<GitHubClient>) -> Context {
        let installation_id = event.installation.as_ref().map(|i| i.id().0);
        Context::with_github_client(Some(event), installation_id, Some(github_client))
    }

    /// Create a context with mock GitHub client
    /// Note: This would be used with a mock client implementation
    pub fn with_mock_client(event: Option<WebhookEvent>) -> Context {
        // For now, return a context without a client
        // In a full implementation, you would create a mock GitHubClient here
        let installation_id = event.as_ref().and_then(|e| e.installation.as_ref()).map(|i| i.id().0);
        Context::new(event, installation_id)
    }
}

impl Default for TestContext {
    fn default() -> Self {
        Self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use octocrab::models::webhook_events::WebhookEventType;

    #[test]
    fn test_empty_context() {
        let context = TestContext::new();
        assert!(context.event.is_none());
        assert!(context.installation_id.is_none());
        assert!(context.github_client.is_none());
    }

    #[test]
    fn test_context_with_installation_id() {
        let context = TestContext::with_installation_id(12345);
        assert!(context.event.is_none());
        assert_eq!(context.installation_id, Some(12345));
        assert!(context.github_client.is_none());
    }

    #[test]
    fn test_context_with_issue_event() {
        let context = TestContext::with_issue_event("test/repo", 42);
        assert!(context.event.is_some());
        
        // Note: The installation_id will depend on the mock implementation
        // Since we're using a simplified mock that may fail to parse,
        // we just check that we have an event
        let event = context.event.unwrap();
        assert!(matches!(event.kind, WebhookEventType::Issues));
    }

    #[test]
    fn test_context_with_issue_comment_event() {
        let context = TestContext::with_issue_comment_event("test/repo", 42, 123);
        assert!(context.event.is_some());
        
        let event = context.event.unwrap();
        assert!(matches!(event.kind, WebhookEventType::IssueComment));
    }

    #[test]
    fn test_context_with_pull_request_event() {
        let context = TestContext::with_pull_request_event("test/repo", 42);
        assert!(context.event.is_some());
        
        let event = context.event.unwrap();
        assert!(matches!(event.kind, WebhookEventType::PullRequest));
    }
}