//! # Complete Issue Comment Handler Example
//!
//! This example shows how to use the GitHub client from within event handlers
//! to interact with the GitHub API in response to webhook events.

use anyhow::Result;
use octofer::{Config, Octofer};
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Create a new Octofer app with default configuration
    let config = Config::default();
    let mut app = Octofer::new(config).await.unwrap_or_else(|_| {
        info!("Failed to create app with config, using default");
        Octofer::new_default()
    });

    // Handle issue comment events
    app.on_issue_comment(|context| async move {
        info!("🎯 Issue comment event received!");
        info!("Event type: {}", context.event_type());

        if let Some(installation_id) = context.installation_id() {
            info!("Installation ID: {}", installation_id);
        }

        // Check if GitHub client is available
        if let Some(_github_client) = context.github() {
            info!("✅ GitHub client is available!");
            info!("🔧 You can use the GitHub client for API operations");

            // Example: App-level operations would be available
            info!("📋 App-level operations: get_installations(), app_client()");

            // Example: Installation-specific operations
            if context.installation_id().is_some() {
                info!("🏢 Installation client would be available via context.installation_client().await");
                info!("🎯 You could make authenticated API calls for this installation");
            }

        } else {
            info!("ℹ️ No GitHub client available (requires proper configuration)");
        }

        // Parse comment information from payload
        if let Some(comment) = context.payload().get("comment") {
            if let Some(body) = comment.get("body").and_then(|b| b.as_str()) {
                info!("💬 Comment: {}", body);

                // Example: Respond to specific commands
                if body.to_lowercase().contains("hello") {
                    info!("👋 Hello command detected!");


                    // ✅ FIXED: GitHub client now works properly in event handlers!
                    // The previous trait implementation error has been resolved.
                    // You can now use context.installation_client() without Sync issues.

                    // Example usage (uncomment when you have proper GitHub credentials):
                    // if let Ok(Some(client)) = context.installation_client().await {
                    //     if let (Some(repo_owner), Some(repo_name), Some(issue_number)) = 
                    //         (extract_repo_owner(&context), extract_repo_name(&context), extract_issue_number(&context)) {
                    //         let response = "Hello! 👋 Thanks for mentioning me!";
                    //         match client.issues(repo_owner, repo_name)
                    //             .create_comment(issue_number, response)
                    //             .await {
                    //             Ok(_) => info!("✅ Replied to comment"),
                    //             Err(e) => info!("❌ Failed to reply: {}", e),
                    //         }
                    //     }
                    // }

                    // Demonstrate that the API is now available:
                    match context.installation_client().await {
                        Ok(Some(_client)) => info!("✅ GitHub installation client is ready!"),
                        Ok(None) => info!("ℹ️ GitHub client not configured (set GITHUB_APP_ID, etc.)"),
                        Err(e) => info!("⚠️ Error getting installation client: {}", e),
                    }
                }

                if body.to_lowercase().contains("help") {
                    info!("🆘 Help request detected!");
                }

                if body.contains("@bot") || body.contains("@octofer") {
                    info!("🤖 Bot mention detected!");
                }
            }
        }

        if let Some(issue) = context.payload().get("issue") {
            if let Some(number) = issue.get("number").and_then(|n| n.as_u64()) {
                info!("🔢 Issue #{}", number);
            }
            if let Some(title) = issue.get("title").and_then(|t| t.as_str()) {
                info!("📝 Issue title: {}", title);
            }
        }

        Ok(())
    }).await;

    // Start the application
    info!("🚀 Starting Octofer issue comment bot...");
    info!("📡 Webhook server will listen for events");
    info!("🔑 To test with real GitHub integration, set these environment variables:");
    info!("   GITHUB_APP_ID=your_app_id");
    info!("   GITHUB_PRIVATE_KEY_PATH=path/to/private-key.pem");
    info!("   GITHUB_WEBHOOK_SECRET=your_webhook_secret");

    app.start().await?;

    Ok(())
}
