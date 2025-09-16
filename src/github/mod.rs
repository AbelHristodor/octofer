//! GitHub API client and authentication module
//!
//! This module provides GitHub API integration with automatic authentication
//! and installation token management for GitHub Apps.

pub mod auth;
pub mod client;
pub mod middlewares;
pub mod models;

pub use auth::*;
pub use client::*;
pub use models::*;
