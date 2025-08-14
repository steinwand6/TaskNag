use crate::models::browser_action::{BrowserAction, BrowserActionError};
use crate::services::url_validator::URLValidator;
use std::process::Command;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::timeout;
use std::pin::Pin;
use std::future::Future;

/// Trait for abstracting shell command execution (for testing)
pub trait ShellExecutor: Send + Sync {
    fn open_url(&self, url: &str) -> Pin<Box<dyn Future<Output = Result<(), BrowserActionError>> + Send + '_>>;
}

/// Real shell executor implementation
pub struct SystemShellExecutor;

impl ShellExecutor for SystemShellExecutor {
    fn open_url(&self, url: &str) -> Pin<Box<dyn Future<Output = Result<(), BrowserActionError>> + Send + '_>> {
        let url = url.to_string();
        Box::pin(async move {
            Self::open_url_impl(&url).await
        })
    }
}

impl SystemShellExecutor {
    async fn open_url_impl(url: &str) -> Result<(), BrowserActionError> {
        let result = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", "start", "", url])
                .spawn()
        } else if cfg!(target_os = "macos") {
            Command::new("open")
                .arg(url)
                .spawn()
        } else {
            Command::new("xdg-open")
                .arg(url)
                .spawn()
        };

        match result {
            Ok(mut child) => {
                // Don't wait for the browser to close, just ensure it started
                match child.try_wait() {
                    Ok(Some(status)) if !status.success() => {
                        Err(BrowserActionError::CommandFailed(
                            format!("Browser command failed with status: {}", status)
                        ))
                    }
                    Ok(Some(_)) => Ok(()), // Exited successfully
                    Ok(None) => Ok(()),    // Still running
                    Err(e) => Err(BrowserActionError::CommandFailed(
                        format!("Failed to check command status: {}", e)
                    ))
                }
            }
            Err(e) => Err(BrowserActionError::CommandFailed(
                format!("Failed to execute browser command: {}", e)
            ))
        }
    }
}

/// Browser action service for executing URL actions
pub struct BrowserActionService {
    shell: Arc<dyn ShellExecutor>,
    url_validator: URLValidator,
    timeout_duration: Duration,
}

impl BrowserActionService {
    pub fn new() -> Self {
        Self {
            shell: Arc::new(SystemShellExecutor),
            url_validator: URLValidator::new(),
            timeout_duration: Duration::from_secs(3),
        }
    }

    /// Create service with custom shell executor (for testing)
    pub fn with_shell(shell: Arc<dyn ShellExecutor>) -> Self {
        Self {
            shell,
            url_validator: URLValidator::new(),
            timeout_duration: Duration::from_secs(3),
        }
    }

    /// Execute multiple browser actions sequentially
    pub async fn execute_actions(&self, actions: &[BrowserAction]) -> Result<(), BrowserActionError> {
        if actions.is_empty() {
            log::debug!("No browser actions to execute");
            return Ok(());
        }

        log::info!("Executing {} browser actions", actions.len());

        for (index, action) in actions.iter().enumerate() {
            if !action.enabled {
                log::debug!("Skipping disabled action: {}", action.label);
                continue;
            }

            log::info!("Executing browser action {}/{}: {} -> {}", 
                index + 1, actions.len(), action.label, action.url);

            // Validate URL before opening
            let validation_result = self.url_validator.validate(&action.url);
            if !validation_result.is_valid {
                let error_msg = validation_result.error
                    .unwrap_or_else(|| "Unknown validation error".to_string());
                log::warn!("Skipping invalid URL {}: {}", action.url, error_msg);
                
                // Continue with next action instead of failing completely
                continue;
            }

            // Execute with timeout
            match self.open_url_with_timeout(&action.url).await {
                Ok(_) => {
                    log::info!("Successfully opened URL: {}", action.url);
                }
                Err(e) => {
                    log::warn!("Failed to open URL {}: {}. Continuing with remaining actions.", 
                        action.url, e);
                    // Continue with next URL instead of failing completely
                }
            }

            // Add delay between actions to avoid overwhelming the system
            if index < actions.len() - 1 {
                tokio::time::sleep(Duration::from_millis(500)).await;
            }
        }

        log::info!("Completed browser actions execution");
        Ok(())
    }

    /// Execute a single browser action
    pub async fn execute_single_action(&self, action: &BrowserAction) -> Result<(), BrowserActionError> {
        if !action.enabled {
            return Ok(());
        }

        // Validate URL
        let validation_result = self.url_validator.validate(&action.url);
        if !validation_result.is_valid {
            let error_msg = validation_result.error
                .unwrap_or_else(|| "Unknown validation error".to_string());
            return Err(BrowserActionError::SecurityViolation(error_msg));
        }

        self.open_url_with_timeout(&action.url).await
    }

    /// Test a URL by opening it immediately
    pub async fn test_url(&self, url: &str) -> Result<(), BrowserActionError> {
        // Validate first
        let validation_result = self.url_validator.validate(url);
        if !validation_result.is_valid {
            let error_msg = validation_result.error
                .unwrap_or_else(|| "Unknown validation error".to_string());
            return Err(BrowserActionError::SecurityViolation(error_msg));
        }

        self.open_url_with_timeout(url).await
    }

    /// Open URL with timeout protection
    async fn open_url_with_timeout(&self, url: &str) -> Result<(), BrowserActionError> {
        match timeout(self.timeout_duration, self.shell.open_url(url)).await {
            Ok(result) => result,
            Err(_) => Err(BrowserActionError::Timeout),
        }
    }

    /// Validate a URL using the internal validator
    pub fn validate_url(&self, url: &str) -> crate::models::browser_action::URLValidationResult {
        self.url_validator.validate(url)
    }

    /// Get URL suggestions for common mistakes
    pub fn get_url_suggestions(&self, url: &str) -> Vec<String> {
        self.url_validator.suggest_corrections(url)
    }

    /// Check if the browser action service is available
    pub async fn is_available(&self) -> bool {
        // Try to execute a safe test command
        let result = if cfg!(target_os = "windows") {
            Command::new("cmd").args(["/C", "echo test"]).output()
        } else {
            // Unix-like systems (macOS, Linux, etc.)
            Command::new("echo").arg("test").output()
        };
        
        match result {
            Ok(output) => output.status.success(),
            Err(_) => false,
        }
    }
}

impl Default for BrowserActionService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::browser_action::BrowserAction;
    use chrono::Utc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    // Mock shell executor for testing
    struct MockShellExecutor {
        call_count: AtomicUsize,
        should_fail: bool,
    }

    impl MockShellExecutor {
        fn new(should_fail: bool) -> Self {
            Self {
                call_count: AtomicUsize::new(0),
                should_fail,
            }
        }

        fn get_call_count(&self) -> usize {
            self.call_count.load(Ordering::SeqCst)
        }
    }

    impl ShellExecutor for MockShellExecutor {
        fn open_url(&self, _url: &str) -> Pin<Box<dyn Future<Output = Result<(), BrowserActionError>> + Send + '_>> {
            let should_fail = self.should_fail;
            let call_count = &self.call_count;
            Box::pin(async move {
                call_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                if should_fail {
                    Err(BrowserActionError::CommandFailed("Mock failure".to_string()))
                } else {
                    Ok(())
                }
            })
        }
    }


    #[tokio::test]
    async fn test_execute_single_action_success() {
        let mock_shell = Arc::new(MockShellExecutor::new(false));
        let service = BrowserActionService::with_shell(mock_shell.clone());

        let action = BrowserAction {
            id: "test".to_string(),
            label: "Test Action".to_string(),
            url: "https://www.google.com".to_string(),
            enabled: true,
            order: 1,
            created_at: Utc::now(),
        };

        let result = service.execute_single_action(&action).await;
        assert!(result.is_ok());
        assert_eq!(mock_shell.get_call_count(), 1);
    }

    #[tokio::test]
    async fn test_execute_disabled_action() {
        let mock_shell = Arc::new(MockShellExecutor::new(false));
        let service = BrowserActionService::with_shell(mock_shell.clone());

        let action = BrowserAction {
            id: "test".to_string(),
            label: "Test Action".to_string(),
            url: "https://www.google.com".to_string(),
            enabled: false,
            order: 1,
            created_at: Utc::now(),
        };

        let result = service.execute_single_action(&action).await;
        assert!(result.is_ok());
        assert_eq!(mock_shell.get_call_count(), 0); // Should not be called
    }

    #[tokio::test]
    async fn test_execute_invalid_url() {
        let mock_shell = Arc::new(MockShellExecutor::new(false));
        let service = BrowserActionService::with_shell(mock_shell.clone());

        let action = BrowserAction {
            id: "test".to_string(),
            label: "Test Action".to_string(),
            url: "javascript:alert('xss')".to_string(),
            enabled: true,
            order: 1,
            created_at: Utc::now(),
        };

        let result = service.execute_single_action(&action).await;
        assert!(result.is_err());
        assert_eq!(mock_shell.get_call_count(), 0); // Should not be called
    }

    #[tokio::test]
    async fn test_execute_multiple_actions() {
        let mock_shell = Arc::new(MockShellExecutor::new(false));
        let service = BrowserActionService::with_shell(mock_shell.clone());

        let actions = vec![
            BrowserAction {
                id: "test1".to_string(),
                label: "Test Action 1".to_string(),
                url: "https://www.google.com".to_string(),
                enabled: true,
                order: 1,
                created_at: Utc::now(),
            },
            BrowserAction {
                id: "test2".to_string(),
                label: "Test Action 2".to_string(),
                url: "https://www.github.com".to_string(),
                enabled: true,
                order: 2,
                created_at: Utc::now(),
            },
        ];

        let result = service.execute_actions(&actions).await;
        assert!(result.is_ok());
        assert_eq!(mock_shell.get_call_count(), 2);
    }

    #[tokio::test]
    async fn test_graceful_failure_handling() {
        let mock_shell = Arc::new(MockShellExecutor::new(true)); // Will fail
        let service = BrowserActionService::with_shell(mock_shell.clone());

        let actions = vec![
            BrowserAction {
                id: "test1".to_string(),
                label: "Test Action 1".to_string(),
                url: "https://www.google.com".to_string(),
                enabled: true,
                order: 1,
                created_at: Utc::now(),
            },
            BrowserAction {
                id: "test2".to_string(),
                label: "Test Action 2".to_string(),
                url: "https://www.github.com".to_string(),
                enabled: true,
                order: 2,
                created_at: Utc::now(),
            },
        ];

        // Should not fail even if individual actions fail
        let result = service.execute_actions(&actions).await;
        assert!(result.is_ok());
        assert_eq!(mock_shell.get_call_count(), 2); // Both should be attempted
    }

    #[test]
    fn test_url_validation() {
        let service = BrowserActionService::new();

        let valid_url = "https://www.google.com";
        let result = service.validate_url(valid_url);
        assert!(result.is_valid);

        let invalid_url = "javascript:alert('xss')";
        let result = service.validate_url(invalid_url);
        assert!(!result.is_valid);
    }

    #[test]
    fn test_url_suggestions() {
        let service = BrowserActionService::new();

        let suggestions = service.get_url_suggestions("google");
        assert!(!suggestions.is_empty());
        assert!(suggestions.contains(&"https://google".to_string()));
    }
}