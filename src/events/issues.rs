//! Issue and issue comment event handlers
//!
//! This module provides implementations for registering handlers for GitHub issue
//! and issue comment webhook events.

use std::sync::Arc;

use octocrab::models::webhook_events::WebhookEventType;

use crate::{Context, Octofer, SerdeToString};

impl Octofer {
    /// Register a handler for issue comment events
    ///
    /// This method registers an event handler that will be called whenever an issue
    /// comment event is received. Issue comment events are triggered when comments
    /// are created, edited, or deleted on issues.
    ///
    /// # Arguments
    ///
    /// * `handler` - An async function that takes a Context and extra data
    /// * `extra` - Additional data to pass to the handler (wrapped in Arc for sharing)
    ///
    /// # Event Types
    ///
    /// Issue comment events include:
    /// - Comment created
    /// - Comment edited  
    /// - Comment deleted
    ///
    /// # Examples
    ///
    /// ## Basic Issue Comment Handler
    ///
    /// ```rust,no_run
    /// use octofer::{Octofer, Config, Context};
    /// use std::sync::Arc;
    ///
    /// # async fn example() -> anyhow::Result<()> {
    /// let config = Config::default();
    /// let mut app = Octofer::new(config).await.unwrap_or_else(|_| Octofer::new_default());
    ///
    /// app.on_issue_comment(
    ///     |context: Context, _extra: Arc<()>| async move {
    ///         println!("Issue comment event: {}", context.kind());
    ///         
    ///         let payload = context.payload();
    ///         if let Some(action) = payload.get("action") {
    ///             println!("Comment action: {}", action);
    ///         }
    ///         
    ///         if let Some(comment) = payload.get("comment") {
    ///             if let Some(body) = comment.get("body") {
    ///                 println!("Comment body: {}", body);
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
    /// ## Responding to Issue Comments
    ///
    /// ```rust,no_run
    /// use octofer::{Octofer, Config, Context};
    /// use std::sync::Arc;
    ///
    /// # async fn example() -> anyhow::Result<()> {
    /// let config = Config::from_env().unwrap_or_default();
    /// let mut app = Octofer::new(config).await.unwrap_or_else(|_| Octofer::new_default());
    ///
    /// app.on_issue_comment(
    ///     |context: Context, _extra: Arc<()>| async move {
    ///         // Only respond to created comments
    ///         let payload = context.payload();
    ///         if payload.get("action").and_then(|v| v.as_str()) != Some("created") {
    ///             return Ok(());
    ///         }
    ///         
    ///         // Check if comment mentions the bot
    ///         if let Some(comment_body) = payload.get("comment")
    ///             .and_then(|c| c.get("body"))
    ///             .and_then(|b| b.as_str())
    ///         {
    ///             if comment_body.contains("@bot") {
    ///                 println!("Bot was mentioned in comment: {}", comment_body);
    ///                 
    ///                 // Use installation client to respond
    ///                 if let Some(installation_client) = context.installation_client().await? {
    ///                     // Would create a response comment here
    ///                     println!("Could respond using installation client");
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
    pub async fn on_issue_comment<F, Fut, E>(&mut self, handler: F, extra: Arc<E>) -> &Self
    where
        F: Fn(Context, Arc<E>) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = anyhow::Result<()>> + Send + 'static,
        E: Send + Sync + 'static,
    {
        self.server
            .on(WebhookEventType::IssueComment.to_string(), handler, extra)
            .await;
        self
    }

    /// Register a handler for issue events
    ///
    /// This method registers an event handler that will be called whenever an issue
    /// event is received. Issue events are triggered when issues are opened, closed,
    /// edited, assigned, labeled, and more.
    ///
    /// # Arguments
    ///
    /// * `handler` - An async function that takes a Context and extra data
    /// * `extra` - Additional data to pass to the handler (wrapped in Arc for sharing)
    ///
    /// # Event Types
    ///
    /// Issue events include:
    /// - Issue opened
    /// - Issue closed
    /// - Issue edited
    /// - Issue assigned/unassigned
    /// - Issue labeled/unlabeled
    /// - Issue locked/unlocked
    /// - Issue transferred
    /// - Issue pinned/unpinned
    ///
    /// # Examples
    ///
    /// ## Basic Issue Handler
    ///
    /// ```rust,no_run
    /// use octofer::{Octofer, Config, Context};
    /// use std::sync::Arc;
    ///
    /// # async fn example() -> anyhow::Result<()> {
    /// let config = Config::default();
    /// let mut app = Octofer::new(config).await.unwrap_or_else(|_| Octofer::new_default());
    ///
    /// app.on_issue(
    ///     |context: Context, _extra: Arc<()>| async move {
    ///         println!("Issue event: {}", context.kind());
    ///         
    ///         let payload = context.payload();
    ///         if let Some(action) = payload.get("action") {
    ///             println!("Issue action: {}", action);
    ///         }
    ///         
    ///         if let Some(issue) = payload.get("issue") {
    ///             if let Some(title) = issue.get("title") {
    ///                 println!("Issue title: {}", title);
    ///             }
    ///             if let Some(number) = issue.get("number") {
    ///                 println!("Issue number: {}", number);
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
    /// ## Auto-labeling New Issues
    ///
    /// ```rust,no_run
    /// use octofer::{Octofer, Config, Context};
    /// use std::sync::Arc;
    ///
    /// # async fn example() -> anyhow::Result<()> {
    /// let config = Config::from_env().unwrap_or_default();
    /// let mut app = Octofer::new(config).await.unwrap_or_else(|_| Octofer::new_default());
    ///
    /// app.on_issue(
    ///     |context: Context, _extra: Arc<()>| async move {
    ///         let payload = context.payload();
    ///         
    ///         // Only process newly opened issues
    ///         if payload.get("action").and_then(|v| v.as_str()) != Some("opened") {
    ///             return Ok(());
    ///         }
    ///         
    ///         println!("New issue opened!");
    ///         
    ///         // Auto-label based on issue title or body
    ///         if let Some(issue) = payload.get("issue") {
    ///             if let Some(title) = issue.get("title").and_then(|t| t.as_str()) {
    ///                 if title.to_lowercase().contains("bug") {
    ///                     println!("Would add 'bug' label to issue");
    ///                     // Use installation client to add label
    ///                 } else if title.to_lowercase().contains("feature") {
    ///                     println!("Would add 'enhancement' label to issue");
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
    pub async fn on_issue<F, Fut, E>(&mut self, handler: F, extra: Arc<E>) -> &Self
    where
        F: Fn(Context, Arc<E>) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = anyhow::Result<()>> + Send + 'static,
        E: Send + Sync + 'static,
    {
        self.server
            .on(WebhookEventType::Issues.to_string(), handler, extra)
            .await;
        self
    }
}
