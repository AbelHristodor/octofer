//! GitHub API models and types
//!
//! This module re-exports commonly used GitHub API models from the octocrab crate.
//! These models represent the data structures returned by the GitHub API and used
//! in webhook events.
//!
//! # Commonly Used Models
//!
//! - `Installation` - Represents a GitHub App installation
//! - `Repository` - Represents a GitHub repository
//! - `User` - Represents a GitHub user or organization
//! - `Issue` - Represents a GitHub issue
//! - `PullRequest` - Represents a GitHub pull request
//! - `WebhookEvent` - Represents incoming webhook events
//!
//! # Examples
//!
//! ```rust,no_run
//! use octofer::github::models::{Repository, Installation};
//!
//! // These types are available for use in your application
//! fn process_repository(repo: Repository) {
//!     if let Some(name) = repo.full_name {
//!         println!("Repository: {}", name);
//!     }
//!     if let Some(private) = repo.private {
//!         println!("Private: {}", private);
//!     }
//! }
//!
//! fn process_installation(installation: Installation) {
//!     println!("Installation ID: {}", installation.id.0);
//!     println!("Account: {}", installation.account.login);
//! }
//! ```
//!
//! For detailed documentation of individual models, refer to the
//! [octocrab documentation](https://docs.rs/octocrab/).

// Re-export commonly used octocrab models
pub use octocrab::models::*;
