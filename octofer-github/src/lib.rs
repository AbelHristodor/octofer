//! # Octofer GitHub
//! 
//! GitHub API client and authentication for Octofer framework using octocrab.

use anyhow::Result;
use octocrab::{Octocrab, OctocrabBuilder};
use serde::{Deserialize, Serialize};

/// GitHub API client wrapper around octocrab
pub struct GitHubClient {
    octocrab: Octocrab,
}

impl GitHubClient {
    /// Create a new GitHub client with default configuration
    pub fn new() -> Self {
        Self {
            octocrab: Octocrab::default(),
        }
    }

    /// Create a new GitHub client with authentication token
    pub fn with_token(token: impl Into<String>) -> Result<Self> {
        let octocrab = OctocrabBuilder::new()
            .personal_token(token.into())
            .build()?;
        
        Ok(Self { octocrab })
    }

    /// Create a new GitHub client for GitHub App authentication
    pub fn with_app_credentials(
        app_id: u64,
        private_key: impl Into<String>,
    ) -> Result<Self> {
        let key = jsonwebtoken::EncodingKey::from_rsa_pem(private_key.into().as_bytes())?;
        let octocrab = OctocrabBuilder::new()
            .app(app_id.into(), key)
            .build()?;
        
        Ok(Self { octocrab })
    }

    /// Get repository information
    pub async fn get_repo(&self, owner: &str, repo: &str) -> Result<Repository> {
        let repo = self.octocrab.repos(owner, repo).get().await?;
        Ok(Repository::from_octocrab_repo(repo))
    }

    /// Get repository issues
    pub async fn get_issues(&self, owner: &str, repo: &str) -> Result<Vec<Issue>> {
        let issues = self.octocrab.issues(owner, repo).list().send().await?;
        Ok(issues.items.into_iter().map(Issue::from_octocrab_issue).collect())
    }

    /// Get a specific issue
    pub async fn get_issue(&self, owner: &str, repo: &str, issue_number: u64) -> Result<Issue> {
        let issue = self.octocrab.issues(owner, repo).get(issue_number).await?;
        Ok(Issue::from_octocrab_issue(issue))
    }

    /// Create a comment on an issue
    pub async fn create_issue_comment(
        &self,
        owner: &str,
        repo: &str,
        issue_number: u64,
        body: &str,
    ) -> Result<IssueComment> {
        let comment = self
            .octocrab
            .issues(owner, repo)
            .create_comment(issue_number, body)
            .await?;
        Ok(IssueComment::from_octocrab_comment(comment))
    }

    /// Get pull requests
    pub async fn get_pull_requests(&self, owner: &str, repo: &str) -> Result<Vec<PullRequest>> {
        let pulls = self.octocrab.pulls(owner, repo).list().send().await?;
        Ok(pulls.items.into_iter().map(PullRequest::from_octocrab_pull).collect())
    }

    /// Get a specific pull request
    pub async fn get_pull_request(
        &self,
        owner: &str,
        repo: &str,
        pull_number: u64,
    ) -> Result<PullRequest> {
        let pull = self.octocrab.pulls(owner, repo).get(pull_number).await?;
        Ok(PullRequest::from_octocrab_pull(pull))
    }

    /// Get the underlying octocrab client for advanced usage
    pub fn octocrab(&self) -> &Octocrab {
        &self.octocrab
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

impl Repository {
    fn from_octocrab_repo(repo: octocrab::models::Repository) -> Self {
        Self {
            id: repo.id.0,
            name: repo.name,
            full_name: repo.full_name.unwrap_or_default(),
            owner: User::from_octocrab_user(repo.owner.unwrap()),
            description: repo.description,
            private: repo.private.unwrap_or(false),
            html_url: repo.html_url.unwrap().to_string(),
            clone_url: repo.clone_url.unwrap().to_string(),
            default_branch: repo.default_branch.unwrap_or_else(|| "main".to_string()),
        }
    }
}

/// GitHub user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub login: String,
    pub avatar_url: String,
    pub html_url: String,
}

impl User {
    fn from_octocrab_user(user: octocrab::models::Author) -> Self {
        Self {
            id: user.id.0,
            login: user.login,
            avatar_url: user.avatar_url.to_string(),
            html_url: user.html_url.to_string(),
        }
    }
}

/// GitHub issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Issue {
    pub id: u64,
    pub number: u64,
    pub title: String,
    pub body: Option<String>,
    pub user: User,
    pub state: String,
    pub html_url: String,
}

impl Issue {
    fn from_octocrab_issue(issue: octocrab::models::issues::Issue) -> Self {
        Self {
            id: issue.id.0,
            number: issue.number,
            title: issue.title,
            body: issue.body,
            user: User::from_octocrab_user(issue.user),
            state: format!("{:?}", issue.state).to_lowercase(),
            html_url: issue.html_url.to_string(),
        }
    }
}

/// GitHub issue comment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueComment {
    pub id: u64,
    pub body: String,
    pub user: User,
    pub html_url: String,
}

impl IssueComment {
    fn from_octocrab_comment(comment: octocrab::models::issues::Comment) -> Self {
        Self {
            id: comment.id.0,
            body: comment.body.unwrap_or_default(),
            user: User::from_octocrab_user(comment.user),
            html_url: comment.html_url.to_string(),
        }
    }
}

/// GitHub pull request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullRequest {
    pub id: u64,
    pub number: u64,
    pub title: String,
    pub body: Option<String>,
    pub user: User,
    pub state: String,
    pub html_url: String,
    pub head: PullRequestRef,
    pub base: PullRequestRef,
}

impl PullRequest {
    fn from_octocrab_pull(pull: octocrab::models::pulls::PullRequest) -> Self {
        Self {
            id: pull.id.0,
            number: pull.number,
            title: pull.title.unwrap_or_default(),
            body: pull.body,
            user: User::from_octocrab_user(*pull.user.unwrap()),
            state: format!("{:?}", pull.state.unwrap()).to_lowercase(),
            html_url: pull.html_url.unwrap().to_string(),
            head: PullRequestRef::from_octocrab_head(*pull.head),
            base: PullRequestRef::from_octocrab_base(*pull.base),
        }
    }
}

/// GitHub pull request reference (head/base)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullRequestRef {
    pub label: String,
    pub ref_name: String,
    pub sha: String,
}

impl PullRequestRef {
    fn from_octocrab_head(head: octocrab::models::pulls::Head) -> Self {
        Self {
            label: head.label.unwrap_or_default(),
            ref_name: head.ref_field,
            sha: head.sha,
        }
    }

    fn from_octocrab_base(base: octocrab::models::pulls::Base) -> Self {
        Self {
            label: base.label.unwrap_or_default(),
            ref_name: base.ref_field,
            sha: base.sha,
        }
    }
}

/// GitHub App installation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Installation {
    pub id: u64,
    pub account: User,
    pub repository_selection: String,
    pub access_tokens_url: String,
}

/// Re-export octocrab types for advanced usage
pub use octocrab;