//! Core types and traits for Octofer
//!
//! This module contains the fundamental types and traits used throughout the framework,
//! including the Context type and event handler definitions.
//!
//! # Context
//!
//! The [`Context`] struct is the primary way event handlers receive information about
//! incoming GitHub webhook events. It contains:
//!
//! - The webhook event data from GitHub
//! - Installation ID for the GitHub App installation that triggered the event
//! - An authenticated GitHub API client for making API calls
//!
//! # Event Handlers
//!
//! Event handlers are functions that process GitHub webhook events. They receive a
//! [`Context`] containing event information and can optionally receive additional
//! data via the `extra` parameter.
//!
//! # Examples
//!
//! ```rust,no_run
//! use octofer::{Context, Octofer, Config};
//! use std::sync::Arc;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let config = Config::default();
//!     let mut app = Octofer::new(config).await.unwrap_or_else(|_| Octofer::new_default());
//!
//!     // Register an event handler
//!     app.on_issue_comment(
//!         |context: Context, _extra: Arc<()>| async move {
//!             println!("Event type: {}", context.kind());
//!             
//!             if let Some(client) = context.github() {
//!                 println!("GitHub client is available for API calls");
//!             }
//!             
//!             Ok(())
//!         },
//!         Arc::new(()),
//!     ).await;
//!
//!     app.start().await?;
//!     Ok(())
//! }
//! ```

use octocrab::models::webhook_events::WebhookEvent;

use crate::{github::GitHubClient, webhook::WebhookEventKind};
use crate::{SerdeToString, UNDEFINED_EVENT_KIND};
use std::sync::Arc;

/// Context passed to event handlers containing event information and utilities
///
/// The Context struct provides event handlers with access to webhook event data,
/// GitHub API clients, and installation information. This is the primary way
/// handlers interact with incoming GitHub events and perform API operations.
///
/// # Fields
///
/// - `event` - The complete webhook event from GitHub (if available)
/// - `installation_id` - The GitHub App installation ID (if available)
/// - `github_client` - An authenticated GitHub API client (if available)
///
/// # Examples
///
/// ## Accessing event information
/// ```rust,no_run
/// use octofer::Context;
///
/// async fn my_handler(context: Context) -> anyhow::Result<()> {
///     // Get the event type
///     println!("Event type: {}", context.kind());
///     
///     // Get the installation ID
///     if let Some(installation_id) = context.installation_id() {
///         println!("Installation ID: {}", installation_id);
///     }
///     
///     // Access raw event payload
///     let payload = context.payload();
///     println!("Payload: {}", payload);
///     
///     Ok(())
/// }
/// ```
///
/// ## Using the GitHub API client
/// ```rust,no_run
/// use octofer::Context;
///
/// async fn api_handler(context: Context) -> anyhow::Result<()> {
///     if let Some(client) = context.github() {
///         // Use the client for app-level operations
///         let installations = client.get_installations().await?;
///         println!("Found {} installations", installations.len());
///         
///         // Or get an installation-specific client
///         if let Some(installation_client) = context.installation_client().await? {
///             // Use installation_client for repository operations
///         }
///     }
///     
///     Ok(())
/// }
/// ```
#[derive(Clone, Debug, Default)]
pub struct Context {
    /// Event payload data from GitHub webhook
    pub event: Option<WebhookEvent>,
    /// Installation ID for GitHub App authentication
    pub installation_id: Option<u64>,
    /// GitHub client for API operations (if available)
    pub github_client: Option<Arc<GitHubClient>>,
}

impl Context {
    /// Create a new context
    ///
    /// Creates a new Context with the given event and installation ID.
    /// The GitHub client will be None and must be set separately if needed.
    ///
    /// # Arguments
    ///
    /// * `event` - Optional webhook event from GitHub
    /// * `installation_id` - Optional GitHub App installation ID
    ///
    /// # Examples
    ///
    /// ```rust
    /// use octofer::Context;
    ///
    /// // Create context without event data
    /// let context = Context::new(None, None);
    ///
    /// // Create context with installation ID
    /// let context = Context::new(None, Some(12345));
    /// ```
    pub fn new(event: Option<WebhookEvent>, installation_id: Option<u64>) -> Self {
        Self {
            event,
            installation_id,
            github_client: None,
        }
    }

    /// Create a new context with GitHub client
    ///
    /// Creates a new Context with all parameters including a GitHub client.
    /// This is typically used internally by the framework when processing
    /// webhook events.
    ///
    /// # Arguments
    ///
    /// * `event` - Optional webhook event from GitHub
    /// * `installation_id` - Optional GitHub App installation ID
    /// * `github_client` - Optional GitHub API client
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use octofer::{Context, github::GitHubClient};
    /// use std::sync::Arc;
    ///
    /// // This is typically done by the framework, not user code
    /// let context = Context::with_github_client(
    ///     None,
    ///     Some(12345),
    ///     None, // Would normally contain an actual client
    /// );
    /// ```
    pub fn with_github_client(
        event: Option<WebhookEvent>,
        installation_id: Option<u64>,
        github_client: Option<Arc<GitHubClient>>,
    ) -> Self {
        Self {
            event,
            installation_id,
            github_client,
        }
    }

    /// Get the event type as a string
    ///
    /// Returns the type of webhook event (e.g., "issues", "pull_request", "issue_comment").
    /// If no event is present, returns "undefined".
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use octofer::Context;
    ///
    /// async fn handler(context: Context) -> anyhow::Result<()> {
    ///     match context.kind().as_str() {
    ///         "issues" => println!("This is an issue event"),
    ///         "pull_request" => println!("This is a pull request event"),
    ///         "issue_comment" => println!("This is an issue comment event"),
    ///         _ => println!("Unknown or undefined event type"),
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub fn kind(&self) -> WebhookEventKind {
        match &self.event {
            Some(e) => e.kind.to_string(),
            None => UNDEFINED_EVENT_KIND.to_string(),
        }
    }

    /// Get the event payload
    ///
    /// Returns a reference to the complete webhook event if available.
    /// This provides access to the full event structure for detailed processing.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use octofer::Context;
    ///
    /// async fn handler(context: Context) -> anyhow::Result<()> {
    ///     if let Some(event) = context.event() {
    ///         println!("Event kind: {:?}", event.kind);
    ///         // Access other event fields as needed
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub fn event(&self) -> &Option<WebhookEvent> {
        &self.event
    }

    /// Get the event payload as a JSON value
    ///
    /// This provides access to the raw webhook payload data as a serde_json::Value,
    /// allowing handlers to extract specific fields they need. This is useful when
    /// you need to access event-specific data that isn't available in the typed
    /// WebhookEvent structure.
    ///
    /// # Returns
    ///
    /// Returns the event data as a JSON Value, or `serde_json::Value::Null` if
    /// no event is present or serialization fails.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use octofer::Context;
    ///
    /// async fn handler(context: Context) -> anyhow::Result<()> {
    ///     let payload = context.payload();
    ///     
    ///     // Extract specific fields from the payload
    ///     if let Some(action) = payload.get("action") {
    ///         println!("Action: {}", action);
    ///     }
    ///     
    ///     if let Some(issue) = payload.get("issue") {
    ///         if let Some(title) = issue.get("title") {
    ///             println!("Issue title: {}", title);
    ///         }
    ///     }
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub fn payload(&self) -> serde_json::Value {
        match &self.event {
            Some(event) => {
                // Convert the WebhookEvent to a JSON value
                serde_json::to_value(event).unwrap_or(serde_json::Value::Null)
            }
            None => serde_json::Value::Null,
        }
    }

    /// Get the installation ID
    ///
    /// Returns the GitHub App installation ID associated with this event.
    /// This ID identifies which installation of your GitHub App triggered
    /// the webhook event.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use octofer::Context;
    ///
    /// async fn handler(context: Context) -> anyhow::Result<()> {
    ///     match context.installation_id() {
    ///         Some(id) => println!("Installation ID: {}", id),
    ///         None => println!("No installation ID available"),
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub fn installation_id(&self) -> Option<u64> {
        self.installation_id
    }

    /// Get access to the GitHub client
    ///
    /// Returns a reference to the GitHub client if available. The client is already
    /// authenticated and can be used to make API calls. If an installation ID is
    /// available in the context, the client will automatically use the appropriate
    /// installation token for API calls.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use octofer::Context;
    ///
    /// async fn handler(context: Context) -> anyhow::Result<()> {
    ///     if let Some(client) = context.github() {
    ///         // Get all installations for this GitHub App
    ///         let installations = client.get_installations().await?;
    ///         println!("Found {} installations", installations.len());
    ///         
    ///         // Access repositories for a specific installation
    ///         if let Some(installation_id) = context.installation_id() {
    ///             let repos = client.get_installation_repositories(installation_id).await?;
    ///             println!("Installation has {} repositories", repos.len());
    ///         }
    ///     } else {
    ///         println!("No GitHub client available");
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub fn github(&self) -> Option<&Arc<GitHubClient>> {
        self.github_client.as_ref()
    }

    /// Get an authenticated installation client for the current installation
    ///
    /// This is a convenience method that returns an Octocrab client authenticated
    /// as the specific installation from this event context. This client can be
    /// used for repository-specific operations that require installation-level
    /// permissions.
    ///
    /// # Returns
    ///
    /// Returns `Ok(Some(client))` if both a GitHub client and installation ID are
    /// available, `Ok(None)` if either is missing, or `Err` if there's an error
    /// creating the installation client.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use octofer::Context;
    ///
    /// async fn handler(context: Context) -> anyhow::Result<()> {
    ///     if let Some(client) = context.installation_client().await? {
    ///         // Use the installation client for repository operations
    ///         let user = client.current().user().await?;
    ///         println!("Acting as: {}", user.login);
    ///         
    ///         // Create issues, comments, etc. with installation permissions
    ///         // let issue = client.issues("owner", "repo").create("Title").send().await?;
    ///     } else {
    ///         println!("Cannot create installation client");
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn installation_client(&self) -> anyhow::Result<Option<octocrab::Octocrab>> {
        match (&self.github_client, self.installation_id) {
            (Some(client), Some(installation_id)) => {
                let octocrab_client = client.installation_client(installation_id).await?;
                Ok(Some(octocrab_client))
            }
            _ => Ok(None),
        }
    }
}

/// Type alias for event handler functions
///
/// This type represents a boxed async function that takes a Context and returns
/// a Result. Event handler functions implement this signature to process GitHub
/// webhook events.
///
/// # Examples
///
/// ```rust,no_run
/// use octofer::{Context, core::EventHandlerFn};
///
/// // This function matches the EventHandlerFn signature
/// async fn my_handler(context: Context) -> anyhow::Result<()> {
///     println!("Processing event: {}", context.kind());
///     Ok(())
/// }
///
/// // Convert to EventHandlerFn
/// let handler: EventHandlerFn = Box::new(|context| {
///     Box::pin(async move {
///         println!("Processing event: {}", context.kind());
///         Ok(())
///     })
/// });
/// ```
pub type EventHandlerFn = Box<
    dyn Fn(
            Context,
        )
            -> std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<()>> + Send>>
        + Send
        + Sync,
>;

/// Trait for types that can handle GitHub events
///
/// This trait allows types to implement event handling logic. It's used internally
/// by the framework but can also be implemented by custom types that need to
/// handle events with additional context or state.
///
/// # Type Parameters
///
/// * `T` - The type of extra data passed to the handler
///
/// # Examples
///
/// ```rust,no_run
/// use octofer::{Context, core::EventHandler};
/// use std::sync::Arc;
///
/// struct MyHandler;
///
/// impl EventHandler<()> for MyHandler {
///     fn handle(
///         &self,
///         context: Context,
///         _extra: Arc<()>,
///     ) -> std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<()>> + Send>> {
///         Box::pin(async move {
///             println!("Handling event: {}", context.kind());
///             Ok(())
///         })
///     }
/// }
/// ```
pub trait EventHandler<T>: Send + Sync
where
    T: Send + Sync + 'static,
{
    /// Handle an event with the provided context
    ///
    /// This method is called when a GitHub webhook event needs to be processed.
    /// Implementations should perform their event handling logic and return
    /// `Ok(())` on success or an error if something goes wrong.
    ///
    /// # Arguments
    ///
    /// * `context` - The event context containing webhook data and API client
    /// * `extra` - Additional data passed to the handler
    ///
    /// # Returns
    ///
    /// Returns a pinned future that resolves to `anyhow::Result<()>`.
    fn handle(
        &self,
        context: Context,
        extra: Arc<T>,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<()>> + Send>>;
}
