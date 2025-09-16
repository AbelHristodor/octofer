use std::{collections::HashMap, sync::Arc};

use axum::routing::{get, post};
use axum::{middleware, Router};
use octofer_core::{Context, EventHandlerFn};
use octofer_github::middlewares::{events, hmac};
use tokio::sync::RwLock;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::{info, Level};

mod config;
mod handlers;

const DEFAULT_PORT: u16 = 8000;
const DEFAULT_ADDRESS: std::net::Ipv4Addr = std::net::Ipv4Addr::LOCALHOST;

#[derive(Clone, Default)]
pub struct AppState {
    handlers: Arc<RwLock<HashMap<String, Vec<EventHandlerFn>>>>,
}

pub struct WebhookServer {
    state: AppState,
    pub address: std::net::Ipv4Addr,
    pub port: u16,
}

impl Default for WebhookServer {
    fn default() -> Self {
        Self {
            state: Default::default(),
            address: DEFAULT_ADDRESS,
            port: DEFAULT_PORT,
        }
    }
}

impl WebhookServer {
    pub fn new(address: std::net::Ipv4Addr, port: u16) -> Self {
        let gh_client = Arc::new(octofer_github::Github::new().await);
        Self {
            state: AppState {
                handlers: Arc::new(RwLock::new(HashMap::new())),
            },
            address,
            port,
        }
    }

    pub async fn start(&self) -> anyhow::Result<()> {
        let listener = tokio::net::TcpListener::bind((self.address, self.port)).await?;
        info!("Server started on {}", listener.local_addr().unwrap());

        axum::serve(listener, self.get_router()).await?;
        Ok(())
    }

    pub async fn on<F, Fut>(&mut self, event: impl Into<String>, handler: F)
    where
        F: Fn(Context) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = anyhow::Result<()>> + Send + Sync + 'static,
    {
        let event = event.into();
        let boxed_handler: EventHandlerFn = Box::new(move |context| Box::pin(handler(context)));

        // TODO: fix this to make it more readable
        // TODO: add error handling
        Arc::get_mut(&mut self.state.handlers)
            .unwrap()
            .write()
            .await
            .entry(event)
            .or_default()
            .push(boxed_handler);
    }

    fn get_router(&self) -> Router {
        let hmac_state = Arc::new(hmac::HmacConfig {
            secret: "mysecret".into(),
            header_name: "x-hub-signature-256".to_string(), // Default GitHub header
        });
        let state = self.state.clone();
        let cors = tower_http::cors::CorsLayer::new()
            .allow_origin(tower_http::cors::Any)
            .allow_methods(tower_http::cors::Any)
            .allow_headers(tower_http::cors::Any);

        Router::new()
            .route("/health", get(handlers::handle_health))
            .route(
                "/webhook",
                post(handlers::handle_webhook)
                    .layer(middleware::from_fn_with_state(
                        hmac_state,
                        hmac::verify_hmac_middleware,
                    ))
                    .layer(middleware::from_fn(events::github_event)),
            )
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(DefaultMakeSpan::new().include_headers(true))
                    .on_request(DefaultOnRequest::new().level(Level::INFO))
                    .on_response(
                        DefaultOnResponse::new()
                            .level(Level::INFO)
                            .latency_unit(tower_http::LatencyUnit::Micros),
                    ),
            )
            .layer(cors)
            .with_state(state)
    }
}
