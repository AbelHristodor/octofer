//! Check and security scanning event handlers
//!
//! This module provides implementations for registering handlers for GitHub check runs,
//! code scanning, and security-related webhook events.

use std::sync::Arc;

use octocrab::models::webhook_events::WebhookEventType;

use crate::{Context, Octofer, SerdeToString};

impl Octofer {
    /// Register a handler for check run events
    pub async fn on_check_run<F, Fut, E>(&mut self, handler: F, extra: Arc<E>) -> &Self
    where
        F: Fn(Context, Arc<E>) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = anyhow::Result<()>> + Send + 'static,
        E: Send + Sync + 'static,
    {
        self.server
            .on(WebhookEventType::CheckRun.to_string(), handler, extra)
            .await;
        self
    }

    /// Register a handler for check suite events
    pub async fn on_check_suite<F, Fut, E>(&mut self, handler: F, extra: Arc<E>) -> &Self
    where
        F: Fn(Context, Arc<E>) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = anyhow::Result<()>> + Send + 'static,
        E: Send + Sync + 'static,
    {
        self.server
            .on(WebhookEventType::CheckSuite.to_string(), handler, extra)
            .await;
        self
    }

    /// Register a handler for code scanning alert events
    pub async fn on_code_scanning_alert<F, Fut, E>(&mut self, handler: F, extra: Arc<E>) -> &Self
    where
        F: Fn(Context, Arc<E>) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = anyhow::Result<()>> + Send + 'static,
        E: Send + Sync + 'static,
    {
        self.server
            .on(
                WebhookEventType::CodeScanningAlert.to_string(),
                handler,
                extra,
            )
            .await;
        self
    }

    /// Register a handler for secret scanning alert events
    pub async fn on_secret_scanning_alert<F, Fut, E>(&mut self, handler: F, extra: Arc<E>) -> &Self
    where
        F: Fn(Context, Arc<E>) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = anyhow::Result<()>> + Send + 'static,
        E: Send + Sync + 'static,
    {
        self.server
            .on(
                WebhookEventType::SecretScanningAlert.to_string(),
                handler,
                extra,
            )
            .await;
        self
    }

    /// Register a handler for secret scanning alert location events
    pub async fn on_secret_scanning_alert_location<F, Fut, E>(
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
                WebhookEventType::SecretScanningAlertLocation.to_string(),
                handler,
                extra,
            )
            .await;
        self
    }

    /// Register a handler for dependabot alert events
    pub async fn on_dependabot_alert<F, Fut, E>(&mut self, handler: F, extra: Arc<E>) -> &Self
    where
        F: Fn(Context, Arc<E>) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = anyhow::Result<()>> + Send + 'static,
        E: Send + Sync + 'static,
    {
        self.server
            .on(
                WebhookEventType::DependabotAlert.to_string(),
                handler,
                extra,
            )
            .await;
        self
    }

    /// Register a handler for repository vulnerability alert events
    pub async fn on_repository_vulnerability_alert<F, Fut, E>(
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
                WebhookEventType::RepositoryVulnerabilityAlert.to_string(),
                handler,
                extra,
            )
            .await;
        self
    }

    /// Register a handler for security advisory events
    pub async fn on_security_advisory<F, Fut, E>(&mut self, handler: F, extra: Arc<E>) -> &Self
    where
        F: Fn(Context, Arc<E>) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = anyhow::Result<()>> + Send + 'static,
        E: Send + Sync + 'static,
    {
        self.server
            .on(
                WebhookEventType::SecurityAdvisory.to_string(),
                handler,
                extra,
            )
            .await;
        self
    }

    /// Register a handler for repository advisory events
    pub async fn on_repository_advisory<F, Fut, E>(&mut self, handler: F, extra: Arc<E>) -> &Self
    where
        F: Fn(Context, Arc<E>) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = anyhow::Result<()>> + Send + 'static,
        E: Send + Sync + 'static,
    {
        self.server
            .on(
                WebhookEventType::RepositoryAdvisory.to_string(),
                handler,
                extra,
            )
            .await;
        self
    }

    /// Register a handler for security and analysis events
    pub async fn on_security_and_analysis<F, Fut, E>(&mut self, handler: F, extra: Arc<E>) -> &Self
    where
        F: Fn(Context, Arc<E>) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = anyhow::Result<()>> + Send + 'static,
        E: Send + Sync + 'static,
    {
        self.server
            .on(
                WebhookEventType::SecurityAndAnalysis.to_string(),
                handler,
                extra,
            )
            .await;
        self
    }
}
