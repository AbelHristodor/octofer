//! GitHub authentication and configuration

use crate::config::GitHubConfig;

/// GitHub authentication configuration
#[derive(Debug, Clone)]
pub struct GitHubAuth {
    /// GitHub App ID
    pub app_id: u64,
    /// Private key for JWT signing
    pub private_key: Vec<u8>,
}

impl GitHubAuth {
    /// Create authentication from GitHubConfig
    pub fn from_config(config: &GitHubConfig) -> Self {
        Self {
            app_id: config.app_id,
            private_key: config.private_key.clone(),
        }
    }

    /// Get the App ID
    pub fn app_id(&self) -> u64 {
        self.app_id
    }

    /// Get the private key
    pub fn private_key(&self) -> &[u8] {
        &self.private_key
    }
}

/// Parse a UTC datetime string
pub fn parse_to_utc(date_str: &str) -> chrono::DateTime<chrono::Utc> {
    date_str
        .parse::<chrono::DateTime<chrono::Utc>>()
        .expect("Invalid date format")
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Datelike, Timelike};

    #[test]
    fn test_parse_to_utc_valid() {
        let datetime_str = "2025-07-10T09:14:47Z";
        let result = parse_to_utc(datetime_str);
        assert_eq!(result.year(), 2025);
        assert_eq!(result.month(), 7);
        assert_eq!(result.day(), 10);
        assert_eq!(result.hour(), 9);
        assert_eq!(result.minute(), 14);
        assert_eq!(result.second(), 47);
    }

    #[test]
    fn test_parse_to_utc_with_microseconds() {
        let datetime_str = "2025-07-10T09:14:47.123456Z";
        let _ = parse_to_utc(datetime_str);
    }

    #[test]
    #[should_panic]
    fn test_parse_to_utc_invalid() {
        let datetime_str = "invalid-datetime";
        let _ = parse_to_utc(datetime_str);
    }

    #[test]
    fn test_parse_to_utc_with_timezone_offset() {
        let datetime_str = "2025-07-10T09:14:47+02:00";
        let result = parse_to_utc(datetime_str);
        assert_eq!(result.hour(), 7); // 9 - 2 = 7 UTC
    }
}
