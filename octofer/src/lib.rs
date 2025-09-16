use anyhow::Result;
use octofer_core::Context;
use octofer_webhook::WebhookServer;
use std::net::Ipv4Addr;

#[derive(Default)]
pub struct Octofer {
    server: WebhookServer,
}

impl Octofer {
    pub fn new() -> Self {
        Octofer {
            server: WebhookServer::new(Ipv4Addr::new(127, 0, 0, 1), 8000),
        }
    }

    pub async fn start(&self) -> anyhow::Result<()> {
        self.server.start().await
    }

    /// Add an issue comment event handler
    pub async fn on_issue_comment<F, Fut>(&mut self, handler: F) -> &Self
    where
        F: Fn(Context) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Result<()>> + Send + Sync + 'static,
    {
        self.server.on("issue_comment", handler).await;
        self
    }
}
