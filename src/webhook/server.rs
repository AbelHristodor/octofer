//! Webhook server implementation

use anyhow::Result;
use axum::routing::{get, post};
use axum::{middleware, Router};
use std::{collections::HashMap, net::Ipv4Addr, sync::Arc};
use tokio::sync::RwLock;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::{info, Level};

use crate::config::GitHubConfig;
use crate::core::{Context, EventHandlerFn};
use crate::github::{
    middlewares::{github_event_middleware, verify_hmac_middleware, HmacConfig},
    GitHubAuth, GitHubClient,
};

use super::handlers;

const DEFAULT_HOST: Ipv4Addr = Ipv4Addr::LOCALHOST;
const DEFAULT_PORT: u16 = 8000;

pub type WebhookEventKind = String;

/// Application state shared across handlers
#[derive(Clone, Default)]
pub struct AppState {
    /// Event handlers mapped by event type
    pub handlers: Arc<RwLock<HashMap<WebhookEventKind, Vec<EventHandlerFn>>>>,
    /// GitHub client for API operations
    pub github_client: Option<Arc<GitHubClient>>,
}

/// Webhook server for handling GitHub webhook events
pub struct WebhookServer {
    /// Application state
    state: AppState,
    /// Server host address
    pub host: Ipv4Addr,
    /// Server port
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
    pub async fn new(host: Ipv4Addr, port: u16, github_config: GitHubConfig) -> Result<Self> {
        let auth = GitHubAuth::from_config(&github_config);
        let github_client = Arc::new(GitHubClient::new(auth).await?);

        let state = AppState {
            handlers: Arc::new(RwLock::new(HashMap::new())),
            github_client: Some(github_client),
        };

        let hmac_config = Arc::new(HmacConfig::default());

        Ok(Self {
            state,
            host,
            port,
            hmac_config,
        })
    }

    /// Create a new webhook server with default configuration
    pub fn new_default() -> Self {
        let state = AppState {
            handlers: Arc::new(RwLock::new(HashMap::new())),
            github_client: None,
        };

        let hmac_config = Arc::new(HmacConfig::default());

        Self {
            state,
            host: DEFAULT_HOST,
            port: DEFAULT_PORT,
            hmac_config,
        }
    }

    /// Start the webhook server
    pub async fn start(&self) -> Result<()> {
        let listener = tokio::net::TcpListener::bind((self.host, self.port)).await?;
        info!("Webhook server started on {}:{}", self.host, self.port);

        axum::serve(listener, self.create_router()).await?;
        Ok(())
    }

    /// Register an event handler for a specific event type
    pub async fn on<F, Fut>(&mut self, event: impl Into<String>, handler: F)
    where
        F: Fn(Context) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Result<()>> + Send + Sync + 'static,
    {
        let event = event.into();
        let boxed_handler: EventHandlerFn = Box::new(move |context| Box::pin(handler(context)));

        self.state
            .handlers
            .write()
            .await
            .entry(event)
            .or_default()
            .push(boxed_handler);
    }

    /// Create the axum router with all routes and middleware
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
    pub fn github_client(&self) -> Option<&Arc<GitHubClient>> {
        self.state.github_client.as_ref()
    }

    /// Update HMAC configuration
    pub fn set_hmac_config(&mut self, config: HmacConfig) {
        self.hmac_config = Arc::new(config);
    }
}
