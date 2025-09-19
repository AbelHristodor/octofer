<p align="center">
  <a href="#"><img src=".github/assets/logo.PNG" width="160" alt="Octofer's logo" /></a>
</p>
<h3 align="center"><a href="#">Octofer</a></h3>
<p align="center">A framework for building GitHub Apps, in Rust </p>

---

> ⚠️ **Under Development** - This framework is currently in active development and not yet ready for production use.

A framework for building GitHub Apps in Rust, inspired by [Probot](https://github.com/probot/probot). Octofer provides a clean, type-safe way to build GitHub Apps with minimal boilerplate and automatic webhook handling.

## What is Octofer?

Octofer simplifies GitHub App development by:

- **Handling webhook events** - Automatically receives and routes GitHub webhook events
- **Managing authentication** - Handles GitHub App authentication and installation tokens
- **Providing type safety** - Full Rust type safety for GitHub API interactions
- **Offering simple APIs** - Clean, intuitive event handler registration

## Available Event Handlers

Currently supported GitHub webhook events:

- `on_issue_comment()` - Issue comment events (created, edited, deleted)
- `on_issue()` - Issue events (opened, closed, edited, etc.)
- `on_pull_request()` - Pull request events (opened, closed, merged, etc.)
- `on_pull_request_review()` - Pull request review events
- `on_pull_request_review_comment()` - Pull request review comment events
- `on_pull_request_review_thread()` - Pull request review thread events

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
