//! GitHub authentication and configuration
//!
//! This module provides authentication utilities for GitHub Apps, including
//! JWT token generation and GitHub API client configuration.
//!
//! # GitHub App Authentication
//!
//! GitHub Apps use JWT tokens for authentication. This module handles the
//! creation and management of these authentication credentials.
//!
//! # Examples
//!
//! ```rust,no_run
//! use octofer::{Config, github::GitHubAuth};
//! 
//! // Create authentication from configuration
//! let config = Config::from_env()?;
//! let auth = GitHubAuth::from_config(&config.github);
//! 
//! println!("App ID: {}", auth.app_id());
//! # Ok::<(), anyhow::Error>(())
//! ```

use crate::config::GitHubConfig;

/// GitHub authentication configuration
///
/// Contains the GitHub App ID and private key needed for JWT authentication.
/// This struct is used to authenticate with the GitHub API as a GitHub App.
///
/// # Examples
///
/// ```rust,no_run
/// use octofer::{Config, github::GitHubAuth};
/// 
/// // Create from configuration
/// let config = Config::from_env()?;
/// let auth = GitHubAuth::from_config(&config.github);
/// 
/// // Access authentication details
/// println!("App ID: {}", auth.app_id());
/// println!("Private key length: {} bytes", auth.private_key().len());
/// # Ok::<(), anyhow::Error>(())
/// ```
#[derive(Debug, Clone)]
pub struct GitHubAuth {
    /// GitHub App ID (found in your GitHub App settings)
    pub app_id: u64,
    /// Private key for JWT signing (PEM format as bytes)
    pub private_key: Vec<u8>,
}

impl GitHubAuth {
    /// Create authentication from GitHubConfig
    ///
    /// Creates a new GitHubAuth instance from the provided configuration.
    /// This is typically used when initializing the GitHub client.
    ///
    /// # Arguments
    ///
    /// * `config` - GitHub configuration containing app ID and private key
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use octofer::{Config, github::GitHubAuth};
    /// 
    /// let config = Config::from_env()?;
    /// let auth = GitHubAuth::from_config(&config.github);
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn from_config(config: &GitHubConfig) -> Self {
        Self {
            app_id: config.app_id,
            private_key: config.private_key.clone(),
        }
    }

    /// Get the App ID
    ///
    /// Returns the GitHub App ID used for authentication.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use octofer::{Config, github::GitHubAuth};
    /// 
    /// let config = Config::from_env()?;
    /// let auth = GitHubAuth::from_config(&config.github);
    /// println!("App ID: {}", auth.app_id());
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn app_id(&self) -> u64 {
        self.app_id
    }

    /// Get the private key
    ///
    /// Returns a reference to the private key bytes used for JWT signing.
    /// The private key is in PEM format.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use octofer::{Config, github::GitHubAuth};
    /// 
    /// let config = Config::from_env()?;
    /// let auth = GitHubAuth::from_config(&config.github);
    /// let key_bytes = auth.private_key();
    /// println!("Private key length: {} bytes", key_bytes.len());
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn private_key(&self) -> &[u8] {
        &self.private_key
    }
}

/// Parse a UTC datetime string
///
/// Parses a datetime string into a UTC DateTime object. This is used internally
/// for handling GitHub API responses that contain datetime strings.
///
/// # Arguments
///
/// * `date_str` - A datetime string in ISO 8601 format
///
/// # Returns
///
/// Returns a `chrono::DateTime<chrono::Utc>` representing the parsed datetime.
///
/// # Panics
///
/// This function will panic if the input string is not a valid datetime format.
/// This is intentional for internal use where the format is expected to be valid.
///
/// # Examples
///
/// ```rust,no_run
/// use octofer::github::auth::parse_to_utc;
/// 
/// let datetime = parse_to_utc("2025-07-10T09:14:47Z");
/// println!("Parsed datetime: {}", datetime);
/// 
/// // With timezone offset
/// let datetime = parse_to_utc("2025-07-10T09:14:47+02:00");
/// println!("UTC datetime: {}", datetime);
/// ```
pub fn parse_to_utc(date_str: &str) -> chrono::DateTime<chrono::Utc> {
    date_str
        .parse::<chrono::DateTime<chrono::Utc>>()
        .expect("Invalid date format")
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Datelike, Timelike};

    #[test]
    fn test_parse_to_utc_valid() {
        let datetime_str = "2025-07-10T09:14:47Z";
        let result = parse_to_utc(datetime_str);
        assert_eq!(result.year(), 2025);
        assert_eq!(result.month(), 7);
        assert_eq!(result.day(), 10);
        assert_eq!(result.hour(), 9);
        assert_eq!(result.minute(), 14);
        assert_eq!(result.second(), 47);
    }

    #[test]
    fn test_parse_to_utc_with_microseconds() {
        let datetime_str = "2025-07-10T09:14:47.123456Z";
        let _ = parse_to_utc(datetime_str);
    }

    #[test]
    #[should_panic]
    fn test_parse_to_utc_invalid() {
        let datetime_str = "invalid-datetime";
        let _ = parse_to_utc(datetime_str);
    }

    #[test]
    fn test_parse_to_utc_with_timezone_offset() {
        let datetime_str = "2025-07-10T09:14:47+02:00";
        let result = parse_to_utc(datetime_str);
        assert_eq!(result.hour(), 7); // 9 - 2 = 7 UTC
    }
}
