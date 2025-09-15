# Octofer - GitHub Apps Framework for Rust

Always follow these instructions first and only fallback to additional search and context gathering if the information here is incomplete or found to be in error.

## Project Overview

Octofer is a Rust framework for building GitHub Apps, inspired by Probot. It uses a Cargo workspace architecture with 5 crates:

- **`octofer`** - Main framework crate that re-exports everything
- **`octofer-core`** - Core types, traits, and utilities  
- **`octofer-github`** - GitHub API client using octocrab
- **`octofer-webhook`** - Webhook handling and event routing
- **`octofer-cli`** - CLI tools for app scaffolding

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

**CRITICAL TIMING:** Initial clean build takes approximately 1 minute 23 seconds. NEVER CANCEL builds before 2 minutes. Always set timeout to 180+ seconds for build commands.

### Running Tests
```bash
# Run all tests (currently no unit tests implemented, only doctests)
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

### Running Examples and CLI

```bash
# Run the basic framework example
cargo run --example basic

# Run the GitHub client example (demonstrates API integration)
cargo run --example github_client

# Use the CLI tool
cargo run --bin octofer -- --help
cargo run --bin octofer -- new my-app
cargo run --bin octofer -- dev --port 3000
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
   # Test the basic example (should start and stop cleanly)
   timeout 10 cargo run --example basic
   
   # Test CLI functionality
   cargo run --bin octofer -- --help
   ```

4. **GitHub API Integration Validation:**
   ```bash
   # Run GitHub client example (may have network errors, but should compile and run)
   cargo run --example github_client
   ```

**Expected Outcomes:**
- Basic example should output: "Starting Octofer app: example-github-app"
- GitHub client example may show network errors but should complete execution
- CLI should display help with "new" and "dev" subcommands
- All builds should complete without compilation errors
- Formatting check should pass with no diffs
- Clippy should pass with no warnings

## Common Development Tasks

### Project Structure Navigation
```bash
# Repository root structure:
# .
# ├── Cargo.toml              # Workspace definition
# ├── README.md               # Project documentation  
# ├── .gitignore              # Git ignore rules
# ├── octofer/                # Main framework crate
# │   ├── src/lib.rs          # Main framework code
# │   ├── examples/           # Usage examples
# │   └── Cargo.toml          # Main crate config
# ├── octofer-core/           # Core types and traits
# │   └── src/lib.rs          # Context, payload types
# ├── octofer-github/         # GitHub API client
# │   └── src/lib.rs          # GitHubClient, models
# ├── octofer-webhook/        # Webhook handling
# │   └── src/lib.rs          # WebhookServer
# └── octofer-cli/            # CLI tools
#     └── src/main.rs         # CLI implementation
```

### Working with the Codebase

When making changes to specific functionality:

- **Core types/traits**: Edit `octofer-core/src/lib.rs`
- **GitHub API features**: Edit `octofer-github/src/lib.rs`  
- **Webhook handling**: Edit `octofer-webhook/src/lib.rs`
- **CLI commands**: Edit `octofer-cli/src/main.rs`
- **Main framework**: Edit `octofer/src/lib.rs`
- **Examples**: Add to `octofer/examples/`

### Adding Dependencies

Dependencies are managed at the workspace level in the root `Cargo.toml`:

```toml
[workspace.dependencies]
# Add new dependencies here
```

Then reference them in individual crate `Cargo.toml` files:
```toml
[dependencies]
new-dependency.workspace = true
```

### Known Issues and Limitations

1. **Doctest Failure**: The main crate has a doctest that references `app.on_issues()` method which is not yet implemented. This is expected until the event handler system is fully implemented.

2. **Network Dependencies**: The GitHub client example may fail with network errors when accessing the GitHub API without proper authentication or due to rate limiting. This is normal behavior.

3. **Incomplete Implementation**: Some methods in examples are commented out as they reference functionality that is planned but not yet implemented.

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
```

### Debugging and Troubleshooting
```bash
# Clean rebuild if having dependency issues
cargo clean && cargo build

# Check specific crate
cargo check -p octofer-core

# Run with debug output
RUST_LOG=debug cargo run --example basic

# Build individual crate
cargo build -p octofer-github
```

### Performance Notes
- Initial clean build: ~1 minute 23 seconds
- Incremental builds: ~4 seconds  
- Test compilation: ~4 seconds
- Example execution: <5 seconds

Always allow sufficient timeout for operations:
- Build commands: 180+ seconds
- Test commands: 60+ seconds  
- Check/clippy: 60+ seconds

**NEVER CANCEL** long-running commands prematurely. Rust compilation can be slow, especially for initial builds with many dependencies.