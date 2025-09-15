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
//!     app.on_issues(|context| async move {
//!         println!("Issue event received: {:?}", context.payload());
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

use std::sync::Arc;
use anyhow::Result;

/// Main Octofer application struct
pub struct Octofer {
    inner: Arc<OctoferInner>,
}

struct OctoferInner {
    app_name: String,
    // Will contain event handlers, GitHub client, etc.
}

impl Octofer {
    /// Create a new Octofer application
    pub async fn new(app_name: impl Into<String>) -> Result<Self> {
        let inner = OctoferInner {
            app_name: app_name.into(),
        };

        Ok(Self {
            inner: Arc::new(inner),
        })
    }

    /// Start the application
    pub async fn start(&self) -> Result<()> {
        tracing::info!("Starting Octofer app: {}", self.inner.app_name);
        // Implementation will go here
        Ok(())
    }
}