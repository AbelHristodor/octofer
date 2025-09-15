//! # Complete Issue Comment Handler with GitHub API
//!
//! This example shows a complete implementation of an issue comment handler
//! that uses octocrab's API to interact with GitHub in response to issue comments.
//! 
//! To run this example with actual GitHub integration, you would need:
//! 1. A GitHub App or Personal Access Token
//! 2. Set GITHUB_TOKEN environment variable
//! 3. Webhook endpoint configured to receive GitHub events

use anyhow::Result;
use octofer::Octofer;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Create a new Octofer app
    let app = Octofer::new("issue-comment-bot").await?;

    // Handle issue comment events
    app.on_issue_comment(|context| async move {
        tracing::info!("🎯 Issue comment event received!");
        
        // Parse the issue comment event
        let event = context.as_issue_comment()?;
        
        tracing::info!("📝 Action: {}", event.action);
        
        // Only handle "created" comments (new comments)
        if event.action == "created" {
            if let (Some(owner), Some(repo), Some(issue_num), Some(comment_body), Some(author)) = (
                event.repository_owner(),
                event.repository_name(),
                event.issue_number(),
                event.comment_body(),
                event.comment_author()
            ) {
                tracing::info!("📍 Repository: {}/{}", owner, repo);
                tracing::info!("🔢 Issue #: {}", issue_num);
                tracing::info!("👤 Comment author: {}", author);
                tracing::info!("💬 Comment: {}", comment_body);
                
                // Example 1: Respond to help requests
                if comment_body.to_lowercase().contains("help") {
                    tracing::info!("🆘 Help request detected! Would respond with help information.");
                    
                    // Simulated API call - in real usage, you'd create the GitHub client with auth:
                    // let github = GitHubClient::with_token(std::env::var("GITHUB_TOKEN")?)?;
                    // let response = "Here's how I can help you:\n\n- Ask me about the codebase\n- Request a code review\n- Report bugs";
                    // github.create_issue_comment(owner, repo, issue_num, response).await?;
                }
                
                // Example 2: Handle mentions of the bot
                if comment_body.contains("@bot") || comment_body.contains("@octofer") {
                    tracing::info!("🤖 Bot mention detected! Would acknowledge the mention.");
                    
                    // Simulated response
                    // let response = format!("Hi @{}, I see you mentioned me! How can I help?", author);
                    // github.create_issue_comment(owner, repo, issue_num, &response).await?;
                }
                
                // Example 3: Auto-label based on comment content
                if comment_body.to_lowercase().contains("bug") {
                    tracing::info!("🐛 Bug report detected! Would add 'bug' label.");
                    
                    // In real usage, you'd use octocrab to add labels:
                    // github.octocrab().issues(owner, repo).add_labels(issue_num, &["bug"]).await?;
                }
                
                // Example 4: Trigger actions based on keywords
                match comment_body.to_lowercase() {
                    body if body.contains("run tests") => {
                        tracing::info!("🧪 Test run requested! Would trigger CI/CD pipeline.");
                    }
                    body if body.contains("deploy") => {
                        tracing::info!("🚀 Deployment requested! Would trigger deployment workflow.");
                    }
                    body if body.contains("close") && author == owner => {
                        tracing::info!("🔒 Close request from owner! Would close the issue.");
                        // github.octocrab().issues(owner, repo).update(issue_num).state(octocrab::models::IssueState::Closed).send().await?;
                    }
                    _ => {}
                }
            }
        } else {
            tracing::info!("ℹ️  Ignoring '{}' action (only processing 'created' comments)", event.action);
        }
        
        Ok(())
    });

    // Start the application and webhook server
    tracing::info!("🚀 Starting Octofer issue comment bot...");
    tracing::info!("📡 Webhook server will listen on port 3000");
    tracing::info!("⚠️  This example shows simulated API calls - set GITHUB_TOKEN for real integration");
    
    app.start().await?;

    Ok(())
}