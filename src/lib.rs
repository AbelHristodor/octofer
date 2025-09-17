//! # Octofer - GitHub Apps Framework for Rust
//!
//! Octofer is a framework for building GitHub Apps in Rust, inspired by Probot.
//! It provides a clean, type-safe way to build GitHub Apps with minimal boilerplate.
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use octofer::{Octofer, Config};
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     // Use default configuration for testing
//!     let config = Config::default();
//!     let mut app = Octofer::new(config).await.unwrap_or_else(|_| {
//!         Octofer::new_default()
//!     });
//!     
//!     app.on_issues(|context| async move {
//!         println!("Issue event received: {:?}", context.payload());
//!         Ok(())
//!     }).await;
//!     
//!     app.start().await?;
//!     Ok(())
//! }
//! ```

pub mod config;
pub mod core;
pub mod github;
pub mod webhook;

pub use config::Config;
pub use core::{Context, EventHandler, EventHandlerFn};
use std::sync::Arc;

use crate::webhook::WebhookServer;
use anyhow::Result;

/// Main Octofer application
#[derive(Default)]
pub struct Octofer {
    server: WebhookServer,
    config: Config,
}

impl Octofer {
    /// Create a new Octofer instance with the provided configuration
    pub async fn new(config: Config) -> Result<Self> {
        let server = WebhookServer::new(
            config.server.host,
            config.server.port,
            config.github.clone(),
        )
        .await?;

        Ok(Octofer {
            config: config.clone(),
            server,
        })
    }

    /// Create a new Octofer instance with default configuration
    pub fn new_default() -> Self {
        let config = Config::default();
        Octofer {
            server: WebhookServer::new_default(),
            config,
        }
    }

    /// Start the application server
    pub async fn start(&self) -> Result<()> {
        self.server.start().await
    }

    /// Add an issue comment event handler
    pub async fn on_issue_comment<F, Fut, E>(&mut self, handler: F, extra: Arc<E>) -> &Self
    where
        F: Fn(Context, Arc<E>) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Result<()>> + Send + 'static,
        E: Send + Sync + 'static + Clone,
    {
        self.server.on("issue_comment", handler, extra).await;
        self
    }

    /// Get access to the configuration
    pub fn config(&self) -> &Config {
        &self.config
    }
}
