//! Event handler registration for GitHub webhook events
//!
//! This module provides methods for registering event handlers for different types
//! of GitHub webhook events. Each method allows you to register a handler function
//! that will be called when the corresponding event is received.
//!
//! # Available Event Handlers
//!
//! ## Issue Events
//! - [`on_issue()`](../struct.Octofer.html#method.on_issue) - Issue opened, closed, edited, etc.
//! - [`on_issue_comment()`](../struct.Octofer.html#method.on_issue_comment) - Comments on issues
//!
//! ## Pull Request Events  
//! - [`on_pull_request()`](../struct.Octofer.html#method.on_pull_request) - PR opened, closed, merged, etc.
//! - [`on_pull_request_review()`](../struct.Octofer.html#method.on_pull_request_review) - PR reviews submitted
//! - [`on_pull_request_review_comment()`](../struct.Octofer.html#method.on_pull_request_review_comment) - Comments on PR reviews
//! - [`on_pull_request_review_thread()`](../struct.Octofer.html#method.on_pull_request_review_thread) - PR review threads
//!
//! ## Repository Events
//! - [`on_push()`](../struct.Octofer.html#method.on_push) - Push to repository
//! - [`on_create()`](../struct.Octofer.html#method.on_create) - Branch/tag created
//! - [`on_delete()`](../struct.Octofer.html#method.on_delete) - Branch/tag deleted
//! - [`on_fork()`](../struct.Octofer.html#method.on_fork) - Repository forked
//! - [`on_commit_comment()`](../struct.Octofer.html#method.on_commit_comment) - Comment on commit
//! - [`on_gollum()`](../struct.Octofer.html#method.on_gollum) - Wiki page update
//! - [`on_public()`](../struct.Octofer.html#method.on_public) - Repository made public
//! - [`on_repository()`](../struct.Octofer.html#method.on_repository) - Repository events
//! - [`on_repository_dispatch()`](../struct.Octofer.html#method.on_repository_dispatch) - Repository dispatch
//! - [`on_repository_import()`](../struct.Octofer.html#method.on_repository_import) - Repository import
//! - [`on_branch_protection_rule()`](../struct.Octofer.html#method.on_branch_protection_rule) - Branch protection
//!
//! ## Workflow Events
//! - [`on_workflow_run()`](../struct.Octofer.html#method.on_workflow_run) - Workflow run
//! - [`on_workflow_job()`](../struct.Octofer.html#method.on_workflow_job) - Workflow job
//! - [`on_workflow_dispatch()`](../struct.Octofer.html#method.on_workflow_dispatch) - Workflow dispatch
//! - [`on_status()`](../struct.Octofer.html#method.on_status) - Commit status
//!
//! ## Check & Security Events
//! - [`on_check_run()`](../struct.Octofer.html#method.on_check_run) - Check run
//! - [`on_check_suite()`](../struct.Octofer.html#method.on_check_suite) - Check suite
//! - [`on_code_scanning_alert()`](../struct.Octofer.html#method.on_code_scanning_alert) - Code scanning alert
//! - [`on_secret_scanning_alert()`](../struct.Octofer.html#method.on_secret_scanning_alert) - Secret scanning alert
//! - [`on_dependabot_alert()`](../struct.Octofer.html#method.on_dependabot_alert) - Dependabot alert
//! - [`on_security_advisory()`](../struct.Octofer.html#method.on_security_advisory) - Security advisory
//!
//! ## Deployment Events
//! - [`on_deployment()`](../struct.Octofer.html#method.on_deployment) - Deployment
//! - [`on_deployment_status()`](../struct.Octofer.html#method.on_deployment_status) - Deployment status
//! - [`on_deploy_key()`](../struct.Octofer.html#method.on_deploy_key) - Deploy key
//!
//! ## Discussion Events
//! - [`on_discussion()`](../struct.Octofer.html#method.on_discussion) - Discussion
//! - [`on_discussion_comment()`](../struct.Octofer.html#method.on_discussion_comment) - Discussion comment
//!
//! ## Project Events
//! - [`on_project()`](../struct.Octofer.html#method.on_project) - Project (classic)
//! - [`on_project_card()`](../struct.Octofer.html#method.on_project_card) - Project card
//! - [`on_project_column()`](../struct.Octofer.html#method.on_project_column) - Project column
//! - [`on_projects_v2()`](../struct.Octofer.html#method.on_projects_v2) - Projects v2
//! - [`on_projects_v2_item()`](../struct.Octofer.html#method.on_projects_v2_item) - Projects v2 item
//!
//! ## Team & Organization Events
//! - [`on_team()`](../struct.Octofer.html#method.on_team) - Team
//! - [`on_team_add()`](../struct.Octofer.html#method.on_team_add) - Team add
//! - [`on_member()`](../struct.Octofer.html#method.on_member) - Member
//! - [`on_membership()`](../struct.Octofer.html#method.on_membership) - Membership
//! - [`on_organization()`](../struct.Octofer.html#method.on_organization) - Organization
//! - [`on_org_block()`](../struct.Octofer.html#method.on_org_block) - Org block
//!
//! ## Release & Package Events
//! - [`on_release()`](../struct.Octofer.html#method.on_release) - Release
//! - [`on_package()`](../struct.Octofer.html#method.on_package) - Package
//! - [`on_registry_package()`](../struct.Octofer.html#method.on_registry_package) - Registry package
//!
//! ## Installation Events
//! - [`on_installation()`](../struct.Octofer.html#method.on_installation) - Installation
//! - [`on_installation_repositories()`](../struct.Octofer.html#method.on_installation_repositories) - Installation repositories
//! - [`on_github_app_authorization()`](../struct.Octofer.html#method.on_github_app_authorization) - App authorization
//!
//! ## Miscellaneous Events
//! - [`on_label()`](../struct.Octofer.html#method.on_label) - Label
//! - [`on_milestone()`](../struct.Octofer.html#method.on_milestone) - Milestone
//! - [`on_watch()`](../struct.Octofer.html#method.on_watch) - Watch (star)
//! - [`on_star()`](../struct.Octofer.html#method.on_star) - Star
//! - [`on_ping()`](../struct.Octofer.html#method.on_ping) - Ping
//! - [`on_sponsorship()`](../struct.Octofer.html#method.on_sponsorship) - Sponsorship
//! - [`on_merge_group()`](../struct.Octofer.html#method.on_merge_group) - Merge group
//!
//! # Handler Function Signature
//!
//! All event handlers must have the following signature:
//!
//! ```rust,ignore
//! async fn handler(context: Context, extra: Arc<ExtraData>) -> anyhow::Result<()>
//! ```
//!
//! Where:
//! - `context` - Contains the webhook event data and GitHub API client
//! - `extra` - Additional data you want to pass to the handler
//!
//! # Examples
//!
//! ## Basic Issue Handler
//!
//! ```rust,no_run
//! use octofer::{Octofer, Config, Context};
//! use std::sync::Arc;
//!
//! # async fn example() -> anyhow::Result<()> {
//! let config = Config::default();
//! let mut app = Octofer::new(config).await.unwrap_or_else(|_| Octofer::new_default());
//!
//! // Register an issue event handler
//! app.on_issue(
//!     |context: Context, _extra: Arc<()>| async move {
//!         println!("Issue event: {}", context.kind());
//!         
//!         // Access the event payload
//!         let payload = context.payload();
//!         if let Some(action) = payload.get("action") {
//!             println!("Action: {}", action);
//!         }
//!         
//!         Ok(())
//!     },
//!     Arc::new(()),
//! ).await;
//! # Ok(())
//! # }
//! ```
//!
//! ## Handler with GitHub API Usage
//!
//! ```rust,no_run
//! use octofer::{Octofer, Config, Context};
//! use std::sync::Arc;
//!
//! # async fn example() -> anyhow::Result<()> {
//! let config = Config::from_env().unwrap_or_default();
//! let mut app = Octofer::new(config).await.unwrap_or_else(|_| Octofer::new_default());
//!
//! app.on_issue_comment(
//!     |context: Context, _extra: Arc<()>| async move {
//!         println!("Issue comment event received");
//!         
//!         // Use the GitHub client if available
//!         if let Some(client) = context.github() {
//!             let installations = client.get_installations().await?;
//!             println!("GitHub client available, found {} installations", installations.len());
//!         }
//!         
//!         // Use installation client for repository operations
//!         if let Some(installation_client) = context.installation_client().await? {
//!             let user = installation_client.current().user().await?;
//!             println!("Acting as: {}", user.login);
//!         }
//!         
//!         Ok(())
//!     },
//!     Arc::new(()),
//! ).await;
//! # Ok(())
//! # }
//! ```
//!
//! ## Handler with Custom Extra Data
//!
//! ```rust,no_run
//! use octofer::{Octofer, Config, Context};
//! use std::sync::Arc;
//!
//! #[derive(Clone, Debug)]
//! struct AppData {
//!     name: String,
//!     version: String,
//! }
//!
//! # async fn example() -> anyhow::Result<()> {
//! let config = Config::default();
//! let mut app = Octofer::new(config).await.unwrap_or_else(|_| Octofer::new_default());
//!
//! let app_data = Arc::new(AppData {
//!     name: "MyBot".to_string(),
//!     version: "1.0.0".to_string(),
//! });
//!
//! app.on_pull_request(
//!     |context: Context, extra: Arc<AppData>| async move {
//!         println!("PR event handled by {} v{}", extra.name, extra.version);
//!         
//!         let payload = context.payload();
//!         if let Some(pr) = payload.get("pull_request") {
//!             if let Some(title) = pr.get("title") {
//!                 println!("PR Title: {}", title);
//!             }
//!         }
//!         
//!         Ok(())
//!     },
//!     app_data,
//! ).await;
//! # Ok(())
//! # }
//! ```

pub mod checks;
pub mod deployments;
pub mod discussions;
pub mod installations;
pub mod issues;
pub mod misc;
pub mod projects;
pub mod prs;
pub mod releases;
pub mod repository;
pub mod teams;
pub mod workflows;
