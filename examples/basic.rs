//! # Basic Octofer Example
//!
//! This example shows how to create a simple GitHub App using Octofer.

use anyhow::Result;
use octofer::{Config, Octofer};
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing using configuration
    let config = Config::from_env().unwrap_or_else(|_| {
        // Fallback to default configuration if environment variables are not set
        Config::default()
    });

    // Initialize logging based on configuration
    config.init_logging();

    info!("Starting Octofer app: example-github-app");

    // Create a new Octofer app with the configuration
    let mut app = Octofer::new(config).await.unwrap_or_else(|_| {
        info!("Failed to create app with config, using default");
        Octofer::new_default()
    });

    app.on_issue_comment(|context| async move {
        info!("Issue comment event received!");
        info!("Event type: {}", context.event_type());
        info!("Installation ID: {:?}", context.installation_id());

        let client = match context.github_client {
            Some(c) => c,
            None => panic!(),
        };

        Ok(())
    })
    .await;

    app.on_issues(|context| async move {
        info!("Issues event received!");
        info!("Event type: {}", context.event_type());
        info!("Installation ID: {:?}", context.installation_id());
        Ok(())
    })
    .await;

    // Start the application
    app.start().await?;

    Ok(())
}
