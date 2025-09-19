//! Pull request event handlers
//!
//! This module provides implementations for registering handlers for GitHub pull request
//! and related webhook events.

use std::sync::Arc;

use octocrab::models::webhook_events::WebhookEventType;

use crate::{Context, Octofer, SerdeToString};

impl Octofer {
    /// Register a handler for pull request events
    ///
    /// This method registers an event handler that will be called whenever a pull request
    /// event is received. Pull request events are triggered when PRs are opened, closed,
    /// merged, edited, and more.
    ///
    /// # Arguments
    ///
    /// * `handler` - An async function that takes a Context and extra data
    /// * `extra` - Additional data to pass to the handler (wrapped in Arc for sharing)
    ///
    /// # Event Types
    ///
    /// Pull request events include:
    /// - PR opened
    /// - PR closed (merged or unmerged)
    /// - PR edited
    /// - PR assigned/unassigned
    /// - PR review requested
    /// - PR labeled/unlabeled
    /// - PR synchronized (new commits pushed)
    ///
    /// # Examples
    ///
    /// ## Basic Pull Request Handler
    ///
    /// ```rust,no_run
    /// use octofer::{Octofer, Config, Context};
    /// use std::sync::Arc;
    ///
    /// # async fn example() -> anyhow::Result<()> {
    /// let config = Config::default();
    /// let mut app = Octofer::new(config).await.unwrap_or_else(|_| Octofer::new_default());
    ///
    /// app.on_pull_request(
    ///     |context: Context, _extra: Arc<()>| async move {
    ///         println!("Pull request event: {}", context.kind());
    ///         
    ///         let payload = context.payload();
    ///         if let Some(action) = payload.get("action") {
    ///             println!("PR action: {}", action);
    ///         }
    ///         
    ///         if let Some(pr) = payload.get("pull_request") {
    ///             if let Some(title) = pr.get("title") {
    ///                 println!("PR title: {}", title);
    ///             }
    ///             if let Some(number) = pr.get("number") {
    ///                 println!("PR number: {}", number);
    ///             }
    ///         }
    ///         
    ///         Ok(())
    ///     },
    ///     Arc::new(()),
    /// ).await;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// ## Auto-merge Handler
    ///
    /// ```rust,no_run
    /// use octofer::{Octofer, Config, Context};
    /// use std::sync::Arc;
    ///
    /// # async fn example() -> anyhow::Result<()> {
    /// let config = Config::from_env().unwrap_or_default();
    /// let mut app = Octofer::new(config).await.unwrap_or_else(|_| Octofer::new_default());
    ///
    /// app.on_pull_request(
    ///     |context: Context, _extra: Arc<()>| async move {
    ///         let payload = context.payload();
    ///         
    ///         // Only process newly opened PRs
    ///         if payload.get("action").and_then(|v| v.as_str()) != Some("opened") {
    ///             return Ok(());
    ///         }
    ///         
    ///         if let Some(pr) = payload.get("pull_request") {
    ///             // Check if it's a dependabot PR
    ///             if let Some(user) = pr.get("user").and_then(|u| u.get("login")) {
    ///                 if user.as_str() == Some("dependabot[bot]") {
    ///                     println!("Dependabot PR detected, would auto-approve");
    ///                     // Use installation client to approve PR
    ///                 }
    ///             }
    ///         }
    ///         
    ///         Ok(())
    ///     },
    ///     Arc::new(()),
    /// ).await;
    /// # Ok(())
    /// # }
    /// ```
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

    /// Register a handler for pull request review events
    ///
    /// This method registers an event handler that will be called whenever a pull request
    /// review is submitted. This includes approvals, change requests, and comment-only reviews.
    ///
    /// # Arguments
    ///
    /// * `handler` - An async function that takes a Context and extra data
    /// * `extra` - Additional data to pass to the handler (wrapped in Arc for sharing)
    ///
    /// # Event Types
    ///
    /// Pull request review events include:
    /// - Review submitted (approved, changes requested, or commented)
    /// - Review edited
    /// - Review dismissed
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use octofer::{Octofer, Config, Context};
    /// use std::sync::Arc;
    ///
    /// # async fn example() -> anyhow::Result<()> {
    /// let config = Config::default();
    /// let mut app = Octofer::new(config).await.unwrap_or_else(|_| Octofer::new_default());
    ///
    /// app.on_pull_request_review(
    ///     |context: Context, _extra: Arc<()>| async move {
    ///         println!("Pull request review event: {}", context.kind());
    ///         
    ///         let payload = context.payload();
    ///         if let Some(review) = payload.get("review") {
    ///             if let Some(state) = review.get("state") {
    ///                 println!("Review state: {}", state);
    ///             }
    ///             if let Some(body) = review.get("body") {
    ///                 println!("Review body: {}", body);
    ///             }
    ///         }
    ///         
    ///         Ok(())
    ///     },
    ///     Arc::new(()),
    /// ).await;
    /// # Ok(())
    /// # }
    /// ```
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

    /// Register a handler for pull request review comment events
    ///
    /// This method registers an event handler that will be called whenever a comment
    /// is made during a pull request review. These are line-specific comments on the
    /// code changes in the PR.
    ///
    /// # Arguments
    ///
    /// * `handler` - An async function that takes a Context and extra data
    /// * `extra` - Additional data to pass to the handler (wrapped in Arc for sharing)
    ///
    /// # Event Types
    ///
    /// Pull request review comment events include:
    /// - Review comment created
    /// - Review comment edited
    /// - Review comment deleted
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use octofer::{Octofer, Config, Context};
    /// use std::sync::Arc;
    ///
    /// # async fn example() -> anyhow::Result<()> {
    /// let config = Config::default();
    /// let mut app = Octofer::new(config).await.unwrap_or_else(|_| Octofer::new_default());
    ///
    /// app.on_pull_request_review_comment(
    ///     |context: Context, _extra: Arc<()>| async move {
    ///         println!("Pull request review comment event: {}", context.kind());
    ///         
    ///         let payload = context.payload();
    ///         if let Some(comment) = payload.get("comment") {
    ///             if let Some(body) = comment.get("body") {
    ///                 println!("Review comment: {}", body);
    ///             }
    ///             if let Some(path) = comment.get("path") {
    ///                 println!("File: {}", path);
    ///             }
    ///             if let Some(line) = comment.get("line") {
    ///                 println!("Line: {}", line);
    ///             }
    ///         }
    ///         
    ///         Ok(())
    ///     },
    ///     Arc::new(()),
    /// ).await;
    /// # Ok(())
    /// # }
    /// ```
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

    /// Register a handler for pull request review thread events
    ///
    /// This method registers an event handler that will be called whenever a pull request
    /// review thread is resolved or unresolved. Review threads are created when someone
    /// starts a conversation on a specific line of code.
    ///
    /// # Arguments
    ///
    /// * `handler` - An async function that takes a Context and extra data
    /// * `extra` - Additional data to pass to the handler (wrapped in Arc for sharing)
    ///
    /// # Event Types
    ///
    /// Pull request review thread events include:
    /// - Thread resolved
    /// - Thread unresolved
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use octofer::{Octofer, Config, Context};
    /// use std::sync::Arc;
    ///
    /// # async fn example() -> anyhow::Result<()> {
    /// let config = Config::default();
    /// let mut app = Octofer::new(config).await.unwrap_or_else(|_| Octofer::new_default());
    ///
    /// app.on_pull_request_review_thread(
    ///     |context: Context, _extra: Arc<()>| async move {
    ///         println!("Pull request review thread event: {}", context.kind());
    ///         
    ///         let payload = context.payload();
    ///         if let Some(action) = payload.get("action") {
    ///             println!("Thread action: {}", action);
    ///         }
    ///         
    ///         if let Some(thread) = payload.get("thread") {
    ///             if let Some(node_id) = thread.get("node_id") {
    ///                 println!("Thread ID: {}", node_id);
    ///             }
    ///         }
    ///         
    ///         Ok(())
    ///     },
    ///     Arc::new(()),
    /// ).await;
    /// # Ok(())
    /// # }
    /// ```
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
