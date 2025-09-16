# Octofer - GitHub Apps Framework for Rust

Always follow these instructions first and only fallback to additional search and context gathering if the information here is incomplete or found to be in error.

## Project Overview

Octofer is a Rust framework for building GitHub Apps, inspired by Probot. It uses a Cargo workspace architecture with 2 crates:

- **`octofer`** - Main framework crate with all core functionality organized into modules
- **`octofer-cli`** - CLI tools for app scaffolding and development

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

### Running Examples and CLI

```bash
# Run the GitHub client example (demonstrates API integration)
cargo run --example github_client

# Use the CLI tool
cargo run -p octofer-cli -- --help
cargo run -p octofer-cli -- new my-app
cargo run -p octofer-cli -- dev --port 3000
```

**Note:** Several examples currently have compilation errors due to API changes and need to be updated.

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
   # Test the GitHub client example (should start and stop cleanly)
   cargo run --example github_client
   
   # Test CLI functionality
   cargo run -p octofer-cli -- --help
   ```

4. **Note on Examples:**
   ```bash
   # Several examples currently have compilation errors and need updates:
   # - basic.rs: API method name changes (event_type -> event)
   # - issue_comment_handler.rs: Missing payload() method
   # - complete_issue_comment_bot.rs: Same API issues
   # - github_context_demo.rs: Sync trait issues
   ```

**Expected Outcomes:**
- GitHub client example should complete silently without errors
- CLI should display help with "new" and "dev" subcommands
- All builds should complete without compilation errors
- Formatting check should pass with no diffs
- Clippy should pass with no warnings
- Some examples will currently fail to compile due to API changes

## Common Development Tasks

### Project Structure Navigation
```bash
# Repository root structure:
# .
# ├── Cargo.toml              # Workspace definition
# ├── README.md               # Project documentation  
# ├── .gitignore              # Git ignore rules
# ├── .github/                # GitHub configuration
# │   └── copilot-instructions.md  # This file
# ├── src/                    # Main framework code
# │   ├── lib.rs             # Main library entry point
# │   ├── core.rs            # Core types and Context
# │   ├── config.rs          # Configuration management
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
# ├── examples/              # Usage examples
# │   ├── basic.rs           # Basic app example (needs fixes)
# │   ├── github_client.rs   # Working API client demo
# │   ├── issue_comment_handler.rs      # Comment handler (needs fixes)
# │   ├── complete_issue_comment_bot.rs # Complete bot (needs fixes)
# │   └── github_context_demo.rs        # Context demo (needs fixes)
# └── octofer-cli/           # CLI tooling crate
#     ├── Cargo.toml         # CLI crate config
#     └── src/
#         └── main.rs        # CLI implementation
```

### Working with the Codebase

When making changes to specific functionality:

- **Core types/traits**: Edit `src/core.rs`
- **GitHub API features**: Edit `src/github/client.rs`, `src/github/models.rs`
- **Authentication**: Edit `src/github/auth.rs`
- **Webhook handling**: Edit `src/webhook/server.rs`, `src/webhook/handlers.rs`
- **Configuration**: Edit `src/config.rs`
- **CLI commands**: Edit `octofer-cli/src/main.rs`
- **Main framework**: Edit `src/lib.rs`
- **Examples**: Add to `examples/`

### Adding Dependencies

Dependencies are managed at the workspace level in the root `Cargo.toml`. The main crate dependencies are in the `[dependencies]` section:

```toml
[dependencies]
# Add new dependencies here
new-dependency = "version"
```

The CLI has its own dependencies in `octofer-cli/Cargo.toml`.

### Known Issues and Limitations

1. **Example Compilation Errors**: Several examples have compilation errors due to API changes:
   - `basic.rs`: Uses `context.event_type()` which should be `context.event()`
   - `issue_comment_handler.rs`: Uses `context.payload()` method which doesn't exist
   - `complete_issue_comment_bot.rs`: Same payload API issues
   - `github_context_demo.rs`: Future Sync trait requirements issues

2. **Network Dependencies**: The GitHub client example may fail with network errors when accessing the GitHub API without proper authentication or due to rate limiting. This is normal behavior.

3. **API Evolution**: The framework is actively being developed and some examples may lag behind the current API.

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

# 5. Test examples (only working ones)
cargo run --example github_client
```

### Debugging and Troubleshooting
```bash
# Clean rebuild if having dependency issues
cargo clean && cargo build

# Check specific crate
cargo check -p octofer-cli

# Run with debug output (if example works)
RUST_LOG=debug cargo run --example github_client

# Build individual crate
cargo build -p octofer-cli
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