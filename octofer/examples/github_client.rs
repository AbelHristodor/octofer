//! # GitHub Client Example
//!
//! This example demonstrates how to use the GitHub client with octocrab integration.

use anyhow::Result;
use octofer_github::GitHubClient;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Create a GitHub client without authentication (for public repos)
    let client = GitHubClient::new();

    // Example: Get repository information
    match client.get_repo("octocat", "Hello-World").await {
        Ok(repo) => {
            println!("Repository: {}", repo.full_name);
            println!("Description: {:?}", repo.description);
            println!("Default branch: {}", repo.default_branch);
            println!("HTML URL: {}", repo.html_url);
        }
        Err(e) => {
            println!("Error fetching repository: {}", e);
        }
    }

    // Example with authentication (uncomment and add your token)
    // let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN must be set");
    // let authenticated_client = GitHubClient::with_token(token)?;

    // Example: Get issues from a repository
    // match authenticated_client.get_issues("owner", "repo").await {
    //     Ok(issues) => {
    //         println!("Found {} issues", issues.len());
    //         for issue in issues.iter().take(5) {
    //             println!("  #{}: {}", issue.number, issue.title);
    //         }
    //     }
    //     Err(e) => {
    //         println!("Error fetching issues: {}", e);
    //     }
    // }

    // Example: GitHub App authentication (uncomment and provide credentials)
    // let app_id = 123456;
    // let private_key = std::fs::read_to_string("path/to/private-key.pem")?;
    // let app_client = GitHubClient::with_app_credentials(app_id, private_key)?;

    // Example: Create an issue comment
    // let comment = app_client.create_issue_comment(
    //     "owner",
    //     "repo",
    //     1,
    //     "Hello from Octofer!"
    // ).await?;
    // println!("Created comment: {}", comment.html_url);

    // Access the underlying octocrab client for advanced usage
    let octocrab = client.octocrab();
    println!("Octocrab client is available for advanced GitHub API operations");

    Ok(())
}
