use anyhow::Result;
use octofer::{
    Octofer,
    github::webhook_events::WebhookEventPayload,
    octocrab::{self, params::issues},
};

#[tokio::main]
async fn main() -> Result<()> {
    let config = octofer::Config::from_env()?;
    let mut app = Octofer::new(config).await?;

    app.on_issue(handler, std::sync::Arc::new(())).await;

    app.start().await?;

    Ok(())
}

async fn handler(ctx: octofer::Context, e: std::sync::Arc<()>) -> Result<()> {
    let event = if let Some(e) = &ctx.event {
        e
    } else {
        return Err(anyhow::anyhow!("Could not get event"));
    };

    let client = if let Some(client) = ctx.installation_client().await? {
        client
    } else {
        return Err(anyhow::anyhow!("Could not retrieve github client!"));
    };

    let author = event
        .sender
        .clone()
        .ok_or(anyhow::anyhow!("Could not get author"))?
        .login;

    let repo = &event
        .repository
        .clone()
        .ok_or(anyhow::anyhow!("Could not get repo!"))?;

    let owner = repo
        .owner
        .clone()
        .ok_or(anyhow::anyhow!("Could not get owner!"))?
        .login;

    let repo = repo.name.clone();

    let WebhookEventPayload::Issues(payload) = &event.specific else {
        panic!();
    };

    let issue = payload.issue.clone();

    let issues = client
        .issues(owner, repo)
        .list()
        .state(octocrab::params::State::Open)
        .sort(issues::Sort::Created)
        .creator(author)
        .send()
        .await?;

    // If this is the first issue, comment
    if issues.items.len() == 1 && issues.items[0].number == issue.number {
        // First issue!
        println!("This is the first issue!");
    }

    Ok(())
}
