use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Individual browser action configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserAction {
    pub id: String,
    pub label: String,
    pub url: String,
    pub enabled: bool,
    pub order: i32,
    pub created_at: DateTime<Utc>,
}

impl BrowserAction {
    pub fn new(label: String, url: String, order: i32) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            label,
            url,
            enabled: true,
            order,
            created_at: Utc::now(),
        }
    }
}

/// Browser action settings for a task
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BrowserActionSettings {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub actions: Vec<BrowserAction>,
}

impl BrowserActionSettings {
    pub fn new(enabled: bool) -> Self {
        Self {
            enabled,
            actions: Vec::new(),
        }
    }

    pub fn add_action(&mut self, action: BrowserAction) {
        // Ensure maximum 5 actions
        if self.actions.len() < 5 {
            self.actions.push(action);
            // Sort by order
            self.actions.sort_by_key(|a| a.order);
        }
    }

    pub fn remove_action(&mut self, action_id: &str) {
        self.actions.retain(|a| a.id != action_id);
    }

    pub fn get_enabled_actions(&self) -> Vec<&BrowserAction> {
        if !self.enabled {
            return Vec::new();
        }
        
        self.actions
            .iter()
            .filter(|action| action.enabled)
            .collect()
    }

    pub fn reorder_actions(&mut self, action_id: &str, new_order: i32) {
        if let Some(action) = self.actions.iter_mut().find(|a| a.id == action_id) {
            action.order = new_order;
            self.actions.sort_by_key(|a| a.order);
        }
    }
}

/// Browser action execution errors
#[derive(Debug, Clone)]
pub enum BrowserActionError {
    InvalidUrl(String),
    CommandFailed(String),
    Timeout,
    SecurityViolation(String),
    ServiceUnavailable,
}

impl fmt::Display for BrowserActionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BrowserActionError::InvalidUrl(url) => {
                write!(f, "Invalid URL: {}", url)
            }
            BrowserActionError::CommandFailed(cmd) => {
                write!(f, "Browser command failed: {}", cmd)
            }
            BrowserActionError::Timeout => {
                write!(f, "Browser action timed out")
            }
            BrowserActionError::SecurityViolation(msg) => {
                write!(f, "Security violation: {}", msg)
            }
            BrowserActionError::ServiceUnavailable => {
                write!(f, "Browser service unavailable")
            }
        }
    }
}

impl std::error::Error for BrowserActionError {}

/// URL validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct URLValidationResult {
    pub is_valid: bool,
    pub protocol: String, // 'http', 'https', 'invalid'
    pub host: String,
    pub error: Option<String>,
}

impl URLValidationResult {
    pub fn valid(protocol: String, host: String) -> Self {
        Self {
            is_valid: true,
            protocol,
            host,
            error: None,
        }
    }

    pub fn invalid(error: String) -> Self {
        Self {
            is_valid: false,
            protocol: "invalid".to_string(),
            host: String::new(),
            error: Some(error),
        }
    }
}

/// URL preview information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct URLPreviewInfo {
    pub title: Option<String>,
    pub favicon: Option<String>,
    pub description: Option<String>,
    pub status: String, // 'success', 'error', 'loading'
}

impl URLPreviewInfo {
    pub fn loading() -> Self {
        Self {
            title: None,
            favicon: None,
            description: None,
            status: "loading".to_string(),
        }
    }

    pub fn success(title: Option<String>, favicon: Option<String>, description: Option<String>) -> Self {
        Self {
            title,
            favicon,
            description,
            status: "success".to_string(),
        }
    }

    pub fn error() -> Self {
        Self {
            title: None,
            favicon: None,
            description: None,
            status: "error".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_browser_action_creation() {
        let action = BrowserAction::new(
            "Test Action".to_string(),
            "https://example.com".to_string(),
            1
        );
        
        assert_eq!(action.label, "Test Action");
        assert_eq!(action.url, "https://example.com");
        assert_eq!(action.order, 1);
        assert!(action.enabled);
        assert!(!action.id.is_empty());
    }

    #[test]
    fn test_browser_action_settings() {
        let mut settings = BrowserActionSettings::new(true);
        assert!(settings.enabled);
        assert!(settings.actions.is_empty());

        let action = BrowserAction::new(
            "Test".to_string(),
            "https://test.com".to_string(),
            1
        );
        
        settings.add_action(action);
        assert_eq!(settings.actions.len(), 1);
        assert_eq!(settings.get_enabled_actions().len(), 1);

        settings.enabled = false;
        assert_eq!(settings.get_enabled_actions().len(), 0);
    }

    #[test]
    fn test_max_actions_limit() {
        let mut settings = BrowserActionSettings::new(true);
        
        // Add 6 actions, should only accept 5
        for i in 1..=6 {
            let action = BrowserAction::new(
                format!("Action {}", i),
                format!("https://example{}.com", i),
                i
            );
            settings.add_action(action);
        }
        
        assert_eq!(settings.actions.len(), 5);
    }
}