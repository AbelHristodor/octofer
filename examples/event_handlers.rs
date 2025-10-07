//! Example demonstrating various event handlers
//!
//! This example shows how to register handlers for different types of GitHub webhook events.

use octofer::{Config, Context, Octofer};
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load configuration from environment
    let config = Config::from_env().unwrap_or_default();
    config.init_logging();

    // Create the app with default settings if GitHub config is missing
    let mut app = Octofer::new(config)
        .await
        .unwrap_or_else(|_| Octofer::new_default());

    // Issue events
    app.on_issue(
        |context: Context, _extra: Arc<()>| async move {
            println!("Issue event: {}", context.kind());
            Ok(())
        },
        Arc::new(()),
    )
    .await;

    // Repository events
    app.on_push(
        |context: Context, _extra: Arc<()>| async move {
            println!("Push event: {}", context.kind());
            Ok(())
        },
        Arc::new(()),
    )
    .await;

    app.on_fork(
        |context: Context, _extra: Arc<()>| async move {
            println!("Fork event: {}", context.kind());
            Ok(())
        },
        Arc::new(()),
    )
    .await;

    // Workflow events
    app.on_workflow_run(
        |context: Context, _extra: Arc<()>| async move {
            println!("Workflow run event: {}", context.kind());
            Ok(())
        },
        Arc::new(()),
    )
    .await;

    // Security events
    app.on_dependabot_alert(
        |context: Context, _extra: Arc<()>| async move {
            println!("Dependabot alert event: {}", context.kind());
            Ok(())
        },
        Arc::new(()),
    )
    .await;

    // Deployment events
    app.on_deployment(
        |context: Context, _extra: Arc<()>| async move {
            println!("Deployment event: {}", context.kind());
            Ok(())
        },
        Arc::new(()),
    )
    .await;

    // Discussion events
    app.on_discussion(
        |context: Context, _extra: Arc<()>| async move {
            println!("Discussion event: {}", context.kind());
            Ok(())
        },
        Arc::new(()),
    )
    .await;

    // Release events
    app.on_release(
        |context: Context, _extra: Arc<()>| async move {
            println!("Release event: {}", context.kind());
            Ok(())
        },
        Arc::new(()),
    )
    .await;

    // Team events
    app.on_team(
        |context: Context, _extra: Arc<()>| async move {
            println!("Team event: {}", context.kind());
            Ok(())
        },
        Arc::new(()),
    )
    .await;

    // Miscellaneous events
    app.on_star(
        |context: Context, _extra: Arc<()>| async move {
            println!("Star event: {}", context.kind());
            Ok(())
        },
        Arc::new(()),
    )
    .await;

    app.on_ping(
        |context: Context, _extra: Arc<()>| async move {
            println!("Ping event: {}", context.kind());
            Ok(())
        },
        Arc::new(()),
    )
    .await;

    println!("Starting webhook server with multiple event handlers...");
    app.start().await?;

    Ok(())
}
