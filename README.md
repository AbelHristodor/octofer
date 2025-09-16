# Octofer

A framework for building GitHub Apps in Rust, inspired by Probot.

## Overview

Octofer provides a clean, type-safe way to build GitHub Apps in Rust. The framework features a modular architecture with centralized configuration management and a simple, intuitive API for handling GitHub webhook events.

## Architecture

The framework is organized into modules within a single crate:

- **`config`** - Centralized configuration management with environment variable support
- **`core`** - Core types, traits, and utilities (Context, EventHandler)
- **`github`** - GitHub API client and authentication with automatic token management
- **`webhook`** - Webhook handling and event routing with middleware support

Additionally, there's a separate CLI crate:

- **`octofer-cli`** - CLI tools for app scaffolding and development

## Quick Start

Add Octofer to your `Cargo.toml`:

```toml
[dependencies]
octofer = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

Create your GitHub App:

```rust
use octofer::{Octofer, Config};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::from_env()?;
    let mut app = Octofer::new(config).await?;
    
    app.on_issues(|context| async move {
        println!("Issue event received: {:?}", context.payload());
        Ok(())
    }).await;
    
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

# Webhook security
export GITHUB_WEBHOOK_SECRET=your_webhook_secret

# Server configuration (optional)
export OCTOFER_HOST=127.0.0.1  # Default: 127.0.0.1
export OCTOFER_PORT=8000       # Default: 8000
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

Run examples:

```bash
# Basic app example
cargo run --example basic

# GitHub client example
cargo run --example github_client
```

Use the CLI:

```bash
# Show help
cargo run -p octofer-cli -- --help

# Create new project (scaffolding not yet implemented)
cargo run -p octofer-cli -- new my-app

# Development server (not yet implemented)
cargo run -p octofer-cli -- dev --port 3000
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
- **Modular Architecture**: Clean separation of concerns across modules
- **Type Safety**: Full Rust type safety for GitHub API interactions
- **Automatic Token Management**: GitHub App installation token caching and refresh
- **Middleware Support**: HMAC verification and event processing middleware
- **CLI Tools**: Scaffolding and development utilities

## Examples

The repository includes several examples:

- `basic.rs` - Simple GitHub App with event handlers
- `github_client.rs` - Direct GitHub API client usage
- `issue_comment_handler.rs` - Issue comment event handling
- `complete_issue_comment_bot.rs` - Full-featured issue comment bot

## License

MIT