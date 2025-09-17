//! Mock GitHub client for testing
//!
//! This module provides a mock implementation of GitHub API interactions
//! that can be used in tests without making real network requests.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use serde_json::Value;

/// Mock GitHub client for testing
#[derive(Debug, Clone)]
pub struct MockGitHubClient {
    /// Recorded API calls for verification
    pub calls: Arc<Mutex<Vec<ApiCall>>>,
    /// Predefined responses for API calls
    pub responses: Arc<Mutex<HashMap<String, Value>>>,
}

/// Represents an API call made to the mock client
#[derive(Debug, Clone, PartialEq)]
pub struct ApiCall {
    pub method: String,
    pub path: String,
    pub body: Option<Value>,
}

impl MockGitHubClient {
    /// Create a new mock GitHub client
    pub fn new() -> Self {
        Self {
            calls: Arc::new(Mutex::new(Vec::new())),
            responses: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Record an API call
    pub fn record_call(&self, method: &str, path: &str, body: Option<Value>) {
        let call = ApiCall {
            method: method.to_string(),
            path: path.to_string(),
            body,
        };
        self.calls.lock().unwrap().push(call);
    }

    /// Set a mock response for a specific API call
    pub fn set_response(&self, key: &str, response: Value) {
        self.responses.lock().unwrap().insert(key.to_string(), response);
    }

    /// Get a mock response for a specific API call
    pub fn get_response(&self, key: &str) -> Option<Value> {
        self.responses.lock().unwrap().get(key).cloned()
    }

    /// Get all recorded API calls
    pub fn get_calls(&self) -> Vec<ApiCall> {
        self.calls.lock().unwrap().clone()
    }

    /// Clear all recorded calls
    pub fn clear_calls(&self) {
        self.calls.lock().unwrap().clear();
    }

    /// Check if a specific API call was made
    pub fn was_called(&self, method: &str, path: &str) -> bool {
        self.get_calls().iter().any(|call| {
            call.method == method && call.path == path
        })
    }

    /// Count how many times a specific API call was made
    pub fn call_count(&self, method: &str, path: &str) -> usize {
        self.get_calls().iter().filter(|call| {
            call.method == method && call.path == path
        }).count()
    }

    /// Get the last call made to the client
    pub fn last_call(&self) -> Option<ApiCall> {
        self.get_calls().last().cloned()
    }

    // Mock API methods that mirror common GitHub API operations

    /// Mock creating an issue comment
    pub async fn create_issue_comment(
        &self,
        repo: &str,
        issue_number: u64,
        body: &str,
    ) -> Result<Value, MockApiError> {
        let path = format!("/repos/{}/issues/{}/comments", repo, issue_number);
        let request_body = serde_json::json!({ "body": body });
        
        self.record_call("POST", &path, Some(request_body));

        // Return a mock response or predefined response
        let key = format!("POST:{}", path);
        if let Some(response) = self.get_response(&key) {
            Ok(response)
        } else {
            // Default mock response
            Ok(serde_json::json!({
                "id": 123456789,
                "body": body,
                "user": {
                    "login": "test-bot",
                    "id": 12345
                },
                "created_at": "2023-01-01T00:00:00Z",
                "updated_at": "2023-01-01T00:00:00Z"
            }))
        }
    }

    /// Mock updating an issue
    pub async fn update_issue(
        &self,
        repo: &str,
        issue_number: u64,
        title: Option<&str>,
        body: Option<&str>,
        state: Option<&str>,
    ) -> Result<Value, MockApiError> {
        let path = format!("/repos/{}/issues/{}", repo, issue_number);
        let mut request_body = serde_json::Map::new();
        
        if let Some(title) = title {
            request_body.insert("title".to_string(), Value::String(title.to_string()));
        }
        if let Some(body) = body {
            request_body.insert("body".to_string(), Value::String(body.to_string()));
        }
        if let Some(state) = state {
            request_body.insert("state".to_string(), Value::String(state.to_string()));
        }

        self.record_call("PATCH", &path, Some(Value::Object(request_body.clone())));

        let key = format!("PATCH:{}", path);
        if let Some(response) = self.get_response(&key) {
            Ok(response)
        } else {
            // Default mock response
            Ok(serde_json::json!({
                "number": issue_number,
                "title": title.unwrap_or("Mock Issue"),
                "body": body.unwrap_or("Mock body"),
                "state": state.unwrap_or("open"),
                "user": {
                    "login": "test-user",
                    "id": 67890
                }
            }))
        }
    }

    /// Mock adding labels to an issue
    pub async fn add_labels_to_issue(
        &self,
        repo: &str,
        issue_number: u64,
        labels: &[&str],
    ) -> Result<Value, MockApiError> {
        let path = format!("/repos/{}/issues/{}/labels", repo, issue_number);
        let request_body = serde_json::json!({ "labels": labels });
        
        self.record_call("POST", &path, Some(request_body));

        let key = format!("POST:{}", path);
        if let Some(response) = self.get_response(&key) {
            Ok(response)
        } else {
            // Default mock response
            let mock_labels: Vec<Value> = labels.iter().map(|label| {
                serde_json::json!({
                    "name": label,
                    "color": "ffffff",
                    "description": format!("Mock label: {}", label)
                })
            }).collect();
            
            Ok(Value::Array(mock_labels))
        }
    }

    /// Mock getting repository information
    pub async fn get_repository(&self, repo: &str) -> Result<Value, MockApiError> {
        let path = format!("/repos/{}", repo);
        self.record_call("GET", &path, None);

        let key = format!("GET:{}", path);
        if let Some(response) = self.get_response(&key) {
            Ok(response)
        } else {
            // Default mock response
            Ok(serde_json::json!({
                "name": repo.split('/').last().unwrap_or("unknown"),
                "full_name": repo,
                "private": false,
                "owner": {
                    "login": repo.split('/').next().unwrap_or("unknown"),
                    "id": 12345
                },
                "description": "Mock repository",
                "default_branch": "main"
            }))
        }
    }
}

impl Default for MockGitHubClient {
    fn default() -> Self {
        Self::new()
    }
}

/// Error type for mock API operations
#[derive(Debug, Clone)]
pub struct MockApiError {
    pub message: String,
    pub status: u16,
}

impl std::fmt::Display for MockApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Mock API Error ({}): {}", self.status, self.message)
    }
}

impl std::error::Error for MockApiError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_client_records_calls() {
        let client = MockGitHubClient::new();
        
        let _ = client.create_issue_comment("test/repo", 42, "Test comment").await;
        let _ = client.update_issue("test/repo", 42, Some("New title"), None, None).await;

        let calls = client.get_calls();
        assert_eq!(calls.len(), 2);
        
        assert_eq!(calls[0].method, "POST");
        assert_eq!(calls[0].path, "/repos/test/repo/issues/42/comments");
        
        assert_eq!(calls[1].method, "PATCH");
        assert_eq!(calls[1].path, "/repos/test/repo/issues/42");
    }

    #[tokio::test]
    async fn test_mock_client_custom_responses() {
        let client = MockGitHubClient::new();
        
        // Set a custom response
        let custom_response = serde_json::json!({
            "id": 999999,
            "body": "Custom response body"
        });
        client.set_response("POST:/repos/test/repo/issues/42/comments", custom_response.clone());

        let result = client.create_issue_comment("test/repo", 42, "Test comment").await.unwrap();
        assert_eq!(result, custom_response);
    }

    #[test]
    fn test_call_verification() {
        let client = MockGitHubClient::new();
        
        client.record_call("GET", "/test", None);
        client.record_call("POST", "/test", Some(serde_json::json!({"key": "value"})));
        client.record_call("GET", "/test", None);

        assert!(client.was_called("GET", "/test"));
        assert!(client.was_called("POST", "/test"));
        assert!(!client.was_called("DELETE", "/test"));
        
        assert_eq!(client.call_count("GET", "/test"), 2);
        assert_eq!(client.call_count("POST", "/test"), 1);
        assert_eq!(client.call_count("DELETE", "/test"), 0);
    }
}