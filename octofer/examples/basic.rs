//! # Basic Octofer Example
//!
//! This example shows how to create a simple GitHub App using Octofer.

use anyhow::Result;
use octofer::Octofer;
use tracing::info;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    // Setup tracing subscriber
    tracing_subscriber::fmt()
        .with_target(false)
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .or_else(|_| EnvFilter::try_new("info"))
                .unwrap(),
        )
        .compact()
        .init();

    info!("Started example 'basic'");

    // Create a new Octofer app
    let mut app = Octofer::new();
    app.on_issue_comment(|context| async move {
        info!("I'm in the handler!");
        info!("Context: {:?}", context);
        Ok(())
    })
    .await;

    // Start the application
    app.start().await?;

    Ok(())
}
