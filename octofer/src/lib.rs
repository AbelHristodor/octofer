//! # Octofer
//!
//! A framework for building GitHub Apps in Rust, inspired by Probot.
//!
//! ## Quick Start
//!
//! ```rust
//! use octofer::Octofer;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let app = Octofer::new("my-github-app").await?;
//!     
//!     app.on_issue_comment(|context| async move {
//!         println!("Issue comment event received: {:?}", context.payload());
//!         Ok(())
//!     });
//!     
//!     app.start().await?;
//!     Ok(())
//! }
//! ```

pub use octofer_core::*;
pub use octofer_github::*;
pub use octofer_webhook::*;

use anyhow::Result;
use std::sync::Arc;

/// Main Octofer application struct
pub struct Octofer {
    inner: Arc<OctoferInner>,
}

struct OctoferInner {
    app_name: String,
    webhook_server: std::sync::RwLock<WebhookServer>,
    github_client: GitHubClient,
}

impl Octofer {
    /// Create a new Octofer application
    pub async fn new(app_name: impl Into<String>) -> Result<Self> {
        let inner = OctoferInner {
            app_name: app_name.into(),
            webhook_server: std::sync::RwLock::new(WebhookServer::default()),
            github_client: GitHubClient::default(),
        };

        Ok(Self {
            inner: Arc::new(inner),
        })
    }

    /// Add an issue comment event handler
    pub fn on_issue_comment<F, Fut>(&self, handler: F) -> &Self
    where
        F: Fn(Context) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Result<()>> + Send + 'static,
    {
        let mut webhook_server = self.inner.webhook_server.write().unwrap();
        webhook_server.on("issue_comment", handler);
        self
    }

    /// Get the GitHub client for making API calls
    pub fn github(&self) -> &GitHubClient {
        &self.inner.github_client
    }

    /// Start the application
    #[allow(clippy::await_holding_lock)]
    pub async fn start(&self) -> Result<()> {
        tracing::info!("Starting Octofer app: {}", self.inner.app_name);
        // Read the webhook server and start it
        let webhook_server = self.inner.webhook_server.read().unwrap();
        webhook_server.start().await
    }
}
