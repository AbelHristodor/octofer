//! # Octofer - GitHub Apps Framework for Rust
//!
//! Octofer is a framework for building GitHub Apps in Rust, inspired by Probot.
//! It provides a clean, type-safe way to build GitHub Apps with minimal boilerplate
//! and automatic webhook handling.
//!
//! ## Key Features
//!
//! - **🔐 Automatic Authentication** - JWT token generation and installation token management
//! - **📡 Webhook Handling** - Built-in HTTP server with HMAC verification
//! - **🎯 Event Routing** - Type-safe event handlers for GitHub webhook events
//! - **🌐 GitHub API Integration** - Authenticated GitHub API client with caching
//! - **⚙️ Configuration Management** - Environment variable based configuration
//! - **📊 Observability** - Built-in logging and request tracing
//!
//! ## Supported Events
//!
//! - **Issues**: `on_issue()`, `on_issue_comment()`
//! - **Pull Requests**: `on_pull_request()`, `on_pull_request_review()`,
//!   `on_pull_request_review_comment()`, `on_pull_request_review_thread()`
//!
//! ## Quick Start
//!
//! ### 1. Basic Example
//!
//! ```rust,no_run
//! use octofer::{Octofer, Config};
//! use std::sync::Arc;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     // Load configuration from environment variables
//!     let config = Config::from_env().unwrap_or_default();
//!     config.init_logging();
//!     
//!     // Create the application
//!     let mut app = Octofer::new(config).await.unwrap_or_else(|_| {
//!         Octofer::new_default()
//!     });
//!
//!     // Register event handlers
//!     app.on_issue(|context, _| async move {
//!         println!("Issue event: {}", context.kind());
//!         Ok(())
//!     }, Arc::new(())).await;
//!     
//!     // Start the webhook server
//!     app.start().await?;
//!     Ok(())
//! }
//! ```
//!
//! ### 2. Full-Featured Example
//!
//! ```rust,no_run
//! use octofer::{Octofer, Config, Context};
//! use std::sync::Arc;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     // Load configuration and initialize logging
//!     let config = Config::from_env()?;
//!     config.init_logging();
//!     
//!     let mut app = Octofer::new(config).await?;
//!
//!     // Handle issue comments
//!     app.on_issue_comment(
//!         |context: Context, _| async move {
//!             let payload = context.payload();
//!             
//!             if let Some(comment) = payload.get("comment") {
//!                 if let Some(body) = comment.get("body") {
//!                     println!("Comment: {}", body);
//!                     
//!                     // Use GitHub API client
//!                     if let Some(client) = context.github() {
//!                         let installations = client.get_installations().await?;
//!                         println!("Found {} installations", installations.len());
//!                     }
//!                 }
//!             }
//!             
//!             Ok(())
//!         },
//!         Arc::new(()),
//!     ).await;
//!
//!     // Handle pull requests
//!     app.on_pull_request(
//!         |context: Context, _| async move {
//!             let payload = context.payload();
//!             
//!             if payload.get("action").and_then(|a| a.as_str()) == Some("opened") {
//!                 println!("New PR opened!");
//!                 
//!                 // Use installation client for repository operations
//!                 if let Some(installation_client) = context.installation_client().await? {
//!                     let user = installation_client.current().user().await?;
//!                     println!("Acting as: {}", user.login);
//!                 }
//!             }
//!             
//!             Ok(())
//!         },
//!         Arc::new(()),
//!     ).await;
//!
//!     println!("Starting GitHub App on {}:{}",
//!         app.config().server.host,
//!         app.config().server.port
//!     );
//!     
//!     app.start().await?;
//!     Ok(())
//! }
//! ```
//!
//! ## Configuration
//!
//! Set the following environment variables:
//!
//! ```bash
//! # Required
//! export GITHUB_APP_ID=your_app_id
//! export GITHUB_PRIVATE_KEY_PATH=path/to/private-key.pem
//! export GITHUB_WEBHOOK_SECRET=your_webhook_secret
//!
//! # Optional
//! export OCTOFER_HOST=0.0.0.0      # Default: 127.0.0.1
//! export OCTOFER_PORT=3000         # Default: 8000
//! export OCTOFER_LOG_LEVEL=debug   # Default: info
//! ```
//!
//! See [`Config`] for complete configuration options.
//!
//! ## Architecture
//!
//! Octofer consists of several key modules:
//!
//! - [`config`] - Configuration management and environment variable loading
//! - [`core`] - Core types including [`Context`] and event handler traits  
//! - [`github`] - GitHub API client with authentication and token management
//! - [`events`] - Event handler registration methods
//! - [`webhook`] - HTTP server for receiving webhook events
//!
//! ## Error Handling
//!
//! All event handlers should return `anyhow::Result<()>`. If a handler returns
//! an error, it will be logged and the webhook request will return a 500 status
//! code, causing GitHub to retry the delivery.
//!
//! ## Thread Safety
//!
//! Octofer is fully thread-safe. Event handlers can be called concurrently,
//! and the GitHub client handles token caching and refresh automatically
//! across threads.

pub mod config;
pub mod core;
pub mod events;
pub mod github;
pub mod webhook;

pub use config::Config;
pub use core::Context;

use octocrab::models::webhook_events::WebhookEventType;
use serde::Serialize;
use tracing::error;

use crate::webhook::WebhookServer;
use anyhow::Result;

const UNDEFINED_EVENT_KIND: &str = "undefined";

/// Trait for converting types to strings via serde serialization
///
/// This trait provides a way to convert types to string representation
/// using serde serialization. It's primarily used internally for converting
/// webhook event types to strings.
pub trait SerdeToString {
    /// Convert the value to a string using serde serialization
    fn to_string(&self) -> String
    where
        Self: Serialize;
}

impl SerdeToString for WebhookEventType {
    /// Convert webhook event type to string
    ///
    /// Converts the WebhookEventType enum to its string representation.
    /// If serialization fails, returns "undefined".
    fn to_string(&self) -> String {
        match serde_json::to_value(self) {
            Ok(v) => v.to_string(),
            Err(e) => {
                error!("Cannot parse event kind: {:?}", e);
                UNDEFINED_EVENT_KIND.to_string()
            }
        }
    }
}

/// Main Octofer application
///
/// The Octofer struct is the main entry point for building GitHub Apps.
/// It combines configuration, webhook server, and event handler registration
/// into a single, easy-to-use interface.
///
/// # Features
///
/// - **Configuration Management** - Handles loading and validation of app configuration
/// - **Webhook Server** - Built-in HTTP server for receiving GitHub webhook events
/// - **Event Handler Registration** - Methods for registering handlers for different event types
/// - **GitHub API Integration** - Automatic authentication and API client management
///
/// # Examples
///
/// ## Basic Usage
///
/// ```rust,no_run
/// use octofer::{Octofer, Config};
/// use std::sync::Arc;
///
/// # async fn example() -> anyhow::Result<()> {
/// let config = Config::from_env().unwrap_or_default();
/// let mut app = Octofer::new(config).await?;
///
/// app.on_issue(|context, _| async move {
///     println!("Issue event: {}", context.kind());
///     Ok(())
/// }, Arc::new(())).await;
///
/// app.start().await?;
/// # Ok(())
/// # }
/// ```
///
/// ## With Custom Configuration
///
/// ```rust,no_run
/// use octofer::{Octofer, Config};
/// use std::net::Ipv4Addr;
/// use std::sync::Arc;
///
/// # async fn example() -> anyhow::Result<()> {
/// let config = Config::new(
///     123456,                                    // app_id
///     Some("private-key.pem".to_string()),       // private_key_path
///     None,                                      // private_key_base64
///     "webhook-secret".to_string(),              // webhook_secret
///     Ipv4Addr::new(0, 0, 0, 0),                // host (all interfaces)
///     3000,                                      // port
/// )?;
///
/// let mut app = Octofer::new(config).await?;
///
/// // Register handlers...
/// app.start().await?;
/// # Ok(())
/// # }
/// ```
#[derive(Default)]
pub struct Octofer {
    /// The underlying webhook server
    server: WebhookServer,
    /// Application configuration
    config: Config,
}

impl Octofer {
    /// Create a new Octofer instance with the provided configuration
    ///
    /// Creates a new Octofer application with the specified configuration.
    /// This will set up the webhook server, GitHub client, and authentication
    /// based on the provided configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - Application configuration containing GitHub App credentials,
    ///   server settings, webhook configuration, and logging options
    ///
    /// # Returns
    ///
    /// Returns `Ok(Octofer)` if the application was created successfully,
    /// or `Err` if there was an error setting up the GitHub client or webhook server.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - GitHub App authentication fails (invalid credentials)
    /// - The webhook server cannot be created
    /// - Network issues prevent GitHub client setup
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use octofer::{Octofer, Config};
    ///
    /// # async fn example() -> anyhow::Result<()> {
    /// // Load configuration from environment variables
    /// let config = Config::from_env()?;
    ///
    /// // Create the application
    /// let mut app = Octofer::new(config).await?;
    ///
    /// // Register event handlers and start the server
    /// app.start().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn new(config: Config) -> Result<Self> {
        let server = WebhookServer::new(
            config.server.host,
            config.server.port,
            config.github.clone(),
            &config.webhook.secret,
            &config.webhook.header_name,
        )
        .await?;

        Ok(Octofer {
            config: config.clone(),
            server,
        })
    }

    /// Create a new Octofer instance with default configuration
    ///
    /// Creates a new Octofer application with default settings suitable for
    /// development and testing. This creates a server without GitHub App
    /// authentication, so handlers won't have access to the GitHub API.
    ///
    /// # Default Settings
    ///
    /// - Host: 127.0.0.1 (localhost only)
    /// - Port: 8000
    /// - No GitHub client (handlers won't have API access)
    /// - Default webhook secret (insecure, for development only)
    /// - Default logging configuration (info level, compact format)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use octofer::Octofer;
    /// use std::sync::Arc;
    ///
    /// # async fn example() -> anyhow::Result<()> {
    /// let mut app = Octofer::new_default();
    ///
    /// // Register a simple handler (no GitHub API access)
    /// app.on_issue(|context, _| async move {
    ///     println!("Issue event: {}", context.kind());
    ///     Ok(())
    /// }, Arc::new(())).await;
    ///
    /// // Start the server (will listen on localhost:8000)
    /// app.start().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new_default() -> Self {
        let config = Config::default();
        Octofer {
            server: WebhookServer::new_default(),
            config,
        }
    }

    /// Start the application server
    ///
    /// Starts the webhook server and begins listening for GitHub webhook events.
    /// This method will block until the server is stopped or an error occurs.
    ///
    /// The server will be available at:
    /// - `POST /webhook` - Endpoint for receiving GitHub webhook events
    /// - `GET /health` - Health check endpoint for monitoring
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the server stops gracefully, or `Err` if there's
    /// an error starting the server or during operation.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use octofer::{Octofer, Config};
    ///
    /// # async fn example() -> anyhow::Result<()> {
    /// let config = Config::from_env().unwrap_or_default();
    /// let mut app = Octofer::new(config).await.unwrap_or_else(|_| {
    ///     Octofer::new_default()
    /// });
    ///
    /// // Register handlers here...
    ///
    /// println!("Starting server on {}:{}",
    ///     app.config().server.host,
    ///     app.config().server.port
    /// );
    ///
    /// app.start().await?;  // This blocks until the server stops
    /// # Ok(())
    /// # }
    /// ```
    pub async fn start(&self) -> Result<()> {
        self.server.start().await
    }

    /// Get access to the configuration
    ///
    /// Returns a reference to the application configuration. This can be used
    /// to access configuration values from within the application.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use octofer::{Octofer, Config};
    ///
    /// # async fn example() -> anyhow::Result<()> {
    /// let config = Config::from_env()?;
    /// let app = Octofer::new(config).await?;
    ///
    /// // Access configuration values
    /// let config = app.config();
    /// println!("Server will listen on {}:{}", config.server.host, config.server.port);
    /// println!("Log level: {}", config.logging.level);
    /// println!("GitHub App ID: {}", config.github.app_id);
    /// # Ok(())
    /// # }
    /// ```
    pub fn config(&self) -> &Config {
        &self.config
    }
}
