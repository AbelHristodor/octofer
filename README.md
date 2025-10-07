<p align="center">
  <a href="#"><img src="https://github.com/AbelHristodor/octofer/blob/main/.github/assets/logo.PNG?raw=true" width="160" alt="Octofer's logo" /></a>

</p>
<h3 align="center"><a href="#">Octofer</a></h3>
<p align="center">A framework for building GitHub Apps, in Rust </p>
<p align="center">
<img alt="Crates.io Version" src="https://img.shields.io/crates/v/octofer">
<img alt="GitHub Actions Workflow Status" src="https://img.shields.io/github/actions/workflow/status/AbelHristodor/octofer/tests.yaml">
<img alt="GitHub License" src="https://img.shields.io/github/license/AbelHristodor/octofer">
<img alt="GitHub top language" src="https://img.shields.io/github/languages/top/AbelHristodor/octofer">
</p>

---

> ⚠️ **Under Development** - This framework is currently in active development and may have bugs and issues.

A framework for building GitHub Apps in Rust, inspired by [Probot](https://github.com/probot/probot). Octofer provides a clean, type-safe way to build GitHub Apps with minimal boilerplate and automatic webhook handling.

## What is Octofer?

Octofer simplifies GitHub App development by:

- **Handling webhook events** - Automatically receives and routes GitHub webhook events
- **Managing authentication** - Handles GitHub App authentication and installation tokens
- **Providing type safety** - Full Rust type safety for GitHub API interactions
- **Offering simple APIs** - Clean, intuitive event handler registration

> 🛠️ Looking for a **full production-ready** example? Check out [AbelHristodor/frezze](https://github.com/AbelHristodor/frezze)! A bot that you
can use to schedule "freeze" periods for your repo, blocking all PR merges for specific periods of time!
Otherwise check the `examples` directory.

## Available Event Handlers

GitHub webhook events supported by Octofer:

### Issues & Pull Requests
- `on_issue()` - Issue events (opened, closed, edited, etc.)
- `on_issue_comment()` - Issue comment events (created, edited, deleted)
- `on_pull_request()` - Pull request events (opened, closed, merged, etc.)
- `on_pull_request_review()` - Pull request review events
- `on_pull_request_review_comment()` - Pull request review comment events
- `on_pull_request_review_thread()` - Pull request review thread events

### Repository & Git
- `on_push()` - Push events
- `on_create()` - Branch/tag created
- `on_delete()` - Branch/tag deleted
- `on_fork()` - Repository forked
- `on_commit_comment()` - Comment on commit
- `on_gollum()` - Wiki page update
- `on_public()` - Repository made public
- `on_repository()` - Repository events
- `on_repository_dispatch()` - Repository dispatch
- `on_repository_import()` - Repository import
- `on_branch_protection_rule()` - Branch protection rule events

### Workflows & Actions
- `on_workflow_run()` - Workflow run events
- `on_workflow_job()` - Workflow job events
- `on_workflow_dispatch()` - Workflow dispatch events
- `on_status()` - Commit status events

### Checks & Security
- `on_check_run()` - Check run events
- `on_check_suite()` - Check suite events
- `on_code_scanning_alert()` - Code scanning alerts
- `on_secret_scanning_alert()` - Secret scanning alerts
- `on_secret_scanning_alert_location()` - Secret scanning alert location
- `on_dependabot_alert()` - Dependabot alerts
- `on_repository_vulnerability_alert()` - Repository vulnerability alerts
- `on_security_advisory()` - Security advisory events
- `on_repository_advisory()` - Repository advisory events
- `on_security_and_analysis()` - Security and analysis events

### Deployments
- `on_deployment()` - Deployment events
- `on_deployment_status()` - Deployment status events
- `on_deploy_key()` - Deploy key events
- `on_deployment_protection_rule()` - Deployment protection rule events

### Discussions
- `on_discussion()` - Discussion events
- `on_discussion_comment()` - Discussion comment events

### Projects
- `on_project()` - Project (classic) events
- `on_project_card()` - Project card events
- `on_project_column()` - Project column events
- `on_projects_v2()` - Projects v2 events
- `on_projects_v2_item()` - Projects v2 item events

### Teams & Organizations
- `on_team()` - Team events
- `on_team_add()` - Team add events
- `on_member()` - Member events
- `on_membership()` - Membership events
- `on_organization()` - Organization events
- `on_org_block()` - Org block events

### Releases & Packages
- `on_release()` - Release events
- `on_package()` - Package events
- `on_registry_package()` - Registry package events

### Installations & Apps
- `on_installation()` - Installation events
- `on_installation_repositories()` - Installation repositories events
- `on_installation_target()` - Installation target events
- `on_github_app_authorization()` - GitHub App authorization events
- `on_personal_access_token_request()` - Personal access token request events

### Miscellaneous
- `on_label()` - Label events
- `on_milestone()` - Milestone events
- `on_watch()` - Watch (star) events
- `on_star()` - Star events
- `on_ping()` - Ping events
- `on_meta()` - Meta events
- `on_page_build()` - Page build events
- `on_schedule()` - Schedule events
- `on_sponsorship()` - Sponsorship events
- `on_marketplace_purchase()` - Marketplace purchase events
- `on_merge_group()` - Merge group events

## Quick Example

```rust
use std::sync::Arc;
use octofer::{Octofer, Config};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::from_env().unwrap_or_default();
    config.init_logging();
    
    let mut app = Octofer::new(config).await.unwrap_or_else(|_| {
        Octofer::new_default()
    });

    // Handle issue comments
    app.on_issue_comment(
        |context, _| async move {
            println!("Issue comment event received!");
            println!("Event type: {}", context.kind());
            
            if let Some(client) = &context.github_client {
                // Use GitHub API client here
                println!("GitHub client available for API calls");
            }
            
            Ok(())
        },
        Arc::new(()),
    ).await;
    
    app.start().await?;
    Ok(())
}
```

## Configuration

Octofer uses a centralized configuration system that loads from environment variables:

```bash
# Required for GitHub App authentication
export GITHUB_APP_ID=your_app_id
export GITHUB_PRIVATE_KEY_PATH=path/to/private-key.pem
# OR
export GITHUB_PRIVATE_KEY_BASE64=base64_encoded_key

# Webhook
export GITHUB_WEBHOOK_SECRET=your_webhook_secret

# Server configuration (optional)
export OCTOFER_HOST=127.0.0.1  # Default: 127.0.0.1
export OCTOFER_PORT=8000       # Default: 8000

# Logging configuration (optional)
export OCTOFER_LOG_LEVEL=info               # Default: info (trace, debug, info, warn, error)
export OCTOFER_LOG_FORMAT=compact           # Default: compact (compact, pretty, json)
export OCTOFER_LOG_WITH_TARGET=false        # Default: false (show target module)
export OCTOFER_LOG_WITH_FILE=false          # Default: false (show file and line info)
export OCTOFER_LOG_WITH_THREAD_IDS=false    # Default: false (show thread IDs)
```

You can also create configuration programmatically:

```rust
use octofer::Config;

let config = Config::new(
    123456,                                    // app_id
    Some("path/to/private-key.pem".to_string()), // private_key_path
    None,                                      // private_key_base64
    "your-webhook-secret".to_string(),         // webhook_secret
    std::net::Ipv4Addr::LOCALHOST,            // host
    8000,                                      // port
)?;

// Initialize logging with the configuration
config.init_logging();
```

## Development

Build all components:

```bash
cargo build
```

Run tests:

```bash
cargo test
```

## Code Quality

Format code:

```bash
cargo fmt
```

Run linting:

```bash
cargo clippy -- -D warnings
```

## Features

- **Centralized Configuration**: All configuration managed through a single `Config` struct
- **Environment Variable Support**: Automatic loading from environment variables
- **GitHub Client Access**: Direct access to authenticated GitHub clients from event handlers
- **Modular Architecture**: Clean separation of concerns across modules
- **Type Safety**: Full Rust type safety for GitHub API interactions
- **Automatic Token Management**: GitHub App installation token caching and refresh
- **Middleware Support**: HMAC verification and event processing middleware

## Event Handler Context

Event handlers receive a `Context` object that provides access to:

- **Event data**: `context.payload()` - The full GitHub webhook payload
- **Event**: `context.event()` - The full GitHub webhook event
- **Event type**: `context.kind()` - The type of event (e.g., "issues", "issue_comment")  
- **Installation ID**: `context.installation_id()` - GitHub App installation ID
- **GitHub client**: `context.github()` - Authenticated GitHub API client
- **Installation client**: `context.installation_client()` - Installation-specific authenticated client

## Examples

The repository includes several examples:

- `basic.rs` - Simple GitHub App with event handlers.
- `github_client.rs` - Direct GitHub API client usage

## License

MIT
