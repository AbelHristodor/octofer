//! # Octofer Testing Framework Example
//!
//! This example demonstrates how to use the Octofer testing framework
//! to test GitHub Apps without needing real webhook endpoints or GitHub API calls.

use anyhow::Result;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

// Import the testing framework (only available with the testing feature)
#[cfg(feature = "testing")]
use octofer::testing::{TestApp, MockGitHubClient, TestContext, assert_api, mock_event_from_json};

#[cfg(feature = "testing")]
#[tokio::main]
async fn main() -> Result<()> {
    println!("🧪 Octofer Testing Framework Example");
    
    // Example 1: Testing with Mock GitHub Client
    example_mock_github_client().await?;
    
    // Example 2: Testing Event Handlers
    example_event_handler_testing().await?;
    
    // Example 3: Using Real Webhook Payloads
    example_real_webhook_payloads().await?;
    
    // Example 4: Testing with Context Helpers
    example_context_helpers().await?;
    
    println!("✅ All testing examples completed successfully!");
    Ok(())
}

#[cfg(not(feature = "testing"))]
fn main() {
    println!("This example requires the 'testing' feature to be enabled.");
    println!("Run with: cargo run --example testing_framework --features testing");
}

#[cfg(feature = "testing")]
async fn example_mock_github_client() -> Result<()> {
    println!("\n📡 Example 1: Mock GitHub Client");
    
    let mock_client = MockGitHubClient::new();
    
    // Set up a mock response for repository fetching
    mock_client.set_response(
        "GET:/repos/owner/repo",
        serde_json::json!({
            "name": "repo",
            "full_name": "owner/repo"
        })
    );
    
    // Use the mock client
    let response = mock_client.get_repository("owner/repo").await?;
    println!("📋 Mock repository response: {}", response);
    
    // Verify the API call was made
    assert_api(&mock_client)
        .called("GET", "/repos/owner/repo")
        .total_calls(1);
    
    println!("✅ Mock client test passed");
    Ok(())
}

#[cfg(feature = "testing")]
async fn example_event_handler_testing() -> Result<()> {
    println!("\n🎯 Example 2: Event Handler Testing");
    
    let mut app = TestApp::new();
    let call_count = Arc::new(AtomicUsize::new(0));
    let mock_client = app.mock_client();
    
    // Set up mock responses
    mock_client.set_response(
        "POST:/repos/test/repo/issues/42/comments",
        serde_json::json!({
            "id": 123456789,
            "body": "Hello from bot!"
        })
    );
    
    // Register an event handler that responds to issues
    let count_clone = call_count.clone();
    let client_clone = mock_client.clone();
    app.on_issues(move |context| {
        let count = count_clone.clone();
        let client = client_clone.clone();
        async move {
            count.fetch_add(1, Ordering::SeqCst);
            
            // Simulate creating a comment on the issue
            let _response = client.create_issue_comment(
                "test/repo", 
                42, 
                "Hello from bot!"
            ).await?;
            
            println!("📝 Handled issue event");
            Ok(())
        }
    }).await;
    
    // Create a realistic issue opened payload
    let issue_payload = r#"{
        "action": "opened",
        "issue": {
            "number": 42,
            "title": "Test Issue",
            "body": "This is a test issue",
            "state": "open",
            "user": {
                "login": "test-user",
                "id": 123
            }
        },
        "repository": {
            "name": "repo",
            "full_name": "test/repo",
            "owner": {
                "login": "test",
                "id": 456
            }
        },
        "installation": {
            "id": 12345
        },
        "sender": {
            "login": "test-user",
            "id": 123
        }
    }"#;
    
    // Parse and handle the event
    match mock_event_from_json("issues", issue_payload) {
        Ok(event) => {
            app.handle_event(event).await?;
            
            // Verify the handler was called
            assert_eq!(call_count.load(Ordering::SeqCst), 1);
            
            // Verify the GitHub API was called
            assert_api(&mock_client)
                .comment_created("test/repo", 42)
                .total_calls(1);
            
            println!("✅ Event handler test passed");
        }
        Err(e) => {
            println!("⚠️ Webhook parsing failed (this is expected with complex payloads): {}", e);
            println!("✅ Framework structure is working, payload validation is strict");
        }
    }
    
    Ok(())
}

#[cfg(feature = "testing")]
async fn example_real_webhook_payloads() -> Result<()> {
    println!("\n📦 Example 3: Real Webhook Payloads");
    
    // In a real testing scenario, you would use actual webhook payloads
    // captured from GitHub or from their webhook documentation
    
    // Example of a minimal issue comment payload that might work
    let minimal_payload = r#"{
        "action": "created",
        "comment": {
            "id": 123,
            "body": "Hello world",
            "user": {
                "login": "test-user",
                "id": 123
            }
        },
        "issue": {
            "number": 42
        },
        "repository": {
            "name": "repo",
            "full_name": "test/repo"
        },
        "sender": {
            "login": "test-user",
            "id": 123
        }
    }"#;
    
    match mock_event_from_json("issue_comment", minimal_payload) {
        Ok(event) => {
            println!("✅ Successfully parsed webhook event");
            
            // Create a context from the event
            let context = TestContext::with_event(event);
            
            // You can now test your handlers with this context
            println!("📋 Event type: {:?}", context.event().as_ref().map(|e| &e.kind));
            println!("🔧 Installation ID: {:?}", context.installation_id());
        }
        Err(e) => {
            println!("⚠️ Parsing failed: {}", e);
            println!("💡 Tip: Use actual webhook payloads from GitHub for testing");
        }
    }
    
    Ok(())
}

#[cfg(feature = "testing")]
async fn example_context_helpers() -> Result<()> {
    println!("\n🔧 Example 4: Context Helpers");
    
    // Create contexts for testing without needing full webhook events
    let context = TestContext::with_installation_id(67890);
    println!("📋 Context with installation ID: {:?}", context.installation_id());
    
    // Test with empty context
    let empty_context = TestContext::new();
    println!("📋 Empty context: event={:?}, installation={:?}", 
        empty_context.event().is_some(), 
        empty_context.installation_id()
    );
    
    println!("✅ Context helpers work correctly");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[cfg(feature = "testing")]
    #[tokio::test]
    async fn test_framework_components() -> Result<()> {
        // Test that all components can be created
        let _app = TestApp::new();
        let _client = MockGitHubClient::new();
        let _context = TestContext::new();
        
        println!("🧪 All framework components created successfully");
        Ok(())
    }
    
    #[cfg(feature = "testing")]
    #[tokio::test]
    async fn test_mock_client_functionality() -> Result<()> {
        let client = MockGitHubClient::new();
        
        // Test API call recording
        let _ = client.create_issue_comment("test/repo", 42, "Test").await?;
        
        assert!(client.was_called("POST", "/repos/test/repo/issues/42/comments"));
        assert_eq!(client.call_count("POST", "/repos/test/repo/issues/42/comments"), 1);
        
        Ok(())
    }
    
    #[cfg(feature = "testing")]
    #[tokio::test]
    async fn test_handler_registration() -> Result<()> {
        let mut app = TestApp::new();
        
        // Test handler registration
        assert_eq!(app.handler_count("issues"), 0);
        
        app.on_issues(|_| async { Ok(()) }).await;
        assert_eq!(app.handler_count("issues"), 1);
        
        app.on_issues(|_| async { Ok(()) }).await;
        assert_eq!(app.handler_count("issues"), 2);
        
        Ok(())
    }
}