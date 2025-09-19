# Octofer - GitHub Apps Framework for Rust

> ⚠️ **Under Development** - This framework is currently in active development and not yet ready for production use.

Always follow these instructions first and only fallback to additional search and context gathering if the information here is incomplete or found to be in error.

## Project Overview

Octofer is a Rust framework for building GitHub Apps, inspired by Probot. It provides a clean, type-safe way to build GitHub Apps with minimal boilerplate and automatic webhook handling.

**Current Status**: Single crate architecture with modular organization:
- **`src/`** - Main framework with modules for config, core, events, github, and webhook handling

## Available Event Handlers

Currently supported GitHub webhook events:

- `on_issue_comment()` - Issue comment events (created, edited, deleted)
- `on_issue()` - Issue events (opened, closed, edited, etc.)
- `on_pull_request()` - Pull request events (opened, closed, merged, etc.)
- `on_pull_request_review()` - Pull request review events
- `on_pull_request_review_comment()` - Pull request review comment events
- `on_pull_request_review_thread()` - Pull request review thread events

## Essential Build and Development Commands

### Initial Setup and Dependencies
All dependencies are managed through Cargo. No additional tools or SDKs need to be installed beyond a standard Rust toolchain.

```bash
# Ensure you have Rust installed (if not already available)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### Building the Project
```bash
# Clean build (when dependencies change or for first build)
cargo clean && cargo build

# Incremental build (for development)
cargo build

# Build release version
cargo build --release
```

**CRITICAL TIMING:** Initial clean build takes approximately 1 minute 19 seconds. NEVER CANCEL builds before 2 minutes. Always set timeout to 180+ seconds for build commands.

### Running Tests
```bash
# Run all tests (currently doctests only, no unit tests implemented)
cargo test

# Build test binaries only (faster for checking compilation)
cargo test --no-run
```

**TIMING:** Test compilation takes ~4 seconds. Test execution is currently minimal since no unit tests are implemented yet. Always set timeout to 60+ seconds for test commands.

### Code Quality and Linting

**CRITICAL:** Always run formatting and linting before committing changes. The codebase requires clean formatting and clippy compliance.

```bash
# Format all code (REQUIRED before commits)
cargo fmt

# Check formatting without modifying files
cargo fmt --check

# Run clippy linter with strict warnings (REQUIRED before commits)
cargo clippy -- -D warnings

# Quick compilation check
cargo check
```

### Running Examples

```bash
# Run the basic GitHub App example
cargo run --example basic

# Run the GitHub client example (demonstrates API integration)
cargo run --example github_client

# Run logging configuration test
cargo run --example logging_test

# Test with custom logging
OCTOFER_LOG_FORMAT=json OCTOFER_LOG_LEVEL=debug cargo run --example logging_test
```

## Validation Scenarios

### MANDATORY: Always Test These Scenarios After Changes

1. **Build Validation:**
   ```bash
   cargo clean && cargo build
   cargo test --no-run
   ```

2. **Code Quality Validation:**
   ```bash
   cargo fmt --check
   cargo clippy -- -D warnings
   ```

3. **Basic Functionality Validation:**
   ```bash
   # Test the basic example (should start webhook server)
   cargo run --example basic
   
   # Test the GitHub client example
   cargo run --example github_client
   
   # Test logging configuration
   cargo run --example logging_test
   ```

**Expected Outcomes:**
- Basic example should start webhook server on localhost:8000
- GitHub client example should complete silently without errors
- Logging test should demonstrate different log formats
- All builds should complete without compilation errors
- Formatting check should pass with no diffs
- Clippy should pass with no warnings

## Common Development Tasks

### Project Structure Navigation
```bash
# Repository root structure:
# .
# ├── Cargo.toml              # Project configuration
# ├── README.md               # Project documentation  
# ├── TESTING.md              # Testing framework documentation
# ├── .gitignore              # Git ignore rules
# ├── .github/                # GitHub configuration
# │   └── copilot-instructions.md  # This file
# ├── src/                    # Main framework code
# │   ├── lib.rs             # Main library entry point
# │   ├── core.rs            # Core types and Context
# │   ├── config.rs          # Configuration management
# │   ├── events/            # Event handler implementations
# │   │   ├── mod.rs         # Events module entry
# │   │   ├── issues.rs      # Issue and issue comment handlers
# │   │   └── prs.rs         # Pull request handlers
# │   ├── github/            # GitHub API integration
# │   │   ├── mod.rs         # GitHub module entry
# │   │   ├── auth.rs        # Authentication logic
# │   │   ├── client.rs      # GitHub client wrapper
# │   │   ├── models.rs      # GitHub data models
# │   │   └── middlewares/   # Request/response middlewares
# │   │       ├── mod.rs     # Middleware module entry
# │   │       ├── events.rs  # Event processing
# │   │       └── hmac.rs    # HMAC verification
# │   └── webhook/           # Webhook handling
# │       ├── mod.rs         # Webhook module entry
# │       ├── server.rs      # HTTP server implementation
# │       └── handlers.rs    # Event routing
# └── examples/              # Usage examples
#     ├── basic.rs           # Basic app example
#     ├── github_client.rs   # Working API client demo
#     └── logging_test.rs    # Logging configuration demo
```

### Working with the Codebase

When making changes to specific functionality:

- **Core types/traits**: Edit `src/core.rs`
- **Event handlers**: Edit `src/events/issues.rs` or `src/events/prs.rs`
- **GitHub API features**: Edit `src/github/client.rs`, `src/github/models.rs`
- **Authentication**: Edit `src/github/auth.rs`
- **Webhook handling**: Edit `src/webhook/server.rs`, `src/webhook/handlers.rs`
- **Configuration**: Edit `src/config.rs`
- **Main framework**: Edit `src/lib.rs`
- **Examples**: Add to `examples/`

### Adding Dependencies

Dependencies are managed in the root `Cargo.toml`:

```toml
[dependencies]
# Add new dependencies here
new-dependency = "version"
```

### Known Issues and Limitations

1. **Development Status**: Framework is under active development and not production-ready

2. **Network Dependencies**: The GitHub client example may fail with network errors when accessing the GitHub API without proper authentication or due to rate limiting. This is normal behavior.

3. **Configuration Requirements**: Most functionality requires proper GitHub App configuration via environment variables

## Quick Reference Commands

### Daily Development Workflow
```bash
# 1. Format code
cargo fmt

# 2. Check compilation  
cargo check

# 3. Run linter
cargo clippy -- -D warnings

# 4. Run tests
cargo test

# 5. Test examples
cargo run --example basic
cargo run --example github_client
cargo run --example logging_test
```

### Debugging and Troubleshooting
```bash
# Clean rebuild if having dependency issues
cargo clean && cargo build

# Run with debug output
RUST_LOG=debug cargo run --example basic

# Run with custom logging configuration
OCTOFER_LOG_FORMAT=json OCTOFER_LOG_LEVEL=debug cargo run --example logging_test
```

### Performance Notes
- Initial clean build: ~1 minute 19 seconds
- Incremental builds: ~4 seconds  
- Test compilation: ~4 seconds
- Example execution: <5 seconds

Always allow sufficient timeout for operations:
- Build commands: 180+ seconds
- Test commands: 60+ seconds  
- Check/clippy: 60+ seconds

**NEVER CANCEL** long-running commands prematurely. Rust compilation can be slow, especially for initial builds with many dependencies.