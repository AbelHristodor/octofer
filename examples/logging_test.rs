//! # Logging Configuration Example
//!
//! This example demonstrates the logging configuration functionality.

use anyhow::Result;
use octofer::Config;
use tracing::{debug, error, info, trace, warn};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging with configuration from environment or defaults
    let config = Config::from_env().unwrap_or_else(|_| Config::default());
    config.init_logging();

    info!("Starting logging configuration test");
    info!("Current configuration:");
    info!("  Log level: {}", config.logging.level);
    info!("  Log format: {}", config.logging.format);
    info!("  With target: {}", config.logging.with_target);
    info!("  With file: {}", config.logging.with_file);
    info!("  With thread IDs: {}", config.logging.with_thread_ids);

    // Test different log levels
    trace!("This is a trace message");
    debug!("This is a debug message");
    info!("This is an info message");
    warn!("This is a warning message");
    error!("This is an error message");

    info!("Logging test completed successfully!");

    Ok(())
}
