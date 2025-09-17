//! Testing framework for Octofer GitHub Apps
//!
//! This module provides utilities and helpers for testing GitHub Apps built with Octofer.
//! It includes mock implementations, test builders, and assertion helpers to make
//! testing event handlers and GitHub App logic straightforward and reliable.
//!
//! ## Quick Start
//!
//! ```rust
//! use octofer::testing::{TestApp, MockWebhookEvent};
//! use anyhow::Result;
//!
//! #[tokio::test]
//! async fn test_my_issue_handler() -> Result<()> {
//!     let mut app = TestApp::new();
//!     
//!     let mut called = false;
//!     app.on_issues(|_context| async move {
//!         called = true;
//!         Ok(())
//!     }).await;
//!     
//!     let event = MockWebhookEvent::issue_opened("test-repo", 42)
//!         .title("Test Issue")
//!         .build();
//!         
//!     app.handle_event(event).await?;
//!     assert!(called);
//!     Ok(())
//! }
//! ```

pub mod mock_client;
pub mod mock_events;
pub mod test_app;
pub mod test_context;
pub mod assertions;

pub use mock_client::MockGitHubClient;
pub use mock_events::MockWebhookEvent;
pub use test_app::TestApp;
pub use test_context::TestContext;
pub use assertions::*;