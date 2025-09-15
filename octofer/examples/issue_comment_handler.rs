//! # Issue Comment Event Handler Example
//!
//! This example demonstrates how to handle issue comment events using Octofer.
//! When an issue comment is created, modified, or deleted, this handler will be triggered.

use anyhow::Result;
use octofer::Octofer;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Create a new Octofer app
    let app = Octofer::new("issue-comment-handler-app").await?;

    // Handle issue comment events
    app.on_issue_comment(|context| async move {
        tracing::info!("Issue comment event received!");

        // Check if this is an issue comment event
        if context.is_issue_comment() {
            // Parse the issue comment event data
            match context.as_issue_comment() {
                Ok(event) => {
                    tracing::info!("Action: {}", event.action);

                    if let Some(repo_name) = event.repository_full_name() {
                        tracing::info!("Repository: {}", repo_name);
                    }

                    if let Some(issue_number) = event.issue_number() {
                        tracing::info!("Issue #{}", issue_number);
                    }

                    if let Some(issue_title) = event.issue_title() {
                        tracing::info!("Issue title: {}", issue_title);
                    }

                    if let Some(comment_body) = event.comment_body() {
                        tracing::info!("Comment: {}", comment_body);
                    }

                    if let Some(author) = event.comment_author() {
                        tracing::info!("Comment author: {}", author);
                    }

                    // Example: Respond to certain comments using GitHub API
                    if let (Some(owner), Some(repo), Some(issue_num), Some(body)) = (
                        event.repository_owner(),
                        event.repository_name(),
                        event.issue_number(),
                        event.comment_body(),
                    ) {
                        // Only respond to comments that mention the bot
                        if body.contains("@bot") && event.action == "created" {
                            tracing::info!("Bot was mentioned! We could respond here with:");
                            tracing::info!("  Owner: {}", owner);
                            tracing::info!("  Repo: {}", repo);
                            tracing::info!("  Issue: {}", issue_num);

                            // Example of how you might use the GitHub client to respond:
                            // let github = app.github();
                            // let response = format!("Thanks for mentioning me! I see you commented: {}", body);
                            // github.create_issue_comment(owner, repo, issue_num, &response).await?;
                        }
                    }
                }
                Err(e) => {
                    tracing::error!("Failed to parse issue comment event: {}", e);
                }
            }
        }

        Ok(())
    });

    // Start the application
    tracing::info!("Starting issue comment handler...");
    app.start().await?;

    Ok(())
}
