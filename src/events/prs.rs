use std::sync::Arc;

use octocrab::models::webhook_events::WebhookEventType;

use crate::{Context, Octofer, SerdeToString};

impl Octofer {
    /// Add an issue comment event handler
    pub async fn on_pull_request<F, Fut, E>(&mut self, handler: F, extra: Arc<E>) -> &Self
    where
        F: Fn(Context, Arc<E>) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = anyhow::Result<()>> + Send + 'static,
        E: Send + Sync + 'static,
    {
        self.server
            .on(WebhookEventType::PullRequest.to_string(), handler, extra)
            .await;
        self
    }

    /// Add an issue comment event handler
    pub async fn on_pull_request_review<F, Fut, E>(&mut self, handler: F, extra: Arc<E>) -> &Self
    where
        F: Fn(Context, Arc<E>) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = anyhow::Result<()>> + Send + 'static,
        E: Send + Sync + 'static,
    {
        self.server
            .on(
                WebhookEventType::PullRequestReview.to_string(),
                handler,
                extra,
            )
            .await;
        self
    }

    /// Add an issue comment event handler
    pub async fn on_pull_request_review_comment<F, Fut, E>(
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
                WebhookEventType::PullRequestReviewComment.to_string(),
                handler,
                extra,
            )
            .await;
        self
    }

    /// Add an issue comment event handler
    pub async fn on_pull_request_review_thread<F, Fut, E>(
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
                WebhookEventType::PullRequestReviewThread.to_string(),
                handler,
                extra,
            )
            .await;
        self
    }
}
