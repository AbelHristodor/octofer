//! # GitHub Client Context Example
//!
//! This example demonstrates how to access the GitHub client from within
//! event handlers through the Context object.

use anyhow::Result;
use octofer::{Octofer, Config};
use tracing::{info, warn};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    info!("🚀 GitHub Client Context Example");

    // Try to create app with configuration from environment
    let app_result = if let Ok(config) = Config::from_env() {
        info!("✅ Using configuration from environment variables");
        Octofer::new(config).await
    } else {
        warn!("⚠️ No environment configuration found, using default (GitHub client will not be available)");
        Ok(Octofer::new_default())
    };

    let mut app = app_result.unwrap_or_else(|e| {
        warn!("Failed to create app with config: {}. Using default.", e);
        Octofer::new_default()
    });

    // Handle issue events
    app.on_issues(|context| async move {
        info!("📝 Issues event received!");
        
        demonstrate_context_access(context.clone());
        
        Ok(())
    }).await;

    // Handle issue comment events
    app.on_issue_comment(|context| async move {
        info!("💬 Issue comment event received!");
        
        demonstrate_context_access(context.clone());
        
        // Example: Process comment content
        if let Some(comment) = context.payload().get("comment") {
            if let Some(body) = comment.get("body").and_then(|b| b.as_str()) {
                info!("Comment content: {}", body);
                
                // Example: Respond to slash commands
                if body.starts_with("/help") {
                    info!("🆘 Help command detected");
                    
                    if context.github().is_some() {
                        info!("📡 GitHub client available - you could respond to this comment");
                        // In a real app, you would use context.installation_client().await 
                        // to get an authenticated client and respond
                    }
                }
                
                if body.starts_with("/status") {
                    info!("📊 Status command detected");
                    
                    if context.github().is_some() {
                        info!("📡 GitHub client available - you could provide status information");
                    }
                }
            }
        }
        
        Ok(())
    }).await;

    info!("🌐 Server starting...");
    info!("📋 Available context methods:");
    info!("   - context.github() -> Option<&Arc<GitHubClient>>");
    info!("   - context.installation_client() -> Result<Option<Octocrab>>");
    info!("   - context.payload() -> &serde_json::Value");
    info!("   - context.event_type() -> &str");
    info!("   - context.installation_id() -> Option<u64>");
    
    if std::env::var("GITHUB_APP_ID").is_err() {
        warn!("💡 Tip: Set GITHUB_APP_ID and GITHUB_PRIVATE_KEY_* environment variables");
        warn!("    to enable GitHub API access in event handlers");
    }

    app.start().await?;

    Ok(())
}

/// Demonstrates various ways to access the GitHub client from the context
fn demonstrate_context_access(context: octofer::Context) {
    info!("🔍 Demonstrating GitHub client access:");
    
    // Method 1: Check if GitHub client is available
    if let Some(_github_client) = context.github() {
        info!("✅ GitHub client is available");
        info!("🔧 You can use github_client.get_installations() for app-level operations");
        info!("🔧 You can use github_client.app_client() for direct octocrab access");
        
        // Note: We don't call the async methods here to avoid Sync issues
        // In real usage, you would spawn a task or use the client directly
        
    } else {
        warn!("❌ No GitHub client available");
        warn!("   This usually means no authentication was configured");
    }
    
    // Method 2: Check installation ID
    if let Some(installation_id) = context.installation_id() {
        info!("🏢 Installation ID: {}", installation_id);
        info!("🔧 You can use context.installation_client().await to get an authenticated client");
        
        // The installation client can be used for any GitHub API operations
        // that the app has permissions for in this specific installation
        info!("🎯 Installation-specific operations would be available");
        
    } else {
        info!("ℹ️ No installation ID in this event");
    }
    
    // Method 3: Event payload information
    info!("📄 Event payload available:");
    info!("   Event type: {}", context.event_type());
    
    if context.payload() != &serde_json::Value::Null {
        info!("   Payload: <structured GitHub event data>");
        
        // Example: Extract repository information
        if let Some(repo) = context.payload().get("repository") {
            if let Some(name) = repo.get("full_name").and_then(|n| n.as_str()) {
                info!("   Repository: {}", name);
            }
        }
    } else {
        info!("   Payload: <empty - using default context>");
    }
}