//! # Complete Issue Comment Handler Example
//!
//! This example shows a simplified issue comment handler using the new Octofer API.

use anyhow::Result;
use octofer::{Octofer, Config};
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

        // Parse comment information from payload
        if let Some(comment) = context.payload().get("comment") {
            if let Some(body) = comment.get("body").and_then(|b| b.as_str()) {
                info!("💬 Comment: {}", body);

                // Example responses to different comment types
                if body.to_lowercase().contains("help") {
                    info!("🆘 Help request detected!");
                }

                if body.contains("@bot") || body.contains("@octofer") {
                    info!("🤖 Bot mention detected!");
                }

                if body.to_lowercase().contains("bug") {
                    info!("🐛 Bug report detected!");
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
    info!("⚠️  This example shows event processing - configure webhooks for real integration");

    app.start().await?;

    Ok(())
}
