//! # Basic Octofer Example
//!
//! This example shows how to create a simple GitHub App using Octofer.

use anyhow::Result;
use octofer::Octofer;
use tracing::info;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    info!("Started example 'basic'");

    // Initialize tracing
    // Setup tracing subscriber
    tracing_subscriber::fmt()
        .with_target(false)
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .or_else(|_| EnvFilter::try_new("octofer=info,tower_http=debug"))
                .unwrap(),
        )
        .compact()
        .init();

    // Create a new Octofer app
    let mut app = Octofer::new();
    app.on_issue_comment(|context| async move {
        println!("I'm in the handler!");
        println!("Context: {:?}", context);
        Ok(())
    })
    .await;

    // The app framework will be extended to support event handlers like:
    //
    // app.on_issues(|context| async move {
    //     println!("Issue event received: {:?}", context.payload());
    //     Ok(())
    // });
    //
    // app.on_pull_request(|context| async move {
    //     println!("Pull request event received: {:?}", context.payload());
    //     Ok(())
    // });

    // Start the application
    app.start().await?;

    Ok(())
}
