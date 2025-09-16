//! # Basic Octofer Example
//!
//! This example shows how to create a simple GitHub App using Octofer.

use anyhow::Result;
use octofer::{Config, Octofer};
use tracing::info;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_target(false)
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .or_else(|_| EnvFilter::try_new("info"))
                .unwrap(),
        )
        .compact()
        .init();

    info!("Starting Octofer app: example-github-app");

    // Create a new Octofer app with default configuration
    let config = Config::default();
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
