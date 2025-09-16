//! # GitHub Client Example
//!
//! This example demonstrates how to use the GitHub client directly.

use anyhow::Result;
use octofer::{
    github::{GitHubAuth, GitHubClient},
    Config,
};
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    info!("GitHub Client Example");

    // Example 1: Create client from configuration (if available)
    if let Ok(config) = Config::from_env() {
        info!("Creating GitHub client from environment configuration");

        let auth = GitHubAuth::from_config(&config.github);
        let client = GitHubClient::new(auth).await?;

        // Get installations
        match client.get_installations().await {
            Ok(installations) => {
                info!("Found {} installations", installations.len());
                for installation in installations {
                    info!("  Installation ID: {}", installation.id.0);
                    info!("  Account: {}", installation.account.login);
                }
            }
            Err(e) => {
                info!("Error fetching installations: {}", e);
            }
        }

        // Example: Use installation client
        if let Ok(installations) = client.get_installations().await {
            if let Some(installation) = installations.first() {
                let installation_id = installation.id.0;
                info!("Using installation client for ID: {}", installation_id);

                match client.get_installation_repositories(installation_id).await {
                    Ok(repos) => {
                        info!("Found {} repositories", repos.len());
                        for repo in repos.iter().take(3) {
                            info!(
                                "  Repository: {}",
                                repo.full_name.as_deref().unwrap_or("unknown")
                            );
                        }
                    }
                    Err(e) => {
                        info!("Error fetching repositories: {}", e);
                    }
                }
            }
        }
    } else {
        info!("No environment configuration found");
        info!("To test GitHub client functionality, set the following environment variables:");
        info!("  GITHUB_APP_ID=your_app_id");
        info!("  GITHUB_PRIVATE_KEY_PATH=path/to/private-key.pem");
        info!("  OR GITHUB_PRIVATE_KEY_BASE64=base64_encoded_key");

        info!("Example completed - network operations skipped due to missing credentials");
    }

    Ok(())
}
