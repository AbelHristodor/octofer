//! Test application wrapper for simplified testing
//!
//! This module provides a TestApp that makes it easy to test Octofer applications
//! without needing to start actual servers or make real network requests.

use crate::core::{Context};
use crate::config::Config;
use crate::testing::{TestContext, MockGitHubClient};
use anyhow::Result;
use octocrab::models::webhook_events::{WebhookEvent, WebhookEventType};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

type TestEventHandler = Arc<dyn Fn(Context) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<()>> + Send>> + Send + Sync>;

/// A test application that simulates Octofer behavior for testing
pub struct TestApp {
    handlers: Arc<RwLock<HashMap<String, Vec<TestEventHandler>>>>,
    mock_client: Arc<MockGitHubClient>,
    config: Config,
}

impl TestApp {
    /// Create a new test application
    pub fn new() -> Self {
        Self {
            handlers: Arc::new(RwLock::new(HashMap::new())),
            mock_client: Arc::new(MockGitHubClient::new()),
            config: Config::default(),
        }
    }

    /// Create a new test application with custom configuration
    pub fn with_config(config: Config) -> Self {
        Self {
            handlers: Arc::new(RwLock::new(HashMap::new())),
            mock_client: Arc::new(MockGitHubClient::new()),
            config,
        }
    }

    /// Get the mock GitHub client for setting up responses and verifying calls
    pub fn mock_client(&self) -> Arc<MockGitHubClient> {
        self.mock_client.clone()
    }

    /// Register an event handler for issues events
    pub async fn on_issues<F, Fut>(&mut self, handler: F) -> &Self
    where
        F: Fn(Context) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Result<()>> + Send + 'static,
    {
        self.register_handler("issues", handler).await;
        self
    }

    /// Register an event handler for issue_comment events
    pub async fn on_issue_comment<F, Fut>(&mut self, handler: F) -> &Self
    where
        F: Fn(Context) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Result<()>> + Send + 'static,
    {
        self.register_handler("issue_comment", handler).await;
        self
    }

    /// Register an event handler for pull_request events
    pub async fn on_pull_request<F, Fut>(&mut self, handler: F) -> &Self
    where
        F: Fn(Context) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Result<()>> + Send + 'static,
    {
        self.register_handler("pull_request", handler).await;
        self
    }

    /// Register a generic event handler
    pub async fn on<F, Fut>(&mut self, event_type: &str, handler: F) -> &Self
    where
        F: Fn(Context) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Result<()>> + Send + 'static,
    {
        self.register_handler(event_type, handler).await;
        self
    }

    /// Handle a webhook event (simulate receiving a webhook)
    pub async fn handle_event(&self, event: WebhookEvent) -> Result<()> {
        let event_type = match &event.kind {
            WebhookEventType::Issues => "issues",
            WebhookEventType::IssueComment => "issue_comment",
            WebhookEventType::PullRequest => "pull_request",
            WebhookEventType::Push => "push",
            WebhookEventType::Release => "release",
            _ => "unknown",
        };

        self.handle_event_with_type(event_type, event).await
    }

    /// Handle a webhook event with a specific event type string
    pub async fn handle_event_with_type(&self, event_type: &str, event: WebhookEvent) -> Result<()> {
        let context = TestContext::with_event(event);
        self.execute_handlers(event_type, context).await
    }

    /// Handle an event using a pre-built context
    pub async fn handle_context(&self, event_type: &str, context: Context) -> Result<()> {
        self.execute_handlers(event_type, context).await
    }

    /// Execute all handlers for a specific event type
    async fn execute_handlers(&self, event_type: &str, context: Context) -> Result<()> {
        let handlers = {
            let handlers_lock = self.handlers.read().unwrap();
            handlers_lock.get(event_type).cloned().unwrap_or_default()
        };

        for handler in handlers {
            handler(context.clone()).await?;
        }

        Ok(())
    }

    /// Register a handler for a specific event type
    async fn register_handler<F, Fut>(&mut self, event_type: &str, handler: F)
    where
        F: Fn(Context) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Result<()>> + Send + 'static,
    {
        let handler_fn: TestEventHandler = Arc::new(move |context| {
            let fut = handler(context);
            Box::pin(fut)
        });

        let mut handlers = self.handlers.write().unwrap();
        handlers
            .entry(event_type.to_string())
            .or_insert_with(Vec::new)
            .push(handler_fn);
    }

    /// Get the application configuration
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// Clear all registered handlers
    pub fn clear_handlers(&mut self) {
        self.handlers.write().unwrap().clear();
    }

    /// Get the number of handlers registered for an event type
    pub fn handler_count(&self, event_type: &str) -> usize {
        self.handlers
            .read()
            .unwrap()
            .get(event_type)
            .map(|handlers| handlers.len())
            .unwrap_or(0)
    }

    /// Check if any handlers are registered for an event type
    pub fn has_handlers(&self, event_type: &str) -> bool {
        self.handler_count(event_type) > 0
    }
}

impl Default for TestApp {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for creating test scenarios
pub struct TestScenario {
    app: TestApp,
    events: Vec<WebhookEvent>,
}

impl TestScenario {
    /// Create a new test scenario
    pub fn new() -> Self {
        Self {
            app: TestApp::new(),
            events: Vec::new(),
        }
    }

    /// Add an event to the scenario
    pub fn with_event(mut self, event: WebhookEvent) -> Self {
        self.events.push(event);
        self
    }

    /// Add an issue event to the scenario
    pub fn with_issue_event(mut self, repo: &str, issue_number: u64) -> Self {
        let event = crate::testing::MockWebhookEvent::issue_opened(repo, issue_number).build();
        self.events.push(event);
        self
    }

    /// Configure the test app
    pub fn configure_app<F>(mut self, configure: F) -> Self
    where
        F: FnOnce(&mut TestApp),
    {
        configure(&mut self.app);
        self
    }

    /// Run the scenario and return the test app for inspection
    pub async fn run(self) -> Result<TestApp> {
        for event in self.events {
            self.app.handle_event(event).await?;
        }
        Ok(self.app)
    }
}

impl Default for TestScenario {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::MockWebhookEvent;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[tokio::test]
    async fn test_basic_event_handling() -> Result<()> {
        let mut app = TestApp::new();
        let call_count = Arc::new(AtomicUsize::new(0));
        
        let call_count_clone = call_count.clone();
        app.on_issues(move |_context| {
            let count = call_count_clone.clone();
            async move {
                count.fetch_add(1, Ordering::SeqCst);
                Ok(())
            }
        }).await;

        let event = MockWebhookEvent::issue_opened("test/repo", 42).build();
        app.handle_event(event).await?;

        assert_eq!(call_count.load(Ordering::SeqCst), 1);
        Ok(())
    }

    #[tokio::test]
    async fn test_multiple_handlers() -> Result<()> {
        let mut app = TestApp::new();
        let call_count = Arc::new(AtomicUsize::new(0));
        
        // Register multiple handlers for the same event
        let call_count_clone1 = call_count.clone();
        app.on_issues(move |_context| {
            let count = call_count_clone1.clone();
            async move {
                count.fetch_add(1, Ordering::SeqCst);
                Ok(())
            }
        }).await;

        let call_count_clone2 = call_count.clone();
        app.on_issues(move |_context| {
            let count = call_count_clone2.clone();
            async move {
                count.fetch_add(10, Ordering::SeqCst);
                Ok(())
            }
        }).await;

        let event = MockWebhookEvent::issue_opened("test/repo", 42).build();
        app.handle_event(event).await?;

        assert_eq!(call_count.load(Ordering::SeqCst), 11); // 1 + 10
        Ok(())
    }

    #[tokio::test]
    async fn test_handler_registration_counts() {
        let mut app = TestApp::new();
        
        assert_eq!(app.handler_count("issues"), 0);
        assert!(!app.has_handlers("issues"));

        app.on_issues(|_| async { Ok(()) }).await;
        assert_eq!(app.handler_count("issues"), 1);
        assert!(app.has_handlers("issues"));

        app.on_issues(|_| async { Ok(()) }).await;
        assert_eq!(app.handler_count("issues"), 2);
    }

    #[tokio::test]
    async fn test_scenario_builder() -> Result<()> {
        let call_count = Arc::new(AtomicUsize::new(0));

        let mut app = TestApp::new();
        let count = call_count.clone();
        app.on_issues(move |_context| {
            let count = count.clone();
            async move {
                count.fetch_add(1, Ordering::SeqCst);
                Ok(())
            }
        }).await;

        // Simulate processing events
        let event1 = MockWebhookEvent::issue_opened("test/repo", 42).build();
        let event2 = MockWebhookEvent::issue_opened("test/repo", 43).build();
        
        app.handle_event(event1).await?;
        app.handle_event(event2).await?;

        assert_eq!(call_count.load(Ordering::SeqCst), 2);
        Ok(())
    }
}