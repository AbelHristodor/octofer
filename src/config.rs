//! Configuration module for Octofer
//!
//! This module provides centralized configuration management for all Octofer components.
//! Configuration can be loaded from environment variables or provided directly.

use anyhow::{anyhow, Result};
use base64::Engine;
use serde::{Deserialize, Serialize};
use std::env;
use std::net::Ipv4Addr;

/// Main configuration struct containing all necessary configuration for Octofer components
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    /// GitHub-specific configuration
    pub github: GitHubConfig,
    /// Server configuration for the webhook server
    pub server: ServerConfig,
    /// Webhook-specific configuration
    pub webhook: WebhookConfig,
}

impl Config {
    /// Create a new configuration from environment variables
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            github: GitHubConfig::from_env()?,
            server: ServerConfig::from_env(),
            webhook: WebhookConfig::from_env(),
        })
    }

    /// Create a new configuration with custom values
    pub fn new(
        app_id: u64,
        private_key_path: Option<String>,
        private_key_base64: Option<String>,
        webhook_secret: String,
        host: Ipv4Addr,
        port: u16,
    ) -> Result<Self> {
        Ok(Self {
            github: GitHubConfig::new(app_id, private_key_path, private_key_base64)?,
            server: ServerConfig { host, port },
            webhook: WebhookConfig {
                secret: webhook_secret,
                header_name: "x-hub-signature-256".to_string(),
            },
        })
    }
}

/// GitHub App configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GitHubConfig {
    /// GitHub App ID
    pub app_id: u64,
    /// Private key as bytes
    pub private_key: Vec<u8>,
}

impl GitHubConfig {
    /// Create GitHub configuration from environment variables
    pub fn from_env() -> Result<Self> {
        let app_id = env::var("GITHUB_APP_ID")
            .map_err(|_| anyhow!("GITHUB_APP_ID environment variable is required"))?
            .parse::<u64>()
            .map_err(|_| anyhow!("GITHUB_APP_ID must be a valid number"))?;

        let private_key = if let Ok(path) = env::var("GITHUB_PRIVATE_KEY_PATH") {
            std::fs::read(&path)
                .map_err(|e| anyhow!("Failed to read private key from {}: {}", path, e))?
        } else if let Ok(base64_key) = env::var("GITHUB_PRIVATE_KEY_BASE64") {
            base64::engine::general_purpose::STANDARD
                .decode(&base64_key)
                .map_err(|e| anyhow!("Failed to decode private key from base64: {}", e))?
        } else {
            return Err(anyhow!(
                "Either GITHUB_PRIVATE_KEY_PATH or GITHUB_PRIVATE_KEY_BASE64 must be set"
            ));
        };

        Ok(Self {
            app_id,
            private_key,
        })
    }

    /// Create GitHub configuration with explicit values
    pub fn new(
        app_id: u64,
        private_key_path: Option<String>,
        private_key_base64: Option<String>,
    ) -> Result<Self> {
        let private_key = if let Some(path) = private_key_path {
            std::fs::read(&path)
                .map_err(|e| anyhow!("Failed to read private key from {}: {}", path, e))?
        } else if let Some(base64_key) = private_key_base64 {
            base64::engine::general_purpose::STANDARD
                .decode(&base64_key)
                .map_err(|e| anyhow!("Failed to decode private key from base64: {}", e))?
        } else {
            return Err(anyhow!(
                "Either private_key_path or private_key_base64 must be provided"
            ));
        };

        Ok(Self {
            app_id,
            private_key,
        })
    }
}

/// Server configuration for the webhook server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Host address to bind to
    pub host: Ipv4Addr,
    /// Port to listen on
    pub port: u16,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: Ipv4Addr::LOCALHOST,
            port: 8000,
        }
    }
}

impl ServerConfig {
    /// Create server configuration from environment variables
    pub fn from_env() -> Self {
        let host = env::var("OCTOFER_HOST")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(Ipv4Addr::LOCALHOST);

        let port = env::var("OCTOFER_PORT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(8000);

        Self { host, port }
    }
}

/// Webhook configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookConfig {
    /// Webhook secret for HMAC verification
    pub secret: String,
    /// Header name for HMAC signature
    pub header_name: String,
}

impl Default for WebhookConfig {
    fn default() -> Self {
        Self {
            secret: "development-secret".to_string(),
            header_name: "x-hub-signature-256".to_string(),
        }
    }
}

impl WebhookConfig {
    /// Create webhook configuration from environment variables
    pub fn from_env() -> Self {
        let secret =
            env::var("GITHUB_WEBHOOK_SECRET").unwrap_or_else(|_| "development-secret".to_string());

        let header_name =
            env::var("GITHUB_WEBHOOK_HEADER").unwrap_or_else(|_| "x-hub-signature-256".to_string());

        Self {
            secret,
            header_name,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.server.host, Ipv4Addr::LOCALHOST);
        assert_eq!(config.server.port, 8000);
        assert_eq!(config.webhook.secret, "development-secret");
        assert_eq!(config.webhook.header_name, "x-hub-signature-256");
    }

    #[test]
    fn test_server_config_from_env() {
        env::set_var("OCTOFER_HOST", "0.0.0.0");
        env::set_var("OCTOFER_PORT", "3000");

        let config = ServerConfig::from_env();
        assert_eq!(config.host, Ipv4Addr::new(0, 0, 0, 0));
        assert_eq!(config.port, 3000);

        env::remove_var("OCTOFER_HOST");
        env::remove_var("OCTOFER_PORT");
    }
}
