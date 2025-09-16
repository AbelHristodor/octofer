//! GitHub API client with installation token management

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
#[derive(Debug)]
struct CachedInstallationClient {
    client: Octocrab,
    token: InstallationToken,
    created_at: chrono::DateTime<chrono::Utc>,
}

impl CachedInstallationClient {
    /// Check if the token is expired (with 5-minute buffer)
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
#[derive(Debug)]
pub struct GitHubClient {
    /// Main app client for app-level operations
    app_client: Octocrab,
    /// Cached installation clients
    installation_clients: Arc<RwLock<HashMap<u64, CachedInstallationClient>>>,
}

impl GitHubClient {
    /// Create a new GitHub client with the provided authentication
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
    pub fn app_client(&self) -> &Octocrab {
        &self.app_client
    }

    /// Get all installations for this GitHub App
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
    pub async fn with_installation<F, R>(&self, installation_id: u64, f: F) -> Result<R>
    where
        F: FnOnce(Octocrab) -> Result<R>,
    {
        let client = self.installation_client(installation_id).await?;
        f(client)
    }

    /// Execute an async closure with an installation client
    pub async fn with_installation_async<F, Fut, R>(&self, installation_id: u64, f: F) -> Result<R>
    where
        F: FnOnce(Octocrab) -> Fut,
        Fut: std::future::Future<Output = Result<R>>,
    {
        let client = self.installation_client(installation_id).await?;
        f(client).await
    }

    /// Clear cached installation client (useful for testing or forcing refresh)
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
