//! GitHub API client and authentication module
//!
//! This module provides GitHub API integration with automatic authentication
//! and installation token management for GitHub Apps.
//!
//! # Key Components
//!
//! - [`GitHubAuth`] - GitHub App authentication configuration
//! - [`GitHubClient`] - High-level GitHub API client with token management
//! - [`middlewares`] - Request/response middleware for security and event processing
//! - [`models`] - GitHub API data models (re-exported from octocrab)
//!
//! # Authentication Flow
//!
//! 1. **App Authentication**: Uses JWT tokens signed with your GitHub App's private key
//! 2. **Installation Tokens**: Generates installation-specific tokens for repository access
//! 3. **Token Caching**: Automatically caches and refreshes tokens as needed
//!
//! # Examples
//!
//! ## Basic Setup
//!
//! ```rust,no_run
//! use octofer::{Config, github::{GitHubAuth, GitHubClient}};
//!
//! # async fn example() -> anyhow::Result<()> {
//! // Load configuration from environment variables
//! let config = Config::from_env()?;
//!
//! // Create authentication
//! let auth = GitHubAuth::from_config(&config.github);
//!
//! // Create GitHub client
//! let client = GitHubClient::new(auth).await?;
//!
//! // Now you can use the client for GitHub API operations
//! let installations = client.get_installations().await?;
//! println!("Found {} installations", installations.len());
//! # Ok(())
//! # }
//! ```
//!
//! ## Working with Installations
//!
//! ```rust,no_run
//! # use octofer::{Config, github::{GitHubAuth, GitHubClient}};
//! # async fn example(client: GitHubClient) -> anyhow::Result<()> {
//! // Get all installations
//! let installations = client.get_installations().await?;
//!
//! for installation in installations {
//!     println!("Installation: {}", installation.account.login);
//!     
//!     // Get repositories for this installation
//!     let repos = client.get_installation_repositories(installation.id.0).await?;
//!     println!("  {} repositories", repos.len());
//!     
//!     // Get an installation-specific client
//!     let installation_client = client.installation_client(installation.id.0).await?;
//!     
//!     // Use the installation client for repository operations
//!     // (This client has permissions scoped to this specific installation)
//! }
//! # Ok(())
//! # }
//! ```

pub mod auth;
pub mod client;
pub mod middlewares;
pub mod models;

pub use auth::*;
pub use client::*;
pub use models::*;
