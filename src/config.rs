//! Configuration module for Octofer
//!
//! This module provides centralized configuration management for all Octofer components.
//! Configuration can be loaded from environment variables or provided directly.

use anyhow::{anyhow, Result};
use base64::Engine;
use serde::{Deserialize, Serialize};
use std::env;
use std::net::Ipv4Addr;
use tracing::Level;

/// Constants
pub const DEFAULT_HOST_ADDR: Ipv4Addr = Ipv4Addr::LOCALHOST;
pub const DEFAULT_PORT: u16 = 8000;

pub const WEBHOOK_SECRET: &str = "octofer-webhook-secret";
pub const WEBHOOK_HEADER_NAME: &str = "X-Hub-Signature-256";

const GH_APP_ID: &str = "GITHUB_APP_ID";
const GH_PRIVATE_KEY_PATH: &str = "GITHUB_PRIVATE_KEY_PATH";
const GH_PRIVATE_KEY_BASE64: &str = "GITHUB_PRIVATE_KEY_BASE64";
const GH_WEBHOOK_SECRET: &str = "GITHUB_WEBHOOK_SECRET";
const GH_WEBHOOK_HEADER_NAME: &str = "GITHUB_WEBHOOK_HEADER_NAME";

const OCTOFER_HOST: &str = "OCTOFER_HOST";
const OCTOFER_PORT: &str = "OCTOFER_PORT";

const OCTOFER_LOG_LEVEL: &str = "OCTOFER_LOG_LEVEL";
const OCTOFER_LOG_FORMAT: &str = "OCTOFER_LOG_FORMAT";
const OCTOFER_LOG_WITH_TARGET: &str = "OCTOFER_LOG_WITH_TARGET";
const OCTOFER_LOG_WITH_FILE: &str = "OCTOFER_LOG_WITH_FILE";
const OCTOFER_LOG_WITH_THREAD_IDS: &str = "OCTOFER_LOG_WITH_THREAD_IDS";
const LOG_FORMAT: &str = "compact";

/// Main configuration struct containing all necessary configuration for Octofer components
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    /// GitHub-specific configuration
    pub github: GitHubConfig,
    /// Server configuration for the webhook server
    pub server: ServerConfig,
    /// Webhook-specific configuration
    pub webhook: WebhookConfig,
    /// Logging configuration
    pub logging: LoggingConfig,
}

impl Config {
    /// Create a new configuration from environment variables
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            github: GitHubConfig::from_env()?,
            server: ServerConfig::from_env(),
            webhook: WebhookConfig::from_env(),
            logging: LoggingConfig::from_env(),
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
                header_name: WEBHOOK_HEADER_NAME.to_string(),
            },
            logging: LoggingConfig::default(),
        })
    }

    /// Initialize tracing based on the logging configuration
    pub fn init_logging(&self) {
        self.logging.init_tracing();
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
        let app_id = env::var(GH_APP_ID)
            .map_err(|_| anyhow!("{GH_APP_ID} environment variable is required"))?
            .parse::<u64>()
            .map_err(|_| anyhow!("{GH_APP_ID} must be a valid number"))?;

        let private_key = if let Ok(path) = env::var(GH_PRIVATE_KEY_PATH) {
            std::fs::read(&path)
                .map_err(|e| anyhow!("Failed to read private key from {}: {}", path, e))?
        } else if let Ok(base64_key) = env::var(GH_PRIVATE_KEY_BASE64) {
            base64::engine::general_purpose::STANDARD
                .decode(&base64_key)
                .map_err(|e| anyhow!("Failed to decode private key from base64: {}", e))?
        } else {
            return Err(anyhow!(
                "Either {GH_PRIVATE_KEY_PATH} or {GH_PRIVATE_KEY_BASE64} must be set"
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
            host: DEFAULT_HOST_ADDR,
            port: DEFAULT_PORT,
        }
    }
}

impl ServerConfig {
    /// Create server configuration from environment variables
    pub fn from_env() -> Self {
        let host = env::var(OCTOFER_HOST)
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(DEFAULT_HOST_ADDR);

        let port = env::var(OCTOFER_PORT)
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(DEFAULT_PORT);

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
            secret: WEBHOOK_SECRET.to_string(),
            header_name: WEBHOOK_HEADER_NAME.to_string(),
        }
    }
}

impl WebhookConfig {
    /// Create webhook configuration from environment variables
    pub fn from_env() -> Self {
        let secret = env::var(GH_WEBHOOK_SECRET).unwrap_or_else(|_| WEBHOOK_SECRET.to_string());

        let header_name =
            env::var(GH_WEBHOOK_HEADER_NAME).unwrap_or_else(|_| WEBHOOK_HEADER_NAME.to_string());

        Self {
            secret,
            header_name,
        }
    }
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Log level (trace, debug, info, warn, error)
    pub level: String,
    /// Log format (compact, pretty, json)
    pub format: String,
    /// Whether to include target information
    pub with_target: bool,
    /// Whether to include file and line information
    pub with_file: bool,
    /// Whether to include thread information
    pub with_thread_ids: bool,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: Level::INFO.to_string(),
            format: LOG_FORMAT.to_string(),
            with_target: false,
            with_file: false,
            with_thread_ids: false,
        }
    }
}

impl LoggingConfig {
    /// Create logging configuration from environment variables
    pub fn from_env() -> Self {
        let level = env::var(OCTOFER_LOG_LEVEL).unwrap_or_else(|_| Level::INFO.to_string());

        let format = env::var(OCTOFER_LOG_FORMAT).unwrap_or_else(|_| LOG_FORMAT.to_string());

        let with_target = env::var(OCTOFER_LOG_WITH_TARGET)
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(false);

        let with_file = env::var(OCTOFER_LOG_WITH_FILE)
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(false);

        let with_thread_ids = env::var(OCTOFER_LOG_WITH_THREAD_IDS)
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(false);

        Self {
            level,
            format,
            with_target,
            with_file,
            with_thread_ids,
        }
    }

    /// Initialize tracing subscriber based on this configuration
    pub fn init_tracing(&self) {
        use tracing_subscriber::{fmt, EnvFilter};

        let env_filter = EnvFilter::try_from_default_env()
            .or_else(|_| EnvFilter::try_new(&self.level))
            .unwrap_or_else(|_| EnvFilter::new(Level::INFO.to_string()));

        let subscriber = fmt()
            .with_env_filter(env_filter)
            .with_target(self.with_target)
            .with_file(self.with_file)
            .with_thread_ids(self.with_thread_ids);

        match self.format.as_str() {
            "pretty" => subscriber.pretty().init(),
            "json" => subscriber.json().init(),
            _ => subscriber.compact().init(), // Default to compact for unknown formats
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.server.host, DEFAULT_HOST_ADDR);
        assert_eq!(config.server.port, DEFAULT_PORT);
        assert_eq!(config.webhook.secret, WEBHOOK_SECRET);
        assert_eq!(config.webhook.header_name, WEBHOOK_HEADER_NAME);
        assert_eq!(config.logging.level, Level::INFO.to_string());
        assert_eq!(config.logging.format, LOG_FORMAT);
        assert!(!config.logging.with_target);
        assert!(!config.logging.with_file);
        assert!(!config.logging.with_thread_ids);
    }

    #[test]
    fn test_server_config_from_env() {
        env::set_var(OCTOFER_HOST, "0.0.0.0");
        env::set_var(OCTOFER_PORT, "3000");

        let config = ServerConfig::from_env();
        assert_eq!(config.host, Ipv4Addr::new(0, 0, 0, 0));
        assert_eq!(config.port, 3000);

        env::remove_var(OCTOFER_HOST);
        env::remove_var(OCTOFER_PORT);
    }

    #[test]
    fn test_logging_config_from_env() {
        env::set_var(OCTOFER_LOG_LEVEL, "debug");
        env::set_var(OCTOFER_LOG_FORMAT, "pretty");
        env::set_var(OCTOFER_LOG_WITH_TARGET, "true");
        env::set_var(OCTOFER_LOG_WITH_FILE, "true");
        env::set_var(OCTOFER_LOG_WITH_THREAD_IDS, "false");

        let config = LoggingConfig::from_env();
        assert_eq!(config.level, "debug");
        assert_eq!(config.format, "pretty");
        assert!(config.with_target);
        assert!(config.with_file);
        assert!(!config.with_thread_ids);

        env::remove_var(OCTOFER_LOG_LEVEL);
        env::remove_var(OCTOFER_LOG_FORMAT);
        env::remove_var(OCTOFER_LOG_WITH_TARGET);
        env::remove_var(OCTOFER_LOG_WITH_FILE);
        env::remove_var(OCTOFER_LOG_WITH_THREAD_IDS);
    }

    #[test]
    fn test_logging_config_defaults() {
        // Remove any potentially set environment variables
        env::remove_var(OCTOFER_LOG_LEVEL);
        env::remove_var(OCTOFER_LOG_FORMAT);
        env::remove_var(OCTOFER_LOG_WITH_TARGET);
        env::remove_var(OCTOFER_LOG_WITH_FILE);
        env::remove_var(OCTOFER_LOG_WITH_THREAD_IDS);

        let config = LoggingConfig::from_env();
        assert_eq!(config.level, Level::INFO.to_string());
        assert_eq!(config.format, LOG_FORMAT);
        assert!(!config.with_target);
        assert!(!config.with_file);
        assert!(!config.with_thread_ids);
    }
}
