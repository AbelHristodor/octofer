//! # GitHub Client Context Example
//!
//! This example demonstrates how to access the GitHub client from within
//! event handlers through the Context object.

use anyhow::Result;
use octofer::{Config, Octofer};
use tracing::{info, warn};

#[tokio::main]
async fn main() -> Result<()> {
    info!("🚀 GitHub Client Context Example");

    // Try to create app with configuration from environment
    let app_result = if let Ok(config) = Config::from_env() {
        info!("✅ Using configuration from environment variables");
        // Initialize logging with the environment configuration
        config.init_logging();
        Octofer::new(config).await
    } else {
        warn!("⚠️ No environment configuration found, using default (GitHub client will not be available)");
        // Initialize logging with default configuration
        let config = Config::default();
        config.init_logging();
        Ok(Octofer::new_default())
    };

    let mut app = app_result.unwrap_or_else(|e| {
        warn!("Failed to create app with config: {}. Using default.", e);
        Octofer::new_default()
    });

    // Handle issue events
    app.on_issues(|context| async move {
        info!("📝 Issues event received!");

        let client = if let Some(gh) = context.installation_client().await.unwrap() {
            gh
        } else {
            panic!("Cannot get gh client!");
        };

        Ok(())
    })
    .await;

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
