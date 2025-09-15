//! # Octofer GitHub
//! 
//! GitHub API client and authentication for Octofer framework.

use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};

/// GitHub API client
pub struct GitHubClient {
    client: Client,
    base_url: String,
    token: Option<String>,
}

impl GitHubClient {
    /// Create a new GitHub client
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: "https://api.github.com".to_string(),
            token: None,
        }
    }

    /// Set authentication token
    pub fn with_token(mut self, token: impl Into<String>) -> Self {
        self.token = Some(token.into());
        self
    }

    /// Get repository information
    pub async fn get_repo(&self, owner: &str, repo: &str) -> Result<Repository> {
        let url = format!("{}/repos/{}/{}", self.base_url, owner, repo);
        let response = self.client.get(&url).send().await?;
        let repo: Repository = response.json().await?;
        Ok(repo)
    }
}

impl Default for GitHubClient {
    fn default() -> Self {
        Self::new()
    }
}

/// GitHub repository
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repository {
    pub id: u64,
    pub name: String,
    pub full_name: String,
    pub owner: User,
    pub description: Option<String>,
    pub private: bool,
    pub html_url: String,
    pub clone_url: String,
    pub default_branch: String,
}

/// GitHub user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub login: String,
    pub avatar_url: String,
    pub html_url: String,
}

/// GitHub App installation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Installation {
    pub id: u64,
    pub account: User,
    pub repository_selection: String,
    pub access_tokens_url: String,
}