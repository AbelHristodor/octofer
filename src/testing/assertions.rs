//! Testing assertion helpers
//!
//! This module provides specialized assertions for testing GitHub Apps,
//! making it easier to verify that event handlers behave correctly.

use crate::testing::MockGitHubClient;
use octocrab::models::webhook_events::{WebhookEvent, WebhookEventType};
use crate::core::Context;

/// Assert that a webhook event matches expected properties
pub fn assert_event_type(event: &WebhookEvent, expected_type: WebhookEventType) {
    assert_eq!(
        std::mem::discriminant(&event.kind),
        std::mem::discriminant(&expected_type),
        "Event type mismatch: expected {:?}, got {:?}",
        expected_type,
        event.kind
    );
}

/// Assert that a context has an event of the expected type
pub fn assert_context_event_type(context: &Context, expected_type: WebhookEventType) {
    match &context.event {
        Some(event) => assert_event_type(event, expected_type),
        None => panic!("Context has no event, expected {:?}", expected_type),
    }
}

/// Assert that a context has a specific installation ID
pub fn assert_installation_id(context: &Context, expected_id: u64) {
    match context.installation_id {
        Some(id) => assert_eq!(id, expected_id, "Installation ID mismatch"),
        None => panic!("Context has no installation ID, expected {}", expected_id),
    }
}

/// Assert that a context has an installation ID
pub fn assert_has_installation_id(context: &Context) {
    assert!(
        context.installation_id.is_some(),
        "Context should have an installation ID"
    );
}

/// Assert that a context has no installation ID
pub fn assert_no_installation_id(context: &Context) {
    assert!(
        context.installation_id.is_none(),
        "Context should not have an installation ID"
    );
}

/// Assert that a context has a GitHub client
pub fn assert_has_github_client(context: &Context) {
    assert!(
        context.github_client.is_some(),
        "Context should have a GitHub client"
    );
}

/// Assert that a context has no GitHub client
pub fn assert_no_github_client(context: &Context) {
    assert!(
        context.github_client.is_none(),
        "Context should not have a GitHub client"
    );
}

/// Assert that a specific API call was made to the mock client
pub fn assert_api_call_made(client: &MockGitHubClient, method: &str, path: &str) {
    assert!(
        client.was_called(method, path),
        "Expected API call not made: {} {}",
        method,
        path
    );
}

/// Assert that a specific API call was made a certain number of times
pub fn assert_api_call_count(client: &MockGitHubClient, method: &str, path: &str, expected_count: usize) {
    let actual_count = client.call_count(method, path);
    assert_eq!(
        actual_count,
        expected_count,
        "API call count mismatch for {} {}: expected {}, got {}",
        method,
        path,
        expected_count,
        actual_count
    );
}

/// Assert that no API calls were made to the mock client
pub fn assert_no_api_calls(client: &MockGitHubClient) {
    let calls = client.get_calls();
    assert!(
        calls.is_empty(),
        "Expected no API calls, but {} calls were made: {:?}",
        calls.len(),
        calls
    );
}

/// Assert that a specific number of API calls were made
pub fn assert_total_api_calls(client: &MockGitHubClient, expected_count: usize) {
    let calls = client.get_calls();
    assert_eq!(
        calls.len(),
        expected_count,
        "Total API call count mismatch: expected {}, got {}",
        expected_count,
        calls.len()
    );
}

/// GitHub API assertion builder for more complex verifications
pub struct ApiAssertions<'a> {
    client: &'a MockGitHubClient,
}

impl<'a> ApiAssertions<'a> {
    /// Create a new API assertions builder
    pub fn new(client: &'a MockGitHubClient) -> Self {
        Self { client }
    }

    /// Assert that a specific call was made
    pub fn called(self, method: &str, path: &str) -> Self {
        assert_api_call_made(self.client, method, path);
        self
    }

    /// Assert that a specific call was made a certain number of times
    pub fn called_times(self, method: &str, path: &str, count: usize) -> Self {
        assert_api_call_count(self.client, method, path, count);
        self
    }

    /// Assert that no calls were made
    pub fn no_calls(self) -> Self {
        assert_no_api_calls(self.client);
        self
    }

    /// Assert the total number of calls
    pub fn total_calls(self, count: usize) -> Self {
        assert_total_api_calls(self.client, count);
        self
    }

    /// Assert that a comment was created
    pub fn comment_created(self, repo: &str, issue_number: u64) -> Self {
        let path = format!("/repos/{}/issues/{}/comments", repo, issue_number);
        self.called("POST", &path)
    }

    /// Assert that an issue was updated
    pub fn issue_updated(self, repo: &str, issue_number: u64) -> Self {
        let path = format!("/repos/{}/issues/{}", repo, issue_number);
        self.called("PATCH", &path)
    }

    /// Assert that labels were added to an issue
    pub fn labels_added(self, repo: &str, issue_number: u64) -> Self {
        let path = format!("/repos/{}/issues/{}/labels", repo, issue_number);
        self.called("POST", &path)
    }

    /// Assert that a repository was fetched
    pub fn repository_fetched(self, repo: &str) -> Self {
        let path = format!("/repos/{}", repo);
        self.called("GET", &path)
    }
}

/// Context assertion builder for more complex verifications
pub struct ContextAssertions<'a> {
    context: &'a Context,
}

impl<'a> ContextAssertions<'a> {
    /// Create a new context assertions builder
    pub fn new(context: &'a Context) -> Self {
        Self { context }
    }

    /// Assert the event type
    pub fn event_type(self, expected_type: WebhookEventType) -> Self {
        assert_context_event_type(self.context, expected_type);
        self
    }

    /// Assert the installation ID
    pub fn installation_id(self, expected_id: u64) -> Self {
        assert_installation_id(self.context, expected_id);
        self
    }

    /// Assert that an installation ID exists
    pub fn has_installation_id(self) -> Self {
        assert_has_installation_id(self.context);
        self
    }

    /// Assert that no installation ID exists
    pub fn no_installation_id(self) -> Self {
        assert_no_installation_id(self.context);
        self
    }

    /// Assert that a GitHub client exists
    pub fn has_github_client(self) -> Self {
        assert_has_github_client(self.context);
        self
    }

    /// Assert that no GitHub client exists
    pub fn no_github_client(self) -> Self {
        assert_no_github_client(self.context);
        self
    }
}

/// Convenience function to create API assertions
pub fn assert_api(client: &MockGitHubClient) -> ApiAssertions<'_> {
    ApiAssertions::new(client)
}

/// Convenience function to create context assertions
pub fn assert_context(context: &Context) -> ContextAssertions<'_> {
    ContextAssertions::new(context)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{MockWebhookEvent, TestContext};

    #[test]
    fn test_context_assertions() {
        let context = TestContext::with_issue_event("test/repo", 42);
        
        assert_context(&context)
            .event_type(WebhookEventType::Issues)
            .installation_id(12345)
            .has_installation_id()
            .no_github_client();
    }

    #[tokio::test]
    async fn test_api_assertions() {
        let client = MockGitHubClient::new();
        
        // Make some API calls
        let _ = client.create_issue_comment("test/repo", 42, "Hello").await;
        let _ = client.update_issue("test/repo", 42, Some("New title"), None, None).await;
        let _ = client.create_issue_comment("test/repo", 43, "World").await;

        assert_api(&client)
            .comment_created("test/repo", 42)
            .comment_created("test/repo", 43)
            .issue_updated("test/repo", 42)
            .total_calls(3);
    }

    #[test]
    fn test_individual_assertions() {
        let context = TestContext::with_installation_id(67890);
        assert_installation_id(&context, 67890);
        assert_has_installation_id(&context);
        assert_no_github_client(&context);

        let event = MockWebhookEvent::issue_opened("test/repo", 42).build();
        assert_event_type(&event, WebhookEventType::Issues);
    }

    #[tokio::test]
    async fn test_api_call_verification() {
        let client = MockGitHubClient::new();
        
        // Initially no calls
        assert_no_api_calls(&client);
        assert_total_api_calls(&client, 0);

        // Make a call
        let _ = client.create_issue_comment("test/repo", 42, "Test").await;
        
        assert_api_call_made(&client, "POST", "/repos/test/repo/issues/42/comments");
        assert_api_call_count(&client, "POST", "/repos/test/repo/issues/42/comments", 1);
        assert_total_api_calls(&client, 1);

        // Make the same call again
        let _ = client.create_issue_comment("test/repo", 42, "Test 2").await;
        assert_api_call_count(&client, "POST", "/repos/test/repo/issues/42/comments", 2);
        assert_total_api_calls(&client, 2);
    }
}