//! # Issue Comment Event Handler Example
//!
//! This example demonstrates how to handle issue comment events using Octofer.

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
        info!("Issue comment event received!");
        info!("Event type: {}", context.event_type());

        if let Some(installation_id) = context.installation_id() {
            info!("Installation ID: {}", installation_id);
        }

        // Extract information from the event payload
        let payload = context.payload();

        if let Some(action) = payload.get("action").and_then(|a| a.as_str()) {
            info!("Action: {}", action);
        }

        if let Some(repository) = payload.get("repository") {
            if let Some(full_name) = repository.get("full_name").and_then(|n| n.as_str()) {
                info!("Repository: {}", full_name);
            }
        }

        if let Some(issue) = payload.get("issue") {
            if let Some(number) = issue.get("number").and_then(|n| n.as_u64()) {
                info!("Issue #{}", number);
            }
            if let Some(title) = issue.get("title").and_then(|t| t.as_str()) {
                info!("Issue title: {}", title);
            }
        }

        if let Some(comment) = payload.get("comment") {
            if let Some(body) = comment.get("body").and_then(|b| b.as_str()) {
                info!("Comment: {}", body);

                // Example: Respond to bot mentions
                if body.contains("@bot") {
                    info!("Bot was mentioned! Could respond here.");
                }
            }

            if let Some(user) = comment.get("user") {
                if let Some(login) = user.get("login").and_then(|l| l.as_str()) {
                    info!("Comment author: {}", login);
                }
            }
        }

        Ok(())
    })
    .await;

    // Start the application
    info!("Starting issue comment handler...");
    app.start().await?;

    Ok(())
}
