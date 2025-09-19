//! GitHub API client with installation token management
//!
//! This module provides a high-level GitHub API client that handles authentication,
//! installation token management, and automatic token refresh for GitHub Apps.
//!
//! # Key Features
//!
//! - **Automatic Token Management**: Installation tokens are automatically created,
//!   cached, and refreshed when needed
//! - **App-level Operations**: Access to GitHub App installations and app-level APIs
//! - **Installation-level Operations**: Per-installation authenticated clients for
//!   repository operations
//! - **Retry Logic**: Built-in retry logic for handling rate limits and transient errors
//! - **Thread Safety**: All operations are thread-safe with internal locking
//!
//! # Examples
//!
//! ## Basic Usage
//!
//! ```rust,no_run
//! use octofer::{Config, github::{GitHubAuth, GitHubClient}};
//!
//! # async fn example() -> anyhow::Result<()> {
//! // Create authentication from configuration
//! let config = Config::from_env()?;
//! let auth = GitHubAuth::from_config(&config.github);
//!
//! // Create a GitHub client
//! let client = GitHubClient::new(auth).await?;
//!
//! // Get all installations
//! let installations = client.get_installations().await?;
//! println!("Found {} installations", installations.len());
//!
//! // Get repositories for a specific installation
//! if let Some(installation) = installations.first() {
//!     let repos = client.get_installation_repositories(installation.id.0).await?;
//!     println!("Installation has {} repositories", repos.len());
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Installation Client Usage
//!
//! ```rust,no_run
//! use octofer::{Config, github::{GitHubAuth, GitHubClient}};
//!
//! # async fn example() -> anyhow::Result<()> {
//! let config = Config::from_env()?;
//! let auth = GitHubAuth::from_config(&config.github);
//! let client = GitHubClient::new(auth).await?;
//!
//! // Get an installation-specific client
//! let installation_id = 12345;
//! let installation_client = client.installation_client(installation_id).await?;
//!
//! // Use the installation client for repository operations
//! let user = installation_client.current().user().await?;
//! println!("Acting as: {}", user.login);
//! # Ok(())
//! # }
//! ```

use crate::github::auth::{parse_to_utc, GitHubAuth};
use anyhow::{anyhow, Result};
use chrono::Utc;
use octocrab::{
    models::{InstallationRepositories, InstallationToken},
    params::apps::CreateInstallationAccessToken,
    Octocrab,
};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use tracing::{debug, info};
use url::Url;

/// Cached installation client with token expiration tracking
///
/// This internal struct manages cached Octocrab clients for specific GitHub App
/// installations, including tracking token expiration times to ensure tokens
/// are refreshed before they expire.
#[derive(Debug)]
struct CachedInstallationClient {
    /// The authenticated Octocrab client for this installation
    client: Octocrab,
    /// The installation token details
    token: InstallationToken,
    /// When this client was created (for expiration calculation)
    created_at: chrono::DateTime<chrono::Utc>,
}

impl CachedInstallationClient {
    /// Check if the token is expired (with 5-minute buffer)
    ///
    /// Returns true if the token will expire within 5 minutes. This buffer
    /// ensures that tokens are refreshed before they actually expire, preventing
    /// authentication failures.
    fn is_expired(&self) -> bool {
        let default_expires_at = self.created_at + chrono::Duration::hours(1);
        let buffer = chrono::Duration::minutes(5);
        let expires_at = self
            .token
            .expires_at
            .clone()
            .unwrap_or(default_expires_at.to_string());

        debug!("Token expires at: {:?}", expires_at);
        Utc::now() + buffer >= parse_to_utc(&expires_at)
    }
}

/// GitHub API client with automatic authentication and token management
///
/// This is the main GitHub client for Octofer applications. It provides both
/// app-level operations (like getting installations) and installation-level
/// operations (like repository access) with automatic token management.
///
/// # Token Management
///
/// The client automatically handles:
/// - Creating installation tokens when needed
/// - Caching tokens to avoid unnecessary API calls
/// - Refreshing tokens before they expire (with a 5-minute buffer)
/// - Thread-safe access to cached tokens
///
/// # Client Types
///
/// - **App Client**: Used for app-level operations like listing installations
/// - **Installation Clients**: Used for repository-level operations, automatically
///   authenticated with the appropriate installation token
///
/// # Examples
///
/// ## Creating a Client
///
/// ```rust,no_run
/// use octofer::{Config, github::{GitHubAuth, GitHubClient}};
///
/// # async fn example() -> anyhow::Result<()> {
/// let config = Config::from_env()?;
/// let auth = GitHubAuth::from_config(&config.github);
/// let client = GitHubClient::new(auth).await?;
/// # Ok(())
/// # }
/// ```
///
/// ## App-Level Operations
///
/// ```rust,no_run
/// # use octofer::{Config, github::{GitHubAuth, GitHubClient}};
/// # async fn example(client: GitHubClient) -> anyhow::Result<()> {
/// // List all installations
/// let installations = client.get_installations().await?;
/// for installation in installations {
///     println!("Installation: {} ({})", installation.account.login, installation.id.0);
/// }
/// # Ok(())
/// # }
/// ```
///
/// ## Installation-Level Operations
///
/// ```rust,no_run
/// # use octofer::{Config, github::{GitHubAuth, GitHubClient}};
/// # async fn example(client: GitHubClient) -> anyhow::Result<()> {
/// let installation_id = 12345;
///
/// // Get repositories for an installation
/// let repos = client.get_installation_repositories(installation_id).await?;
/// println!("Found {} repositories", repos.len());
///
/// // Get an installation client for direct API access
/// let installation_client = client.installation_client(installation_id).await?;
/// let user = installation_client.current().user().await?;
/// println!("Acting as: {}", user.login);
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct GitHubClient {
    /// Main app client for app-level operations
    app_client: Octocrab,
    /// Cached installation clients with automatic token refresh
    installation_clients: Arc<RwLock<HashMap<u64, CachedInstallationClient>>>,
}

impl GitHubClient {
    /// Create a new GitHub client with the provided authentication
    ///
    /// Creates a new GitHubClient configured for the provided GitHub App.
    /// The client will be authenticated as the app and ready to perform
    /// app-level operations and create installation clients.
    ///
    /// # Arguments
    ///
    /// * `auth` - GitHub App authentication containing app ID and private key
    ///
    /// # Returns
    ///
    /// Returns `Ok(GitHubClient)` if the client was created successfully,
    /// or `Err` if there was an error with authentication or client setup.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The private key cannot be parsed as valid RSA PEM
    /// - The Octocrab client cannot be built with the provided credentials
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use octofer::{Config, github::{GitHubAuth, GitHubClient}};
    ///
    /// # async fn example() -> anyhow::Result<()> {
    /// let config = Config::from_env()?;
    /// let auth = GitHubAuth::from_config(&config.github);
    /// let client = GitHubClient::new(auth).await?;
    /// println!("GitHub client created successfully");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn new(auth: GitHubAuth) -> Result<Self> {
        let app_client = octocrab::OctocrabBuilder::new()
            .add_retry_config(octocrab::service::middleware::retry::RetryConfig::Simple(
                20,
            ))
            .app(
                auth.app_id().into(),
                jsonwebtoken::EncodingKey::from_rsa_pem(auth.private_key())
                    .map_err(|e| anyhow!("Failed to create encoding key from PEM: {}", e))?,
            )
            .build()
            .map_err(|e| anyhow!("Failed to build GitHub client: {}", e))?;

        Ok(Self {
            app_client,
            installation_clients: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Get the app client for app-level operations
    ///
    /// Returns a reference to the underlying Octocrab client authenticated
    /// as the GitHub App. This client can be used for app-level operations
    /// like listing installations or accessing app-level APIs.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use octofer::github::GitHubClient;
    /// # async fn example(client: GitHubClient) -> anyhow::Result<()> {
    /// let app_client = client.app_client();
    ///
    /// // Use the app client directly for custom operations
    /// // Note: Specific API methods depend on octocrab version
    /// println!("App client is available for custom operations");
    /// # Ok(())
    /// # }
    /// ```
    pub fn app_client(&self) -> &Octocrab {
        &self.app_client
    }

    /// Get all installations for this GitHub App
    ///
    /// Retrieves a list of all installations of this GitHub App across
    /// all organizations and user accounts where it's installed.
    ///
    /// # Returns
    ///
    /// Returns `Ok(Vec<Installation>)` containing all installations,
    /// or `Err` if the API request fails.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The GitHub API request fails
    /// - Authentication is invalid
    /// - Network connectivity issues
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use octofer::github::GitHubClient;
    /// # async fn example(client: GitHubClient) -> anyhow::Result<()> {
    /// let installations = client.get_installations().await?;
    ///
    /// for installation in installations {
    ///     println!("Installation: {} (ID: {})",
    ///         installation.account.login,
    ///         installation.id.0
    ///     );
    ///     println!("  Target type: {:?}", installation.target_type);
    ///     if let Some(app_id) = installation.app_id {
    ///         println!("  App ID: {}", app_id.0);
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_installations(&self) -> Result<Vec<octocrab::models::Installation>> {
        let installations = self
            .app_client
            .apps()
            .installations()
            .send()
            .await
            .map_err(|e| anyhow!("Failed to fetch installations: {}", e))?
            .take_items();

        info!("Fetched {} installations", installations.len());
        Ok(installations)
    }

    /// Get a client authenticated as a specific installation
    ///
    /// Returns an Octocrab client authenticated with an installation token
    /// for the specified installation. The client is cached and tokens are
    /// automatically refreshed when needed.
    ///
    /// # Arguments
    ///
    /// * `installation_id` - The ID of the installation to authenticate as
    ///
    /// # Returns
    ///
    /// Returns `Ok(Octocrab)` with an authenticated installation client,
    /// or `Err` if the installation doesn't exist or token creation fails.
    ///
    /// # Token Caching
    ///
    /// This method automatically caches installation clients and reuses them
    /// until their tokens are close to expiring (within 5 minutes). When a
    /// token is about to expire, a new one is automatically created.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use octofer::github::GitHubClient;
    /// # async fn example(client: GitHubClient) -> anyhow::Result<()> {
    /// let installation_id = 12345;
    /// let installation_client = client.installation_client(installation_id).await?;
    ///
    /// // Use the installation client for repository operations
    /// let user = installation_client.current().user().await?;
    /// println!("Acting as: {}", user.login);
    ///
    /// // Create an issue (if the installation has the necessary permissions)
    /// // let issue = installation_client
    /// //     .issues("owner", "repo")
    /// //     .create("Issue Title")
    /// //     .body("Issue body")
    /// //     .send()
    /// //     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn installation_client(&self, installation_id: u64) -> Result<Octocrab> {
        // Check if we have a cached client that's still valid
        {
            let clients = self.installation_clients.read().await;
            if let Some(cached) = clients.get(&installation_id) {
                if !cached.is_expired() {
                    debug!("Using cached installation client for {}", installation_id);
                    return Ok(cached.client.clone());
                }
                debug!("Cached client for {} is expired", installation_id);
            }
        }

        // Create a new installation client
        self.create_installation_client(installation_id).await
    }

    /// Create a new installation client and cache it
    ///
    /// This is an internal method that creates a new installation client,
    /// generates a token, and caches the client for future use.
    async fn create_installation_client(&self, installation_id: u64) -> Result<Octocrab> {
        info!(
            "Creating new installation client for ID: {}",
            installation_id
        );

        let token = self
            .create_installation_token(installation_id, None)
            .await?;

        let client = Octocrab::builder()
            .add_retry_config(octocrab::service::middleware::retry::RetryConfig::Simple(
                20,
            ))
            .personal_token(token.token.clone())
            .build()
            .map_err(|e| anyhow!("Failed to create installation client: {}", e))?;

        // Cache the client
        let cached_client = CachedInstallationClient {
            client: client.clone(),
            token,
            created_at: Utc::now(),
        };

        {
            let mut clients = self.installation_clients.write().await;
            clients.insert(installation_id, cached_client);
        }

        Ok(client)
    }

    /// Create a new installation access token
    ///
    /// This is an internal method that creates a new installation access token
    /// for the specified installation, optionally scoped to specific repositories.
    async fn create_installation_token(
        &self,
        installation_id: u64,
        repositories: Option<Vec<String>>,
    ) -> Result<InstallationToken> {
        let installations = self.get_installations().await?;

        let installation = installations
            .iter()
            .find(|i| i.id.0 == installation_id)
            .ok_or_else(|| anyhow!("Installation with ID {} not found", installation_id))?;

        let access_tokens_url = installation
            .access_tokens_url
            .as_ref()
            .ok_or_else(|| anyhow!("No access tokens URL for installation {}", installation_id))?;

        let mut create_token_request = CreateInstallationAccessToken::default();
        if let Some(repos) = repositories {
            create_token_request.repositories = repos;
        }

        let url = Url::parse(access_tokens_url)
            .map_err(|e| anyhow!("Invalid access tokens URL: {}", e))?;

        let token: InstallationToken = self
            .app_client
            .post(url.path(), Some(&create_token_request))
            .await
            .map_err(|e| anyhow!("Failed to create installation token: {}", e))?;

        info!(
            "Created installation token for installation {}",
            installation_id
        );
        Ok(token)
    }

    /// Get repositories accessible by an installation
    ///
    /// Retrieves a list of all repositories that the specified installation
    /// has access to. This includes repositories that the GitHub App was
    /// explicitly granted access to during installation.
    ///
    /// # Arguments
    ///
    /// * `installation_id` - The ID of the installation to query
    ///
    /// # Returns
    ///
    /// Returns `Ok(Vec<Repository>)` containing all accessible repositories,
    /// or `Err` if the installation doesn't exist or the API request fails.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use octofer::github::GitHubClient;
    /// # async fn example(client: GitHubClient) -> anyhow::Result<()> {
    /// let installation_id = 12345;
    /// let repositories = client.get_installation_repositories(installation_id).await?;
    ///
    /// for repo in repositories {
    ///     println!("Repository: {}", repo.full_name.unwrap_or_default());
    ///     println!("  Private: {}", repo.private.unwrap_or(false));
    ///     println!("  Language: {}", repo.language.unwrap_or_default());
    ///     
    ///     if let Some(description) = repo.description {
    ///         println!("  Description: {}", description);
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_installation_repositories(
        &self,
        installation_id: u64,
    ) -> Result<Vec<octocrab::models::Repository>> {
        let client = self.installation_client(installation_id).await?;

        let installation_repos: InstallationRepositories = client
            .get("/installation/repositories", None::<&()>)
            .await
            .map_err(|e| anyhow!("Failed to get installation repositories: {}", e))?;

        info!(
            "Installation {} has access to {} repositories",
            installation_id,
            installation_repos.repositories.len()
        );

        Ok(installation_repos.repositories)
    }

    /// Execute a closure with an installation client
    ///
    /// This is a utility method that provides access to an installation client
    /// for the duration of a synchronous closure. The client is automatically
    /// managed and cleaned up.
    ///
    /// # Arguments
    ///
    /// * `installation_id` - The ID of the installation to get a client for
    /// * `f` - A closure that receives the installation client and returns a Result
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use octofer::github::GitHubClient;
    /// # async fn example(client: GitHubClient) -> anyhow::Result<()> {
    /// let installation_id = 12345;
    ///
    /// let user_login = client.with_installation(installation_id, |installation_client| {
    ///     // Note: This is a simplified example - the actual API is async
    ///     // In practice, you'd use with_installation_async for async operations
    ///     Ok("example_user".to_string())
    /// }).await?;
    ///
    /// println!("User: {}", user_login);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn with_installation<F, R>(&self, installation_id: u64, f: F) -> Result<R>
    where
        F: FnOnce(Octocrab) -> Result<R>,
    {
        let client = self.installation_client(installation_id).await?;
        f(client)
    }

    /// Execute an async closure with an installation client
    ///
    /// This is a utility method that provides access to an installation client
    /// for the duration of an async closure. This is the preferred method for
    /// most operations since GitHub API calls are async.
    ///
    /// # Arguments
    ///
    /// * `installation_id` - The ID of the installation to get a client for
    /// * `f` - An async closure that receives the installation client and returns a `Future<Result>`
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use octofer::github::GitHubClient;
    /// # async fn example(client: GitHubClient) -> anyhow::Result<()> {
    /// let installation_id = 12345;
    ///
    /// let user_info = client.with_installation_async(installation_id, |installation_client| async move {
    ///     let user = installation_client.current().user().await?;
    ///     Ok(format!("{} ({})", user.login, user.name.unwrap_or_default()))
    /// }).await?;
    ///
    /// println!("User info: {}", user_info);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn with_installation_async<F, Fut, R>(&self, installation_id: u64, f: F) -> Result<R>
    where
        F: FnOnce(Octocrab) -> Fut,
        Fut: std::future::Future<Output = Result<R>>,
    {
        let client = self.installation_client(installation_id).await?;
        f(client).await
    }

    /// Clear cached installation client (useful for testing or forcing refresh)
    ///
    /// Removes cached installation clients to force the creation of new ones
    /// on the next request. This can be useful for testing or when you need
    /// to ensure fresh tokens are used.
    ///
    /// # Arguments
    ///
    /// * `installation_id` - Optional installation ID to clear. If None, clears all cached clients.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use octofer::github::GitHubClient;
    /// # async fn example(client: GitHubClient) -> anyhow::Result<()> {
    /// // Clear cache for a specific installation
    /// client.clear_installation_cache(Some(12345)).await;
    ///
    /// // Clear all cached clients
    /// client.clear_installation_cache(None).await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn clear_installation_cache(&self, installation_id: Option<u64>) {
        let mut clients = self.installation_clients.write().await;

        if let Some(id) = installation_id {
            clients.remove(&id);
            info!("Cleared cache for installation {}", id);
        } else {
            clients.clear();
            info!("Cleared all installation caches");
        }
    }
}
