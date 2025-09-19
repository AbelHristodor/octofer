//! Event handler registration for GitHub webhook events
//!
//! This module provides methods for registering event handlers for different types
//! of GitHub webhook events. Each method allows you to register a handler function
//! that will be called when the corresponding event is received.
//!
//! # Available Event Handlers
//!
//! ## Issue Events
//! - [`on_issue()`](../struct.Octofer.html#method.on_issue) - Issue opened, closed, edited, etc.
//! - [`on_issue_comment()`](../struct.Octofer.html#method.on_issue_comment) - Comments on issues
//!
//! ## Pull Request Events  
//! - [`on_pull_request()`](../struct.Octofer.html#method.on_pull_request) - PR opened, closed, merged, etc.
//! - [`on_pull_request_review()`](../struct.Octofer.html#method.on_pull_request_review) - PR reviews submitted
//! - [`on_pull_request_review_comment()`](../struct.Octofer.html#method.on_pull_request_review_comment) - Comments on PR reviews
//! - [`on_pull_request_review_thread()`](../struct.Octofer.html#method.on_pull_request_review_thread) - PR review threads
//!
//! # Handler Function Signature
//!
//! All event handlers must have the following signature:
//!
//! ```rust,ignore
//! async fn handler(context: Context, extra: Arc<ExtraData>) -> anyhow::Result<()>
//! ```
//!
//! Where:
//! - `context` - Contains the webhook event data and GitHub API client
//! - `extra` - Additional data you want to pass to the handler
//!
//! # Examples
//!
//! ## Basic Issue Handler
//!
//! ```rust,no_run
//! use octofer::{Octofer, Config, Context};
//! use std::sync::Arc;
//!
//! # async fn example() -> anyhow::Result<()> {
//! let config = Config::default();
//! let mut app = Octofer::new(config).await.unwrap_or_else(|_| Octofer::new_default());
//!
//! // Register an issue event handler
//! app.on_issue(
//!     |context: Context, _extra: Arc<()>| async move {
//!         println!("Issue event: {}", context.kind());
//!         
//!         // Access the event payload
//!         let payload = context.payload();
//!         if let Some(action) = payload.get("action") {
//!             println!("Action: {}", action);
//!         }
//!         
//!         Ok(())
//!     },
//!     Arc::new(()),
//! ).await;
//! # Ok(())
//! # }
//! ```
//!
//! ## Handler with GitHub API Usage
//!
//! ```rust,no_run
//! use octofer::{Octofer, Config, Context};
//! use std::sync::Arc;
//!
//! # async fn example() -> anyhow::Result<()> {
//! let config = Config::from_env().unwrap_or_default();
//! let mut app = Octofer::new(config).await.unwrap_or_else(|_| Octofer::new_default());
//!
//! app.on_issue_comment(
//!     |context: Context, _extra: Arc<()>| async move {
//!         println!("Issue comment event received");
//!         
//!         // Use the GitHub client if available
//!         if let Some(client) = context.github() {
//!             let installations = client.get_installations().await?;
//!             println!("GitHub client available, found {} installations", installations.len());
//!         }
//!         
//!         // Use installation client for repository operations
//!         if let Some(installation_client) = context.installation_client().await? {
//!             let user = installation_client.current().user().await?;
//!             println!("Acting as: {}", user.login);
//!         }
//!         
//!         Ok(())
//!     },
//!     Arc::new(()),
//! ).await;
//! # Ok(())
//! # }
//! ```
//!
//! ## Handler with Custom Extra Data
//!
//! ```rust,no_run
//! use octofer::{Octofer, Config, Context};
//! use std::sync::Arc;
//!
//! #[derive(Clone, Debug)]
//! struct AppData {
//!     name: String,
//!     version: String,
//! }
//!
//! # async fn example() -> anyhow::Result<()> {
//! let config = Config::default();
//! let mut app = Octofer::new(config).await.unwrap_or_else(|_| Octofer::new_default());
//!
//! let app_data = Arc::new(AppData {
//!     name: "MyBot".to_string(),
//!     version: "1.0.0".to_string(),
//! });
//!
//! app.on_pull_request(
//!     |context: Context, extra: Arc<AppData>| async move {
//!         println!("PR event handled by {} v{}", extra.name, extra.version);
//!         
//!         let payload = context.payload();
//!         if let Some(pr) = payload.get("pull_request") {
//!             if let Some(title) = pr.get("title") {
//!                 println!("PR Title: {}", title);
//!             }
//!         }
//!         
//!         Ok(())
//!     },
//!     app_data,
//! ).await;
//! # Ok(())
//! # }
//! ```

mod issues;
mod prs;
