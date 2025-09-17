# Octofer Testing Framework

The Octofer testing framework provides comprehensive tools for testing GitHub Apps built with Octofer without requiring real GitHub webhooks or API endpoints.

## Features

- 🎯 **TestApp**: Simplified test application for unit testing event handlers
- 📡 **MockGitHubClient**: Mock GitHub API client with call recording and response simulation
- 🔧 **TestContext**: Helpers for creating test contexts with mock data
- ✅ **Assertions**: Specialized assertion helpers for GitHub App testing
- 📦 **Mock Events**: Builders for creating webhook events (with support for real payloads)

## Getting Started

### Enable the Testing Feature

Add the testing feature to your Cargo.toml when running tests:

```toml
[dev-dependencies]
octofer = { path = ".", features = ["testing"] }
```

Or run tests with the feature enabled:

```bash
cargo test --features testing
```

### Basic Usage

```rust
#[cfg(test)]
mod tests {
    use octofer::testing::{TestApp, MockGitHubClient, TestContext, assert_api};
    use anyhow::Result;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;

    #[tokio::test]
    async fn test_issue_handler() -> Result<()> {
        let mut app = TestApp::new();
        let called = Arc::new(AtomicBool::new(false));
        
        let called_clone = called.clone();
        app.on_issues(move |_context| {
            let called = called_clone.clone();
            async move {
                called.store(true, Ordering::SeqCst);
                Ok(())
            }
        }).await;

        // Create a context and test the handler
        let context = TestContext::with_installation_id(12345);
        app.handle_context("issues", context).await?;
        
        assert!(called.load(Ordering::SeqCst));
        Ok(())
    }
}
```

## Components

### TestApp

The `TestApp` provides a simplified interface for testing event handlers without starting a real server.

```rust
use octofer::testing::TestApp;

let mut app = TestApp::new();

// Register event handlers
app.on_issues(|context| async move {
    println!("Issue event: {:?}", context.payload());
    Ok(())
}).await;

app.on_issue_comment(|context| async move {
    println!("Comment event: {:?}", context.payload());
    Ok(())
}).await;

// Test with mock context
let context = TestContext::with_installation_id(12345);
app.handle_context("issues", context).await?;
```

### MockGitHubClient

The `MockGitHubClient` simulates GitHub API interactions for testing.

```rust
use octofer::testing::{MockGitHubClient, assert_api};

let client = MockGitHubClient::new();

// Set up mock responses
client.set_response(
    "POST:/repos/owner/repo/issues/42/comments",
    serde_json::json!({
        "id": 123456,
        "body": "Test comment",
        "user": { "login": "test-bot" }
    })
);

// Use the client
let response = client.create_issue_comment("owner/repo", 42, "Test comment").await?;

// Verify API calls
assert_api(&client)
    .comment_created("owner/repo", 42)
    .total_calls(1);
```

#### Available Mock Methods

- `create_issue_comment(repo, issue_number, body)` - Create an issue comment
- `update_issue(repo, issue_number, title, body, state)` - Update an issue
- `add_labels_to_issue(repo, issue_number, labels)` - Add labels to an issue
- `get_repository(repo)` - Get repository information

### TestContext

Helper for creating test contexts with various configurations.

```rust
use octofer::testing::TestContext;

// Empty context
let context = TestContext::new();

// Context with installation ID
let context = TestContext::with_installation_id(12345);

// Context with mock event (if you have a parsed webhook event)
let context = TestContext::with_event(webhook_event);
```

### Assertions

Specialized assertions for GitHub App testing.

```rust
use octofer::testing::{assert_api, assert_context};

// API assertions
assert_api(&mock_client)
    .comment_created("owner/repo", 42)
    .issue_updated("owner/repo", 42)
    .total_calls(2);

// Context assertions
assert_context(&context)
    .installation_id(12345)
    .has_installation_id()
    .no_github_client();
```

## Testing with Real Webhook Payloads

For the most realistic testing, use actual webhook payloads from GitHub:

```rust
use octofer::testing::mock_event_from_json;

// Use a real webhook payload from GitHub's documentation
let payload = r#"{
    "action": "opened",
    "issue": {
        "id": 1,
        "node_id": "MDU6SXNzdWUx",
        "number": 42,
        "title": "Test Issue",
        "user": {
            "login": "octocat",
            "id": 1,
            "node_id": "MDQ6VXNlcjE=",
            "type": "User",
            "site_admin": false
        },
        "state": "open",
        "body": "I'm having a problem with this."
    },
    "repository": {
        "id": 1296269,
        "node_id": "MDEwOlJlcG9zaXRvcnkxMjk2MjY5",
        "name": "Hello-World",
        "full_name": "octocat/Hello-World",
        "owner": {
            "login": "octocat",
            "id": 1,
            "node_id": "MDQ6VXNlcjE=",
            "type": "User",
            "site_admin": false
        }
    },
    "installation": {
        "id": 12345
    },
    "sender": {
        "login": "octocat",
        "id": 1,
        "node_id": "MDQ6VXNlcjE=",
        "type": "User",
        "site_admin": false
    }
}"#;

// Parse and use the webhook event
if let Ok(event) = mock_event_from_json("issues", payload) {
    let context = TestContext::with_event(event);
    app.handle_context("issues", context).await?;
}
```

## Complete Example

Here's a complete example testing a GitHub App that responds to issue comments:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use octofer::testing::{TestApp, assert_api};
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    #[tokio::test]
    async fn test_issue_comment_bot() -> Result<()> {
        let mut app = TestApp::new();
        let mock_client = app.mock_client();
        let response_count = Arc::new(AtomicUsize::new(0));

        // Set up mock response
        mock_client.set_response(
            "POST:/repos/test/repo/issues/42/comments",
            serde_json::json!({
                "id": 987654321,
                "body": "Thanks for the issue!",
                "user": { "login": "my-bot" }
            })
        );

        // Register handler that responds to comments containing "help"
        let count_clone = response_count.clone();
        let client_clone = mock_client.clone();
        app.on_issue_comment(move |context| {
            let count = count_clone.clone();
            let client = client_clone.clone();
            async move {
                if let Some(event) = context.event() {
                    // In a real app, you'd extract the comment body from the event
                    // For this test, we'll simulate finding "help" in the comment
                    count.fetch_add(1, Ordering::SeqCst);
                    
                    let _response = client.create_issue_comment(
                        "test/repo",
                        42,
                        "Thanks for the issue!"
                    ).await?;
                }
                Ok(())
            }
        }).await;

        // Simulate handling an issue comment event
        let context = TestContext::with_installation_id(12345);
        app.handle_context("issue_comment", context).await?;

        // Verify the bot responded
        assert_eq!(response_count.load(Ordering::SeqCst), 1);
        
        // Verify the GitHub API was called correctly
        assert_api(&mock_client)
            .comment_created("test/repo", 42)
            .total_calls(1);

        Ok(())
    }
}
```

## Running Tests

```bash
# Run all tests with the testing feature
cargo test --features testing

# Run specific test
cargo test test_issue_comment_bot --features testing

# Run the testing framework example
cargo run --example testing_framework --features testing
```

## Tips for Effective Testing

1. **Use Real Payloads**: When possible, use actual webhook payloads from GitHub's documentation or captured from real webhooks.

2. **Test Error Conditions**: Test how your app handles malformed events, missing fields, and API errors.

3. **Mock External Dependencies**: Use the MockGitHubClient to avoid making real API calls during tests.

4. **Test Handler Logic**: Focus on testing the business logic of your event handlers rather than the framework itself.

5. **Verify API Interactions**: Use the assertion helpers to ensure your app makes the expected GitHub API calls.

## Limitations

- The mock webhook event builders create simplified events that may not pass octocrab's strict validation
- For comprehensive testing, use real webhook payloads from GitHub
- The framework is designed for unit testing; integration testing may require additional setup

## Contributing

The testing framework is designed to be extensible. You can:

- Add new mock methods to `MockGitHubClient`
- Create additional assertion helpers
- Extend `TestContext` with more helper methods
- Add support for additional webhook event types

See the source code in `src/testing/` for implementation details.