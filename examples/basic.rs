//! # Basic Octofer Example
//!
//! This example shows how to create a simple GitHub App using Octofer.

use std::sync::Arc;

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

    let cors_layer = tower_http::cors::CorsLayer::new()
        .allow_origin(tower_http::cors::Any)
        .allow_methods(tower_http::cors::Any)
        .allow_headers(tower_http::cors::Any);

    app.server.add_middleware(cors_layer)?;

    #[derive(Clone, Debug)]
    struct Hello {
        a: String,
    }

    let h = Hello { a: "Hello".into() };

    app.on_issue_comment(
        |context, e| async move {
            info!("Issue comment event received!");
            info!("Event type: {}", context.kind());
            info!("Installation ID: {:?}", context.installation_id());

            info!("Extra: {:?}", e.a);

            let client = match context.github_client {
                Some(c) => c,
                None => panic!(),
            };

            Ok(())
        },
        Arc::new(h),
    )
    .await;
    // Start the application
    app.start().await?;

    Ok(())
}
