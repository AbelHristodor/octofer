# Octofer

A framework for building GitHub Apps in Rust, inspired by Probot.

## Overview

Octofer provides a clean, type-safe way to build GitHub Apps in Rust. It's designed with modularity in mind, splitting functionality into separate crates for better maintainability and allowing users to include only what they need.

## Architecture

The framework is organized into multiple cargo crates:

- **`octofer`** - Main framework crate that re-exports everything
- **`octofer-core`** - Core types, traits and utilities
- **`octofer-github`** - GitHub API client and authentication
- **`octofer-webhook`** - Webhook handling and event routing
- **`octofer-cli`** - CLI tools for app scaffolding

## Quick Start

Add Octofer to your `Cargo.toml`:

```toml
[dependencies]
octofer = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

Create your GitHub App:

```rust
use octofer::Octofer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Octofer::new("my-github-app").await?;
    
    app.on_issues(|context| async move {
        println!("Issue event received: {:?}", context.payload());
        Ok(())
    });
    
    app.start().await?;
    Ok(())
}
```

## Development

This project uses Cargo workspaces. To build all crates:

```bash
cargo build
```

To run tests:

```bash
cargo test
```

To run the CLI:

```bash
cargo run --bin octofer -- --help
```