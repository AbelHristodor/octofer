//! Deployment event handlers
//!
//! This module provides implementations for registering handlers for GitHub deployment
//! and deploy key webhook events.

use std::sync::Arc;

use octocrab::models::webhook_events::WebhookEventType;

use crate::{Context, Octofer, SerdeToString};

impl Octofer {
    /// Register a handler for deployment events
    pub async fn on_deployment<F, Fut, E>(&mut self, handler: F, extra: Arc<E>) -> &Self
    where
        F: Fn(Context, Arc<E>) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = anyhow::Result<()>> + Send + 'static,
        E: Send + Sync + 'static,
    {
        self.server
            .on(WebhookEventType::Deployment.to_string(), handler, extra)
            .await;
        self
    }

    /// Register a handler for deployment status events
    pub async fn on_deployment_status<F, Fut, E>(&mut self, handler: F, extra: Arc<E>) -> &Self
    where
        F: Fn(Context, Arc<E>) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = anyhow::Result<()>> + Send + 'static,
        E: Send + Sync + 'static,
    {
        self.server
            .on(
                WebhookEventType::DeploymentStatus.to_string(),
                handler,
                extra,
            )
            .await;
        self
    }

    /// Register a handler for deploy key events
    pub async fn on_deploy_key<F, Fut, E>(&mut self, handler: F, extra: Arc<E>) -> &Self
    where
        F: Fn(Context, Arc<E>) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = anyhow::Result<()>> + Send + 'static,
        E: Send + Sync + 'static,
    {
        self.server
            .on(WebhookEventType::DeployKey.to_string(), handler, extra)
            .await;
        self
    }

    /// Register a handler for deployment protection rule events
    pub async fn on_deployment_protection_rule<F, Fut, E>(
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
                WebhookEventType::DeploymentProtectionRule.to_string(),
                handler,
                extra,
            )
            .await;
        self
    }
}
