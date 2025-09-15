//! # Basic Octofer Example
//! 
//! This example shows how to create a simple GitHub App using Octofer.

use octofer::Octofer;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Create a new Octofer app
    let app = Octofer::new("example-github-app").await?;

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