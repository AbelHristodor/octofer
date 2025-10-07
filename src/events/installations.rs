//! Installation and GitHub App event handlers
//!
//! This module provides implementations for registering handlers for GitHub App
//! installation and authorization webhook events.

use std::sync::Arc;

use octocrab::models::webhook_events::WebhookEventType;

use crate::{Context, Octofer, SerdeToString};

impl Octofer {
    /// Register a handler for installation events
    pub async fn on_installation<F, Fut, E>(&mut self, handler: F, extra: Arc<E>) -> &Self
    where
        F: Fn(Context, Arc<E>) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = anyhow::Result<()>> + Send + 'static,
        E: Send + Sync + 'static,
    {
        self.server
            .on(WebhookEventType::Installation.to_string(), handler, extra)
            .await;
        self
    }

    /// Register a handler for installation repositories events
    pub async fn on_installation_repositories<F, Fut, E>(
        &mut self,
        handler: F,
        extra: Arc<E>,
    ) -> &Self
    where
        F: Fn(Context, Arc<E>) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = anyhow::Result<()>> + Send + 'static,
        E: Send + Sync + 'static,
    {
        self.server
            .on(
                WebhookEventType::InstallationRepositories.to_string(),
                handler,
                extra,
            )
            .await;
        self
    }

    /// Register a handler for installation target events
    pub async fn on_installation_target<F, Fut, E>(&mut self, handler: F, extra: Arc<E>) -> &Self
    where
        F: Fn(Context, Arc<E>) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = anyhow::Result<()>> + Send + 'static,
        E: Send + Sync + 'static,
    {
        self.server
            .on(
                WebhookEventType::InstallationTarget.to_string(),
                handler,
                extra,
            )
            .await;
        self
    }

    /// Register a handler for GitHub App authorization events
    pub async fn on_github_app_authorization<F, Fut, E>(
        &mut self,
        handler: F,
        extra: Arc<E>,
    ) -> &Self
    where
        F: Fn(Context, Arc<E>) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = anyhow::Result<()>> + Send + 'static,
        E: Send + Sync + 'static,
    {
        self.server
            .on(
                WebhookEventType::GithubAppAuthorization.to_string(),
                handler,
                extra,
            )
            .await;
        self
    }

    /// Register a handler for personal access token request events
    pub async fn on_personal_access_token_request<F, Fut, E>(
        &mut self,
        handler: F,
        extra: Arc<E>,
    ) -> &Self
    where
        F: Fn(Context, Arc<E>) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = anyhow::Result<()>> + Send + 'static,
        E: Send + Sync + 'static,
    {
        self.server
            .on(
                WebhookEventType::PersonalAccessTokenRequest.to_string(),
                handler,
                extra,
            )
            .await;
        self
    }
}
