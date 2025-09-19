//! Webhook server module for handling GitHub webhook events
//!
//! This module provides the HTTP server implementation for receiving and processing
//! GitHub webhook events. It includes HMAC verification, event routing, and handler
//! execution.
//!
//! # Key Components
//!
//! - [`WebhookServer`] - HTTP server for receiving webhook events
//! - [`AppState`] - Shared application state containing handlers and GitHub client
//! - [`handlers`] - Request handlers for webhook and health check endpoints
//!
//! # Architecture
//!
//! The webhook server uses Axum for HTTP handling with the following flow:
//!
//! 1. **HMAC Verification** - Validates webhook authenticity using shared secret
//! 2. **Event Processing** - Extracts GitHub event information from headers
//! 3. **Handler Routing** - Routes events to registered handlers based on event type
//! 4. **Context Creation** - Creates Context with event data and GitHub client
//! 5. **Handler Execution** - Executes all registered handlers for the event type
//!
//! # Examples
//!
//! ## Basic Server Setup
//!
//! ```rust,no_run
//! use octofer::{Config, webhook::WebhookServer};
//!
//! # async fn example() -> anyhow::Result<()> {
//! let config = Config::from_env()?;
//! let server = WebhookServer::new(
//!     config.server.host,
//!     config.server.port,
//!     config.github.clone(),
//!     &config.webhook.secret,
//!     &config.webhook.header_name,
//! ).await?;
//!
//! // Server is ready to receive webhooks
//! server.start().await?;
//! # Ok(())
//! # }
//! ```

pub mod handlers;
pub mod server;

pub use server::*;
