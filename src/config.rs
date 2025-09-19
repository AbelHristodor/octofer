//! Configuration module for Octofer
//!
//! This module provides centralized configuration management for all Octofer components.
//! Configuration can be loaded from environment variables or provided directly.
//!
//! # Environment Variables
//!
//! ## Required GitHub App Configuration
//!
//! These environment variables are required for proper GitHub App authentication:
//!
//! * `GITHUB_APP_ID` - Your GitHub App ID (numeric value)
//!   - Example: `GITHUB_APP_ID=123456`
//!   - Where to find: GitHub App settings page
//!
//! * `GITHUB_PRIVATE_KEY_PATH` **OR** `GITHUB_PRIVATE_KEY_BASE64` - GitHub App private key
//!   - `GITHUB_PRIVATE_KEY_PATH=/path/to/private-key.pem` - Path to PEM file
//!   - `GITHUB_PRIVATE_KEY_BASE64=LS0tLS1C...` - Base64 encoded private key
//!   - Where to find: Download from GitHub App settings page
//!
//! ## Webhook Configuration
//!
//! * `GITHUB_WEBHOOK_SECRET` - Webhook secret for HMAC verification
//!   - Example: `GITHUB_WEBHOOK_SECRET=your-webhook-secret-here`
//!   - Default: `"octofer-webhook-secret"` (for development only)
//!   - Should be a cryptographically secure random string
//!
//! * `GITHUB_WEBHOOK_HEADER_NAME` - HMAC signature header name
//!   - Example: `GITHUB_WEBHOOK_HEADER_NAME=X-Hub-Signature-256`
//!   - Default: `"X-Hub-Signature-256"`
//!   - Usually doesn't need to be changed
//!
//! ## Server Configuration (Optional)
//!
//! * `OCTOFER_HOST` - Host address to bind webhook server to
//!   - Example: `OCTOFER_HOST=0.0.0.0` (bind to all interfaces)
//!   - Default: `127.0.0.1` (localhost only)
//!   - Values: Any valid IPv4 address
//!
//! * `OCTOFER_PORT` - Port for webhook server to listen on
//!   - Example: `OCTOFER_PORT=3000`
//!   - Default: `8000`
//!   - Values: Any valid port number (1-65535)
//!
//! ## Logging Configuration (Optional)
//!
//! * `OCTOFER_LOG_LEVEL` - Logging verbosity level
//!   - Example: `OCTOFER_LOG_LEVEL=debug`
//!   - Default: `"info"`
//!   - Values: `trace`, `debug`, `info`, `warn`, `error`
//!
//! * `OCTOFER_LOG_FORMAT` - Log output format
//!   - Example: `OCTOFER_LOG_FORMAT=json`
//!   - Default: `"compact"`
//!   - Values: `compact`, `pretty`, `json`
//!
//! * `OCTOFER_LOG_WITH_TARGET` - Include module target in logs
//!   - Example: `OCTOFER_LOG_WITH_TARGET=true`
//!   - Default: `false`
//!   - Values: `true`, `false`
//!
//! * `OCTOFER_LOG_WITH_FILE` - Include file/line info in logs
//!   - Example: `OCTOFER_LOG_WITH_FILE=true`
//!   - Default: `false`
//!   - Values: `true`, `false`
//!
//! * `OCTOFER_LOG_WITH_THREAD_IDS` - Include thread IDs in logs
//!   - Example: `OCTOFER_LOG_WITH_THREAD_IDS=true`
//!   - Default: `false`
//!   - Values: `true`, `false`
//!
//! # Configuration Examples
//!
//! ## Basic Configuration
//! ```bash
//! export GITHUB_APP_ID=123456
//! export GITHUB_PRIVATE_KEY_PATH=/path/to/private-key.pem
//! export GITHUB_WEBHOOK_SECRET=my-secure-webhook-secret
//! ```
//!
//! ## Production Configuration
//! ```bash
//! export GITHUB_APP_ID=123456
//! export GITHUB_PRIVATE_KEY_BASE64=LS0tLS1CRUdJTi...
//! export GITHUB_WEBHOOK_SECRET=production-webhook-secret
//! export OCTOFER_HOST=0.0.0.0
//! export OCTOFER_PORT=8080
//! export OCTOFER_LOG_LEVEL=info
//! export OCTOFER_LOG_FORMAT=json
//! ```
//!
//! ## Development Configuration with Debug Logging
//! ```bash
//! export GITHUB_APP_ID=123456
//! export GITHUB_PRIVATE_KEY_PATH=./dev-private-key.pem
//! export GITHUB_WEBHOOK_SECRET=dev-webhook-secret
//! export OCTOFER_LOG_LEVEL=debug
//! export OCTOFER_LOG_FORMAT=pretty
//! export OCTOFER_LOG_WITH_TARGET=true
//! export OCTOFER_LOG_WITH_FILE=true
//! ```

use anyhow::{anyhow, Result};
use base64::Engine;
use serde::{Deserialize, Serialize};
use std::env;
use std::net::Ipv4Addr;
use tracing::Level;

/// Default host address for the webhook server (127.0.0.1)
pub const DEFAULT_HOST_ADDR: Ipv4Addr = Ipv4Addr::LOCALHOST;

/// Default port for the webhook server
pub const DEFAULT_PORT: u16 = 8000;

/// Default webhook secret used when no environment variable is set
///
/// **Note**: This should only be used for development. In production,
/// always set `GITHUB_WEBHOOK_SECRET` to a secure random value.
pub const WEBHOOK_SECRET: &str = "octofer-webhook-secret";

/// Default header name for GitHub webhook HMAC signatures
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
///
/// This struct aggregates all configuration needed to run an Octofer GitHub App,
/// including GitHub App credentials, server settings, webhook configuration, and logging options.
///
/// # Examples
///
/// ## Create from environment variables
/// ```rust,no_run
/// use octofer::Config;
///
/// // Load configuration from environment variables
/// let config = Config::from_env().expect("Missing required environment variables");
/// config.init_logging();
/// ```
///
/// ## Create with explicit values
/// ```rust,no_run
/// use octofer::Config;
/// use std::net::Ipv4Addr;
///
/// let config = Config::new(
///     123456,                                    // app_id
///     Some("path/to/private-key.pem".to_string()), // private_key_path
///     None,                                      // private_key_base64
///     "your-webhook-secret".to_string(),         // webhook_secret
///     Ipv4Addr::LOCALHOST,                       // host
///     8000,                                      // port
/// ).expect("Failed to create configuration");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    /// GitHub-specific configuration including App ID and private key
    pub github: GitHubConfig,
    /// Server configuration for the webhook server (host and port)
    pub server: ServerConfig,
    /// Webhook-specific configuration including secret and header name
    pub webhook: WebhookConfig,
    /// Logging configuration for tracing setup
    pub logging: LoggingConfig,
}

impl Config {
    /// Create a new configuration from environment variables
    ///
    /// Loads all configuration from environment variables. All GitHub-related
    /// environment variables are required, while server and logging variables
    /// have sensible defaults.
    ///
    /// # Returns
    ///
    /// Returns `Ok(Config)` if all required environment variables are present and valid,
    /// or `Err` if any required variables are missing or invalid.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - `GITHUB_APP_ID` is not set or not a valid number
    /// - Neither `GITHUB_PRIVATE_KEY_PATH` nor `GITHUB_PRIVATE_KEY_BASE64` is set
    /// - Private key file cannot be read (if using `GITHUB_PRIVATE_KEY_PATH`)
    /// - Private key cannot be decoded (if using `GITHUB_PRIVATE_KEY_BASE64`)
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use octofer::Config;
    ///
    /// // Assuming environment variables are set:
    /// // GITHUB_APP_ID=123456
    /// // GITHUB_PRIVATE_KEY_PATH=/path/to/key.pem
    /// // GITHUB_WEBHOOK_SECRET=my-secret
    ///
    /// let config = Config::from_env()?;
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            github: GitHubConfig::from_env()?,
            server: ServerConfig::from_env(),
            webhook: WebhookConfig::from_env(),
            logging: LoggingConfig::from_env(),
        })
    }

    /// Create a new configuration with custom values
    ///
    /// Creates a configuration with explicitly provided values instead of
    /// reading from environment variables. This is useful for testing or
    /// when configuration comes from other sources.
    ///
    /// # Arguments
    ///
    /// * `app_id` - GitHub App ID
    /// * `private_key_path` - Optional path to PEM private key file
    /// * `private_key_base64` - Optional base64-encoded private key
    /// * `webhook_secret` - Secret for webhook HMAC verification
    /// * `host` - Host address to bind server to
    /// * `port` - Port for server to listen on
    ///
    /// # Returns
    ///
    /// Returns `Ok(Config)` if the private key can be loaded, or `Err` if
    /// the private key file cannot be read or decoded.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - Both `private_key_path` and `private_key_base64` are `None`
    /// - Private key file cannot be read (if using `private_key_path`)
    /// - Private key cannot be decoded (if using `private_key_base64`)
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use octofer::Config;
    /// use std::net::Ipv4Addr;
    ///
    /// // Using private key file
    /// let config = Config::new(
    ///     123456,
    ///     Some("private-key.pem".to_string()),
    ///     None,
    ///     "webhook-secret".to_string(),
    ///     Ipv4Addr::LOCALHOST,
    ///     8000,
    /// )?;
    ///
    /// // Using base64-encoded private key
    /// let config = Config::new(
    ///     123456,
    ///     None,
    ///     Some("LS0tLS1CRUdJTi...".to_string()),
    ///     "webhook-secret".to_string(),
    ///     Ipv4Addr::new(0, 0, 0, 0),
    ///     3000,
    /// )?;
    /// # Ok::<(), anyhow::Error>(())
    /// ```
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
    ///
    /// Sets up the tracing subscriber using the logging configuration.
    /// This should be called early in your application startup.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use octofer::Config;
    ///
    /// let config = Config::from_env().unwrap_or_default();
    /// config.init_logging(); // Initialize logging before any other operations
    /// ```
    pub fn init_logging(&self) {
        self.logging.init_tracing();
    }
}

/// GitHub App configuration
///
/// Contains the GitHub App ID and private key needed for authentication.
/// The private key is stored as raw bytes and can be loaded from either
/// a PEM file or a base64-encoded string.
///
/// # Examples
///
/// ```rust,no_run
/// use octofer::config::GitHubConfig;
///
/// // Load from environment variables
/// let config = GitHubConfig::from_env()?;
///
/// // Create with explicit values
/// let config = GitHubConfig::new(
///     123456,
///     Some("private-key.pem".to_string()),
///     None,
/// )?;
/// # Ok::<(), anyhow::Error>(())
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GitHubConfig {
    /// GitHub App ID (found in your GitHub App settings)
    pub app_id: u64,
    /// Private key as bytes (loaded from PEM file or base64 string)
    pub private_key: Vec<u8>,
}

impl GitHubConfig {
    /// Create GitHub configuration from environment variables
    ///
    /// Loads the GitHub App ID and private key from environment variables.
    /// Requires `GITHUB_APP_ID` and either `GITHUB_PRIVATE_KEY_PATH` or
    /// `GITHUB_PRIVATE_KEY_BASE64`.
    ///
    /// # Environment Variables
    ///
    /// * `GITHUB_APP_ID` - Your GitHub App ID (required)
    /// * `GITHUB_PRIVATE_KEY_PATH` - Path to PEM private key file (optional if base64 is set)
    /// * `GITHUB_PRIVATE_KEY_BASE64` - Base64-encoded private key (optional if path is set)
    ///
    /// # Returns
    ///
    /// Returns `Ok(GitHubConfig)` if all required environment variables are present
    /// and valid, or `Err` if any are missing or invalid.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - `GITHUB_APP_ID` is not set or not a valid number
    /// - Neither `GITHUB_PRIVATE_KEY_PATH` nor `GITHUB_PRIVATE_KEY_BASE64` is set
    /// - Private key file cannot be read
    /// - Private key cannot be decoded from base64
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use octofer::config::GitHubConfig;
    ///
    /// // Set environment variables first:
    /// // GITHUB_APP_ID=123456
    /// // GITHUB_PRIVATE_KEY_PATH=/path/to/private-key.pem
    ///
    /// let config = GitHubConfig::from_env()?;
    /// assert_eq!(config.app_id, 123456);
    /// # Ok::<(), anyhow::Error>(())
    /// ```
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
    ///
    /// Creates a GitHub configuration with provided values instead of reading
    /// from environment variables. Either `private_key_path` or `private_key_base64`
    /// must be provided (but not both).
    ///
    /// # Arguments
    ///
    /// * `app_id` - GitHub App ID
    /// * `private_key_path` - Optional path to PEM private key file
    /// * `private_key_base64` - Optional base64-encoded private key
    ///
    /// # Returns
    ///
    /// Returns `Ok(GitHubConfig)` if the private key can be loaded, or `Err`
    /// if both key options are None or if the key cannot be loaded.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - Both `private_key_path` and `private_key_base64` are `None`
    /// - Private key file cannot be read
    /// - Private key cannot be decoded from base64
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use octofer::config::GitHubConfig;
    ///
    /// // Using private key file
    /// let config = GitHubConfig::new(
    ///     123456,
    ///     Some("private-key.pem".to_string()),
    ///     None,
    /// )?;
    ///
    /// // Using base64-encoded key
    /// let config = GitHubConfig::new(
    ///     123456,
    ///     None,
    ///     Some("LS0tLS1CRUdJTi...".to_string()),
    /// )?;
    /// # Ok::<(), anyhow::Error>(())
    /// ```
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
///
/// Specifies the host address and port for the webhook server to bind to.
/// Can be configured via environment variables or set explicitly.
///
/// # Examples
///
/// ```rust
/// use octofer::config::ServerConfig;
/// use std::net::Ipv4Addr;
///
/// // Use defaults (127.0.0.1:8000)
/// let config = ServerConfig::default();
///
/// // Load from environment variables
/// let config = ServerConfig::from_env();
///
/// // Create with explicit values
/// let config = ServerConfig {
///     host: Ipv4Addr::new(0, 0, 0, 0), // Bind to all interfaces
///     port: 3000,
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Host address to bind to (e.g., 127.0.0.1 for localhost, 0.0.0.0 for all interfaces)
    pub host: Ipv4Addr,
    /// Port to listen on (e.g., 8000, 3000, 80, 443)
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
    ///
    /// Loads server configuration from `OCTOFER_HOST` and `OCTOFER_PORT`
    /// environment variables. If not set, uses sensible defaults.
    ///
    /// # Environment Variables
    ///
    /// * `OCTOFER_HOST` - Host address (default: 127.0.0.1)
    /// * `OCTOFER_PORT` - Port number (default: 8000)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use octofer::config::ServerConfig;
    ///
    /// // Will use environment variables if set, otherwise defaults
    /// let config = ServerConfig::from_env();
    /// ```
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
///
/// Contains the webhook secret for HMAC verification and the header name
/// where GitHub sends the HMAC signature. These settings ensure webhook
/// requests are authentic and come from GitHub.
///
/// # Security Note
///
/// The webhook secret should be a cryptographically secure random string.
/// Never use the default secret in production environments.
///
/// # Examples
///
/// ```rust
/// use octofer::config::WebhookConfig;
///
/// // Use defaults (for development only)
/// let config = WebhookConfig::default();
///
/// // Load from environment variables
/// let config = WebhookConfig::from_env();
///
/// // Create with explicit values
/// let config = WebhookConfig {
///     secret: "my-secure-webhook-secret".to_string(),
///     header_name: "X-Hub-Signature-256".to_string(),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookConfig {
    /// Webhook secret for HMAC verification (should be cryptographically secure)
    pub secret: String,
    /// Header name for HMAC signature (typically "X-Hub-Signature-256")
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
    ///
    /// Loads webhook configuration from environment variables with fallback
    /// to defaults if not set.
    ///
    /// # Environment Variables
    ///
    /// * `GITHUB_WEBHOOK_SECRET` - Webhook secret (default: "octofer-webhook-secret")
    /// * `GITHUB_WEBHOOK_HEADER_NAME` - Header name (default: "X-Hub-Signature-256")
    ///
    /// # Security Warning
    ///
    /// If `GITHUB_WEBHOOK_SECRET` is not set, a default development secret
    /// will be used. This is **insecure** for production use.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use octofer::config::WebhookConfig;
    ///
    /// // Load from environment, with defaults if not set
    /// let config = WebhookConfig::from_env();
    /// ```
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
///
/// Controls the behavior of the tracing/logging system, including log level,
/// format, and additional information to include in log messages.
///
/// # Log Levels
///
/// - `trace` - Very verbose, includes all events
/// - `debug` - Detailed information for debugging
/// - `info` - General information (default)
/// - `warn` - Warning messages
/// - `error` - Error messages only
///
/// # Log Formats
///
/// - `compact` - Concise single-line format (default)
/// - `pretty` - Multi-line format with colors and indentation
/// - `json` - JSON format for structured logging
///
/// # Examples
///
/// ```rust,no_run
/// use octofer::config::LoggingConfig;
///
/// // Use defaults (info level, compact format)
/// let config = LoggingConfig::default();
/// config.init_tracing();
///
/// // Load from environment variables
/// let config = LoggingConfig::from_env();
/// config.init_tracing();
///
/// // Create with explicit values
/// let config = LoggingConfig {
///     level: "debug".to_string(),
///     format: "pretty".to_string(),
///     with_target: true,
///     with_file: false,
///     with_thread_ids: false,
/// };
/// config.init_tracing();
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Log level (trace, debug, info, warn, error)
    pub level: String,
    /// Log format (compact, pretty, json)
    pub format: String,
    /// Whether to include target information (module paths) in logs
    pub with_target: bool,
    /// Whether to include file and line information in logs
    pub with_file: bool,
    /// Whether to include thread information in logs
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
    ///
    /// Loads logging configuration from environment variables with fallback
    /// to sensible defaults if not set.
    ///
    /// # Environment Variables
    ///
    /// * `OCTOFER_LOG_LEVEL` - Log level (default: "info")
    /// * `OCTOFER_LOG_FORMAT` - Log format (default: "compact")
    /// * `OCTOFER_LOG_WITH_TARGET` - Include target info (default: false)
    /// * `OCTOFER_LOG_WITH_FILE` - Include file/line info (default: false)
    /// * `OCTOFER_LOG_WITH_THREAD_IDS` - Include thread IDs (default: false)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use octofer::config::LoggingConfig;
    ///
    /// // Load from environment, with defaults if not set
    /// let config = LoggingConfig::from_env();
    /// config.init_tracing();
    /// ```
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
    ///
    /// Sets up the global tracing subscriber using the configuration settings.
    /// This should be called once at application startup, before any logging occurs.
    ///
    /// # Format Options
    ///
    /// - `"compact"` - Single-line format with minimal information
    /// - `"pretty"` - Multi-line format with colors and indentation
    /// - `"json"` - JSON format for structured logging
    /// - Any other value defaults to compact format
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use octofer::config::LoggingConfig;
    ///
    /// let config = LoggingConfig::default();
    /// config.init_tracing(); // Must be called before any logging
    ///
    /// // Now you can use tracing macros
    /// tracing::info!("Application started");
    /// ```
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
