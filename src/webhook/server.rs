//! Webhook server implementation
//!
//! This module provides the HTTP server for receiving GitHub webhook events.
//! It handles HMAC verification, event processing, and routing to registered handlers.

use anyhow::Result;
use axum::routing::{get, post};
use axum::{middleware, Router};
use std::{collections::HashMap, net::Ipv4Addr, sync::Arc};
use tokio::sync::RwLock;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::{info, Level};

use crate::config::{GitHubConfig, DEFAULT_HOST_ADDR, DEFAULT_PORT};
use crate::core::{Context, EventHandlerFn};
use crate::github::{
    middlewares::{github_event_middleware, verify_hmac_middleware, HmacConfig},
    GitHubAuth, GitHubClient,
};

use super::handlers;

/// Type alias for webhook event kinds (event type strings)
pub type WebhookEventKind = String;

/// Application state shared across handlers
///
/// This struct contains the shared state that all webhook handlers can access,
/// including registered event handlers and the GitHub API client.
#[derive(Clone, Default)]
pub struct AppState {
    /// Event handlers mapped by event type (e.g., "issues", "pull_request")
    pub handlers: Arc<RwLock<HashMap<WebhookEventKind, Vec<EventHandlerFn>>>>,
    /// GitHub client for API operations (if available)
    pub github_client: Option<Arc<GitHubClient>>,
}

/// Webhook server for handling GitHub webhook events
///
/// The WebhookServer is responsible for receiving HTTP webhook requests from GitHub,
/// verifying their authenticity, and routing them to the appropriate event handlers.
///
/// # Features
///
/// - **HMAC Verification** - Validates webhook requests using the shared secret
/// - **Event Routing** - Routes events to handlers based on event type
/// - **GitHub Client Integration** - Provides authenticated API access to handlers
/// - **Health Checks** - Provides a health check endpoint for monitoring
/// - **Request Tracing** - Logs all incoming requests for debugging
///
/// # Examples
///
/// ## Creating and Starting a Server
///
/// ```rust,no_run
/// use octofer::{Config, webhook::WebhookServer};
/// use std::net::Ipv4Addr;
///
/// # async fn example() -> anyhow::Result<()> {
/// let config = Config::from_env()?;
///
/// let server = WebhookServer::new(
///     config.server.host,
///     config.server.port,
///     config.github.clone(),
///     &config.webhook.secret,
///     &config.webhook.header_name,
/// ).await?;
///
/// println!("Starting webhook server on {}:{}", server.host, server.port);
/// server.start().await?;
/// # Ok(())
/// # }
/// ```
///
/// ## Default Server Configuration
///
/// ```rust
/// use octofer::webhook::WebhookServer;
///
/// // Creates a server with default settings (localhost:8000, no GitHub client)
/// let server = WebhookServer::new_default();
/// assert_eq!(server.host.to_string(), "127.0.0.1");
/// assert_eq!(server.port, 8000);
/// ```
pub struct WebhookServer {
    /// Application state containing handlers and GitHub client
    state: AppState,
    /// Server host address to bind to
    pub host: Ipv4Addr,
    /// Server port to listen on
    pub port: u16,
    /// HMAC configuration for webhook verification
    hmac_config: Arc<HmacConfig>,
}

impl Default for WebhookServer {
    fn default() -> Self {
        Self::new_default()
    }
}

impl WebhookServer {
    /// Create a new webhook server with the provided configuration
    ///
    /// Creates a webhook server with GitHub App authentication and HMAC verification.
    /// The server will be ready to receive webhook events and route them to handlers.
    ///
    /// # Arguments
    ///
    /// * `host` - IP address to bind the server to
    /// * `port` - Port number to listen on
    /// * `github_config` - GitHub App configuration for authentication
    /// * `secret` - Webhook secret for HMAC verification
    /// * `hmac_header` - Header name containing HMAC signature (typically "X-Hub-Signature-256")
    ///
    /// # Returns
    ///
    /// Returns `Ok(WebhookServer)` if the server was created successfully,
    /// or `Err` if GitHub client creation failed.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use octofer::{Config, webhook::WebhookServer};
    /// use std::net::Ipv4Addr;
    ///
    /// # async fn example() -> anyhow::Result<()> {
    /// let config = Config::from_env()?;
    ///
    /// let server = WebhookServer::new(
    ///     Ipv4Addr::new(0, 0, 0, 0),  // Bind to all interfaces
    ///     3000,                       // Port 3000
    ///     config.github,
    ///     &config.webhook.secret,
    ///     &config.webhook.header_name,
    /// ).await?;
    ///
    /// println!("Server created successfully");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn new(
        host: Ipv4Addr,
        port: u16,
        github_config: GitHubConfig,
        secret: &str,
        hmac_header: &str,
    ) -> Result<Self> {
        let auth = GitHubAuth::from_config(&github_config);
        let github_client = Arc::new(GitHubClient::new(auth).await?);

        let state = AppState {
            handlers: Arc::new(RwLock::new(HashMap::new())),
            github_client: Some(github_client),
        };

        let hmac_config = Arc::new(HmacConfig::new(secret.into(), hmac_header.into()));

        Ok(Self {
            state,
            host,
            port,
            hmac_config,
        })
    }

    /// Create a new webhook server with default configuration
    ///
    /// Creates a webhook server with default settings suitable for development
    /// or testing. No GitHub client is created, so handlers won't have API access.
    ///
    /// # Default Settings
    ///
    /// - Host: 127.0.0.1 (localhost)
    /// - Port: 8000
    /// - No GitHub client
    /// - Default HMAC configuration (development secret)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use octofer::webhook::WebhookServer;
    ///
    /// let server = WebhookServer::new_default();
    /// assert_eq!(server.host.to_string(), "127.0.0.1");
    /// assert_eq!(server.port, 8000);
    /// ```
    pub fn new_default() -> Self {
        let state = AppState {
            handlers: Arc::new(RwLock::new(HashMap::new())),
            github_client: None,
        };

        let hmac_config = Arc::new(HmacConfig::default());

        Self {
            state,
            host: DEFAULT_HOST_ADDR,
            port: DEFAULT_PORT,
            hmac_config,
        }
    }

    /// Start the webhook server
    ///
    /// Starts the HTTP server and begins listening for webhook requests.
    /// This method will block until the server is stopped or an error occurs.
    ///
    /// The server provides two endpoints:
    /// - `POST /webhook` - Receives GitHub webhook events
    /// - `GET /health` - Health check endpoint
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the server stops gracefully, or `Err` if there's
    /// an error starting the server or binding to the specified address.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use octofer::webhook::WebhookServer;
    ///
    /// # async fn example() -> anyhow::Result<()> {
    /// let server = WebhookServer::new_default();
    ///
    /// println!("Starting server on {}:{}", server.host, server.port);
    /// server.start().await?;  // This blocks until server stops
    /// # Ok(())
    /// # }
    /// ```
    pub async fn start(&self) -> Result<()> {
        let listener = tokio::net::TcpListener::bind((self.host, self.port)).await?;
        info!("Webhook server started on {}:{}", self.host, self.port);

        axum::serve(listener, self.create_router()).await?;
        Ok(())
    }

    /// Register an event handler for a specific event type
    ///
    /// Registers a handler function that will be called when webhook events
    /// of the specified type are received. Multiple handlers can be registered
    /// for the same event type.
    ///
    /// # Arguments
    ///
    /// * `event` - The event type to handle (e.g., "issues", "pull_request")
    /// * `handler` - Async function to handle the event
    /// * `extra` - Additional data to pass to the handler (shared across all calls)
    ///
    /// # Handler Signature
    ///
    /// The handler function must have the signature:
    /// ```rust,ignore
    /// async fn handler(context: Context, extra: Arc<ExtraData>) -> anyhow::Result<()>
    /// ```
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use octofer::{webhook::WebhookServer, Context};
    /// use std::sync::Arc;
    ///
    /// # async fn example() -> anyhow::Result<()> {
    /// let mut server = WebhookServer::new_default();
    ///
    /// // Register a handler for issue events
    /// server.on(
    ///     "issues",
    ///     |context: Context, _extra: Arc<()>| async move {
    ///         println!("Issue event received: {}", context.kind());
    ///         Ok(())
    ///     },
    ///     Arc::new(()),
    /// ).await;
    ///
    /// // Register a handler with custom data
    /// let app_name = Arc::new("MyBot".to_string());
    /// server.on(
    ///     "pull_request",
    ///     |context: Context, app_name: Arc<String>| async move {
    ///         println!("{} handling PR event", app_name);
    ///         Ok(())
    ///     },
    ///     app_name,
    /// ).await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn on<F, Fut, E>(&mut self, event: impl Into<String>, handler: F, extra: Arc<E>)
    where
        F: Fn(Context, Arc<E>) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Result<()>> + Send + 'static,
        E: Send + Sync + 'static,
    {
        let event = event.into();
        let boxed_handler: EventHandlerFn = Box::new(move |context| {
            // Clone the extra data for this handler call
            let extra = extra.clone();
            Box::pin(handler(context, extra))
        });

        self.state
            .handlers
            .write()
            .await
            .entry(event)
            .or_default()
            .push(boxed_handler);
    }

    /// Create the axum router with all routes and middleware
    ///
    /// Creates the HTTP router with all endpoints and middleware layers.
    /// This is an internal method used by `start()`.
    ///
    /// # Middleware Stack
    ///
    /// The router includes the following middleware (in order):
    /// 1. **CORS** - Allows cross-origin requests
    /// 2. **Tracing** - Logs all requests and responses
    /// 3. **GitHub Event Processing** - Extracts GitHub event metadata
    /// 4. **HMAC Verification** - Validates webhook authenticity (webhook endpoint only)
    ///
    /// # Endpoints
    ///
    /// - `GET /health` - Health check endpoint (no authentication required)
    /// - `POST /webhook` - Webhook endpoint (requires valid HMAC signature)
    fn create_router(&self) -> Router {
        let cors_layer = tower_http::cors::CorsLayer::new()
            .allow_origin(tower_http::cors::Any)
            .allow_methods(tower_http::cors::Any)
            .allow_headers(tower_http::cors::Any);

        let trace_layer = TraceLayer::new_for_http()
            .make_span_with(DefaultMakeSpan::new().include_headers(true))
            .on_request(DefaultOnRequest::new().level(Level::INFO))
            .on_response(
                DefaultOnResponse::new()
                    .level(Level::INFO)
                    .latency_unit(tower_http::LatencyUnit::Micros),
            );

        Router::new()
            .route("/health", get(handlers::handle_health))
            .route(
                "/webhook",
                post(handlers::handle_webhook)
                    .layer(middleware::from_fn_with_state(
                        self.hmac_config.clone(),
                        verify_hmac_middleware,
                    ))
                    .layer(middleware::from_fn(github_event_middleware)),
            )
            .layer(trace_layer)
            .layer(cors_layer)
            .with_state(self.state.clone())
    }

    /// Get access to the GitHub client
    ///
    /// Returns a reference to the GitHub client if one was configured during
    /// server creation. This can be used for app-level GitHub API operations.
    ///
    /// # Returns
    ///
    /// Returns `Some(client)` if a GitHub client is available, or `None` if
    /// the server was created without GitHub configuration.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use octofer::{Config, webhook::WebhookServer};
    ///
    /// # async fn example() -> anyhow::Result<()> {
    /// let config = Config::from_env()?;
    /// let server = WebhookServer::new(
    ///     config.server.host,
    ///     config.server.port,
    ///     config.github,
    ///     &config.webhook.secret,
    ///     &config.webhook.header_name,
    /// ).await?;
    ///
    /// if let Some(client) = server.github_client() {
    ///     let installations = client.get_installations().await?;
    ///     println!("Found {} installations", installations.len());
    /// } else {
    ///     println!("No GitHub client available");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn github_client(&self) -> Option<&Arc<GitHubClient>> {
        self.state.github_client.as_ref()
    }

    /// Update HMAC configuration
    ///
    /// Updates the HMAC configuration used for webhook verification.
    /// This can be useful for rotating webhook secrets or changing
    /// verification settings.
    ///
    /// # Arguments
    ///
    /// * `config` - New HMAC configuration
    ///
    /// # Examples
    ///
    /// ```rust
    /// use octofer::{webhook::WebhookServer, github::middlewares::HmacConfig};
    ///
    /// let mut server = WebhookServer::new_default();
    ///
    /// // Update to a new webhook secret
    /// let new_config = HmacConfig::new(
    ///     "new-secure-secret".to_string(),
    ///     "X-Hub-Signature-256".to_string(),
    /// );
    /// server.set_hmac_config(new_config);
    /// ```
    pub fn set_hmac_config(&mut self, config: HmacConfig) {
        self.hmac_config = Arc::new(config);
    }
}
