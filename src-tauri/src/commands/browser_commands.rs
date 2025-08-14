use crate::models::browser_action::{BrowserAction, URLValidationResult};
use crate::services::{BrowserActionService, URLValidator};
use tauri::State;
use std::sync::Arc;

#[tauri::command]
pub async fn validate_url_command(url: String) -> Result<URLValidationResult, String> {
    let validator = URLValidator::new();
    Ok(validator.validate(&url))
}

#[tauri::command]
pub async fn test_browser_action_command(
    browser_action_service: State<'_, Arc<BrowserActionService>>
) -> Result<bool, String> {
    match browser_action_service.is_available().await {
        true => Ok(true),
        false => Err("Browser action service is not available".to_string()),
    }
}

#[tauri::command]
pub async fn execute_browser_action_command(
    action: BrowserAction,
    browser_action_service: State<'_, Arc<BrowserActionService>>
) -> Result<(), String> {
    browser_action_service
        .execute_single_action(&action)
        .await
        .map_err(|e| format!("Failed to execute browser action: {}", e))
}

#[tauri::command]
pub async fn execute_browser_actions_command(
    actions: Vec<BrowserAction>,
    browser_action_service: State<'_, Arc<BrowserActionService>>
) -> Result<(), String> {
    browser_action_service
        .execute_actions(&actions)
        .await
        .map_err(|e| format!("Failed to execute browser actions: {}", e))
}

#[tauri::command]
pub async fn test_url_command(
    url: String,
    browser_action_service: State<'_, Arc<BrowserActionService>>
) -> Result<(), String> {
    browser_action_service
        .test_url(&url)
        .await
        .map_err(|e| format!("Failed to test URL: {}", e))
}

#[tauri::command]
pub async fn get_url_suggestions_command(url: String) -> Result<Vec<String>, String> {
    let validator = URLValidator::new();
    Ok(validator.suggest_corrections(&url))
}

#[tauri::command]
pub async fn get_url_preview_command(url: String) -> Result<URLPreview, String> {
    // Basic URL preview implementation
    let validator = URLValidator::new();
    let validation_result = validator.validate(&url);
    
    if !validation_result.is_valid {
        return Err(validation_result.error.unwrap_or("Invalid URL".to_string()));
    }
    
    // For now, return basic information
    // In future, this could fetch favicon, page title, etc.
    let domain = extract_domain(&url).unwrap_or_else(|| "Unknown".to_string());
    
    Ok(URLPreview {
        url: url.clone(),
        title: format!("Open {}", domain),
        domain,
        favicon_url: None,
        description: None,
    })
}

#[derive(serde::Serialize)]
pub struct URLPreview {
    pub url: String,
    pub title: String,
    pub domain: String,
    pub favicon_url: Option<String>,
    pub description: Option<String>,
}

fn extract_domain(url: &str) -> Option<String> {
    if let Ok(parsed) = url::Url::parse(url) {
        parsed.host_str().map(|s| s.to_string())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_domain() {
        assert_eq!(extract_domain("https://www.google.com/search"), Some("www.google.com".to_string()));
        assert_eq!(extract_domain("http://github.com"), Some("github.com".to_string()));
        assert_eq!(extract_domain("invalid-url"), None);
    }

    #[tokio::test]
    async fn test_validate_url_command() {
        let result = validate_url_command("https://www.google.com".to_string()).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_valid);

        let result = validate_url_command("javascript:alert(1)".to_string()).await;
        assert!(result.is_ok());
        assert!(!result.unwrap().is_valid);
    }

    #[tokio::test]
    async fn test_get_url_preview_command() {
        let result = get_url_preview_command("https://www.google.com".to_string()).await;
        assert!(result.is_ok());
        let preview = result.unwrap();
        assert_eq!(preview.domain, "www.google.com");
        assert_eq!(preview.url, "https://www.google.com");

        let result = get_url_preview_command("invalid-url".to_string()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_url_suggestions_command() {
        let result = get_url_suggestions_command("google".to_string()).await;
        assert!(result.is_ok());
        let suggestions = result.unwrap();
        assert!(!suggestions.is_empty());
        assert!(suggestions.contains(&"https://google".to_string()));
    }
}