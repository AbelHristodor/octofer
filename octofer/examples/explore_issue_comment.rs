//! Test to explore octocrab webhook event types

use octofer::octocrab::models::webhook_events::WebhookEventType;

#[tokio::main]
async fn main() {
    println!("Exploring octocrab webhook event types");
    println!("IssueComment: {:?}", WebhookEventType::IssueComment);
    
    // Let's also check what a typical issue comment payload might look like
    let sample_payload = serde_json::json!({
        "action": "created",
        "issue": {
            "number": 1,
            "title": "Test Issue",
            "body": "This is a test issue",
            "user": {
                "login": "testuser",
                "id": 123
            }
        },
        "comment": {
            "id": 456,
            "body": "This is a test comment",
            "user": {
                "login": "commentuser",
                "id": 789
            }
        },
        "repository": {
            "name": "test-repo",
            "full_name": "owner/test-repo",
            "owner": {
                "login": "owner",
                "id": 111
            }
        }
    });
    
    println!("Sample issue comment payload: {}", serde_json::to_string_pretty(&sample_payload).unwrap());
}