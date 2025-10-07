//! Repository and git event handlers
//!
//! This module provides implementations for registering handlers for GitHub repository
//! and git-related webhook events.

use std::sync::Arc;

use octocrab::models::webhook_events::WebhookEventType;

use crate::{Context, Octofer, SerdeToString};

impl Octofer {
    /// Register a handler for push events
    pub async fn on_push<F, Fut, E>(&mut self, handler: F, extra: Arc<E>) -> &Self
    where
        F: Fn(Context, Arc<E>) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = anyhow::Result<()>> + Send + 'static,
        E: Send + Sync + 'static,
    {
        self.server
            .on(WebhookEventType::Push.to_string(), handler, extra)
            .await;
        self
    }

    /// Register a handler for create events (branch/tag created)
    pub async fn on_create<F, Fut, E>(&mut self, handler: F, extra: Arc<E>) -> &Self
    where
        F: Fn(Context, Arc<E>) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = anyhow::Result<()>> + Send + 'static,
        E: Send + Sync + 'static,
    {
        self.server
            .on(WebhookEventType::Create.to_string(), handler, extra)
            .await;
        self
    }

    /// Register a handler for delete events (branch/tag deleted)
    pub async fn on_delete<F, Fut, E>(&mut self, handler: F, extra: Arc<E>) -> &Self
    where
        F: Fn(Context, Arc<E>) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = anyhow::Result<()>> + Send + 'static,
        E: Send + Sync + 'static,
    {
        self.server
            .on(WebhookEventType::Delete.to_string(), handler, extra)
            .await;
        self
    }

    /// Register a handler for fork events
    pub async fn on_fork<F, Fut, E>(&mut self, handler: F, extra: Arc<E>) -> &Self
    where
        F: Fn(Context, Arc<E>) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = anyhow::Result<()>> + Send + 'static,
        E: Send + Sync + 'static,
    {
        self.server
            .on(WebhookEventType::Fork.to_string(), handler, extra)
            .await;
        self
    }

    /// Register a handler for commit comment events
    pub async fn on_commit_comment<F, Fut, E>(&mut self, handler: F, extra: Arc<E>) -> &Self
    where
        F: Fn(Context, Arc<E>) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = anyhow::Result<()>> + Send + 'static,
        E: Send + Sync + 'static,
    {
        self.server
            .on(WebhookEventType::CommitComment.to_string(), handler, extra)
            .await;
        self
    }

    /// Register a handler for gollum events (wiki page updates)
    pub async fn on_gollum<F, Fut, E>(&mut self, handler: F, extra: Arc<E>) -> &Self
    where
        F: Fn(Context, Arc<E>) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = anyhow::Result<()>> + Send + 'static,
        E: Send + Sync + 'static,
    {
        self.server
            .on(WebhookEventType::Gollum.to_string(), handler, extra)
            .await;
        self
    }

    /// Register a handler for public events (repository made public)
    pub async fn on_public<F, Fut, E>(&mut self, handler: F, extra: Arc<E>) -> &Self
    where
        F: Fn(Context, Arc<E>) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = anyhow::Result<()>> + Send + 'static,
        E: Send + Sync + 'static,
    {
        self.server
            .on(WebhookEventType::Public.to_string(), handler, extra)
            .await;
        self
    }

    /// Register a handler for repository events
    pub async fn on_repository<F, Fut, E>(&mut self, handler: F, extra: Arc<E>) -> &Self
    where
        F: Fn(Context, Arc<E>) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = anyhow::Result<()>> + Send + 'static,
        E: Send + Sync + 'static,
    {
        self.server
            .on(WebhookEventType::Repository.to_string(), handler, extra)
            .await;
        self
    }

    /// Register a handler for repository dispatch events
    pub async fn on_repository_dispatch<F, Fut, E>(&mut self, handler: F, extra: Arc<E>) -> &Self
    where
        F: Fn(Context, Arc<E>) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = anyhow::Result<()>> + Send + 'static,
        E: Send + Sync + 'static,
    {
        self.server
            .on(
                WebhookEventType::RepositoryDispatch.to_string(),
                handler,
                extra,
            )
            .await;
        self
    }

    /// Register a handler for repository import events
    pub async fn on_repository_import<F, Fut, E>(&mut self, handler: F, extra: Arc<E>) -> &Self
    where
        F: Fn(Context, Arc<E>) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = anyhow::Result<()>> + Send + 'static,
        E: Send + Sync + 'static,
    {
        self.server
            .on(
                WebhookEventType::RepositoryImport.to_string(),
                handler,
                extra,
            )
            .await;
        self
    }

    /// Register a handler for branch protection rule events
    pub async fn on_branch_protection_rule<F, Fut, E>(&mut self, handler: F, extra: Arc<E>) -> &Self
    where
        F: Fn(Context, Arc<E>) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = anyhow::Result<()>> + Send + 'static,
        E: Send + Sync + 'static,
    {
        self.server
            .on(
                WebhookEventType::BranchProtectionRule.to_string(),
                handler,
                extra,
            )
            .await;
        self
    }
}
