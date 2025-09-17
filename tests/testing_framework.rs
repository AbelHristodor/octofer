//! Integration tests for the Octofer testing framework
//!
//! These tests demonstrate how developers would use the testing framework
//! to test their GitHub Apps.

#[cfg(feature = "testing")]
mod testing_framework_integration {
    use anyhow::Result;
    use octofer::testing::{TestApp, MockGitHubClient, TestContext, assert_api};
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::sync::Arc;

    #[tokio::test]
    async fn test_mock_github_client_functionality() -> Result<()> {
        let client = MockGitHubClient::new();

        // Test basic API call recording
        let response = client.create_issue_comment("test/repo", 42, "Hello").await?;
        
        assert!(client.was_called("POST", "/repos/test/repo/issues/42/comments"));
        assert_eq!(client.call_count("POST", "/repos/test/repo/issues/42/comments"), 1);
        
        // Verify response structure
        assert!(response.is_object());
        assert_eq!(response["body"], "Hello");

        Ok(())
    }

    #[tokio::test]
    async fn test_mock_client_with_custom_responses() -> Result<()> {
        let client = MockGitHubClient::new();
        
        // Set up a custom response
        client.set_response(
            "GET:/repos/test/repo",
            serde_json::json!({
                "name": "repo",
                "full_name": "test/repo"
            })
        );

        let response = client.get_repository("test/repo").await?;
        
        assert_eq!(response["name"], "repo");
        assert_eq!(response["full_name"], "test/repo");

        Ok(())
    }

    #[tokio::test]
    async fn test_event_handler_registration_and_execution() -> Result<()> {
        let mut app = TestApp::new();
        let handler_called = Arc::new(AtomicBool::new(false));
        let call_count = Arc::new(AtomicUsize::new(0));

        // Test multiple handler registration
        let called_clone = handler_called.clone();
        app.on_issues(move |_context| {
            let called = called_clone.clone();
            async move {
                called.store(true, Ordering::SeqCst);
                Ok(())
            }
        }).await;

        let count_clone = call_count.clone();
        app.on_issues(move |_context| {
            let count = count_clone.clone();
            async move {
                count.fetch_add(1, Ordering::SeqCst);
                Ok(())
            }
        }).await;

        // Verify handler counts
        assert_eq!(app.handler_count("issues"), 2);
        assert!(app.has_handlers("issues"));
        assert!(!app.has_handlers("pull_request"));

        // Execute handlers
        let context = TestContext::with_installation_id(12345);
        app.handle_context("issues", context).await?;

        // Verify handlers were called
        assert!(handler_called.load(Ordering::SeqCst));
        assert_eq!(call_count.load(Ordering::SeqCst), 1);

        Ok(())
    }

    #[tokio::test]
    async fn test_github_app_issue_responder() -> Result<()> {
        // This test simulates a complete GitHub App that responds to issues
        let mut app = TestApp::new();
        let mock_client = app.mock_client();

        // Set up mock responses
        mock_client.set_response(
            "POST:/repos/test/repo/issues/42/comments",
            serde_json::json!({
                "id": 123456789,
                "body": "Thank you for reporting this issue!"
            })
        );

        mock_client.set_response(
            "POST:/repos/test/repo/issues/42/labels",
            serde_json::json!([
                { "name": "needs-triage" }
            ])
        );

        // Register an issues handler
        let client_clone = mock_client.clone();
        app.on_issues(move |context| {
            let client = client_clone.clone();
            async move {
                // Simulate processing an "opened" issue
                if let Some(installation_id) = context.installation_id() {
                    // Create a comment
                    let _comment = client.create_issue_comment(
                        "test/repo",
                        42,
                        "Thank you for reporting this issue!"
                    ).await?;

                    // Add a label
                    let _labels = client.add_labels_to_issue(
                        "test/repo",
                        42,
                        &["needs-triage"]
                    ).await?;

                    println!("Processed issue for installation {}", installation_id);
                }
                Ok(())
            }
        }).await;

        // Execute the handler
        let context = TestContext::with_installation_id(67890);
        app.handle_context("issues", context).await?;

        // Verify the correct API calls were made
        assert_api(&mock_client)
            .comment_created("test/repo", 42)
            .labels_added("test/repo", 42)
            .total_calls(2);

        Ok(())
    }

    #[tokio::test]
    async fn test_context_helpers() -> Result<()> {
        // Test empty context
        let empty_context = TestContext::new();
        assert!(empty_context.event().is_none());
        assert!(empty_context.installation_id().is_none());
        assert!(empty_context.github_client.is_none());

        // Test context with installation ID
        let context_with_id = TestContext::with_installation_id(12345);
        assert!(context_with_id.event().is_none());
        assert_eq!(context_with_id.installation_id(), Some(12345));
        assert!(context_with_id.github_client.is_none());

        Ok(())
    }

    #[tokio::test]
    async fn test_error_handling_in_handlers() -> Result<()> {
        let mut app = TestApp::new();
        let success_count = Arc::new(AtomicUsize::new(0));
        let error_count = Arc::new(AtomicUsize::new(0));

        // Register a handler that sometimes fails
        let success_clone = success_count.clone();
        let error_clone = error_count.clone();
        app.on_issues(move |context| {
            let success = success_clone.clone();
            let error = error_clone.clone();
            async move {
                if let Some(installation_id) = context.installation_id() {
                    if installation_id == 999 {
                        error.fetch_add(1, Ordering::SeqCst);
                        anyhow::bail!("Simulated error");
                    } else {
                        success.fetch_add(1, Ordering::SeqCst);
                    }
                }
                Ok(())
            }
        }).await;

        // Test successful execution
        let success_context = TestContext::with_installation_id(12345);
        app.handle_context("issues", success_context).await?;
        assert_eq!(success_count.load(Ordering::SeqCst), 1);
        assert_eq!(error_count.load(Ordering::SeqCst), 0);

        // Test error handling
        let error_context = TestContext::with_installation_id(999);
        let result = app.handle_context("issues", error_context).await;
        assert!(result.is_err());
        assert_eq!(success_count.load(Ordering::SeqCst), 1);
        assert_eq!(error_count.load(Ordering::SeqCst), 1);

        Ok(())
    }

    #[tokio::test]
    async fn test_multiple_event_types() -> Result<()> {
        let mut app = TestApp::new();
        let issue_calls = Arc::new(AtomicUsize::new(0));
        let comment_calls = Arc::new(AtomicUsize::new(0));
        let pr_calls = Arc::new(AtomicUsize::new(0));

        // Register handlers for different event types
        let issue_count = issue_calls.clone();
        app.on_issues(move |_| {
            let count = issue_count.clone();
            async move {
                count.fetch_add(1, Ordering::SeqCst);
                Ok(())
            }
        }).await;

        let comment_count = comment_calls.clone();
        app.on_issue_comment(move |_| {
            let count = comment_count.clone();
            async move {
                count.fetch_add(1, Ordering::SeqCst);
                Ok(())
            }
        }).await;

        let pr_count = pr_calls.clone();
        app.on_pull_request(move |_| {
            let count = pr_count.clone();
            async move {
                count.fetch_add(1, Ordering::SeqCst);
                Ok(())
            }
        }).await;

        // Test each event type
        let context = TestContext::with_installation_id(12345);
        
        app.handle_context("issues", context.clone()).await?;
        app.handle_context("issue_comment", context.clone()).await?;
        app.handle_context("pull_request", context).await?;

        // Verify each handler was called
        assert_eq!(issue_calls.load(Ordering::SeqCst), 1);
        assert_eq!(comment_calls.load(Ordering::SeqCst), 1);
        assert_eq!(pr_calls.load(Ordering::SeqCst), 1);

        Ok(())
    }
}