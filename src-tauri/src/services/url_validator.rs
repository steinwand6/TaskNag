use crate::models::browser_action::URLValidationResult;
use std::collections::HashSet;
use url::Url;
use regex::Regex;

/// URL validation service with security checks
pub struct URLValidator {
    allowed_protocols: HashSet<String>,
    blocked_protocols: HashSet<String>,
    max_length: usize,
    blocked_patterns: Vec<Regex>,
}

impl URLValidator {
    pub fn new() -> Self {
        let mut blocked_patterns = Vec::new();
        
        // Compile regex patterns for dangerous content
        if let Ok(js_pattern) = Regex::new(r"(?i)javascript:") {
            blocked_patterns.push(js_pattern);
        }
        if let Ok(data_pattern) = Regex::new(r"(?i)data:") {
            blocked_patterns.push(data_pattern);
        }
        if let Ok(vb_pattern) = Regex::new(r"(?i)vbscript:") {
            blocked_patterns.push(vb_pattern);
        }
        if let Ok(script_pattern) = Regex::new(r"(?i)<script") {
            blocked_patterns.push(script_pattern);
        }

        Self {
            allowed_protocols: ["http", "https"].iter().map(|s| s.to_string()).collect(),
            blocked_protocols: ["javascript", "data", "file", "ftp", "vbscript"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            max_length: 2048,
            blocked_patterns,
        }
    }

    /// Validate a URL with comprehensive security checks
    pub fn validate(&self, url_str: &str) -> URLValidationResult {
        // Length check
        if url_str.len() > self.max_length {
            return URLValidationResult::invalid(
                format!("URL too long: {} characters (max: {})", url_str.len(), self.max_length)
            );
        }

        // Pattern-based security checks
        for pattern in &self.blocked_patterns {
            if pattern.is_match(url_str) {
                return URLValidationResult::invalid(
                    "URL contains dangerous patterns".to_string()
                );
            }
        }

        // Parse URL
        let url = match self.parse_url_with_protocol(url_str) {
            Ok(url) => url,
            Err(err) => {
                return URLValidationResult::invalid(format!("Invalid URL format: {}", err));
            }
        };

        // Protocol validation
        let scheme = url.scheme().to_lowercase();
        if self.blocked_protocols.contains(&scheme) {
            return URLValidationResult::invalid(
                format!("Blocked protocol: {}", scheme)
            );
        }

        if !self.allowed_protocols.contains(&scheme) {
            return URLValidationResult::invalid(
                format!("Protocol not allowed: {}", scheme)
            );
        }

        // Host validation
        let host = match url.host_str() {
            Some(host) => host,
            None => {
                return URLValidationResult::invalid("No host found in URL".to_string());
            }
        };

        if !self.is_valid_host(host) {
            return URLValidationResult::invalid(
                format!("Invalid host format: {}", host)
            );
        }

        URLValidationResult::valid(scheme, host.to_string())
    }

    /// Parse URL and add https:// if no protocol is specified
    fn parse_url_with_protocol(&self, url_str: &str) -> Result<Url, url::ParseError> {
        // Try parsing as-is first
        if let Ok(url) = Url::parse(url_str) {
            return Ok(url);
        }

        // If no protocol, try adding https://
        if !url_str.contains("://") {
            let with_https = format!("https://{}", url_str);
            return Url::parse(&with_https);
        }

        // If it contains :// but still fails, return the original error
        Url::parse(url_str)
    }

    /// Validate host format
    fn is_valid_host(&self, host: &str) -> bool {
        // Basic host validation (simplified)
        if host.is_empty() {
            return false;
        }

        // Check for localhost or IP patterns (basic check)
        if host == "localhost" || host.starts_with("127.") || host.starts_with("192.168.") {
            return true; // Allow localhost for development
        }

        // Must contain at least one dot for domain names
        if !host.contains('.') {
            return false;
        }

        // Check for valid domain format (basic regex)
        let domain_regex = match Regex::new(r"^[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$") {
            Ok(regex) => regex,
            Err(_) => return false,
        };

        domain_regex.is_match(host)
    }

    /// Quick validation for UI feedback
    pub fn quick_validate(&self, url_str: &str) -> bool {
        self.validate(url_str).is_valid
    }

    /// Suggest corrections for common URL mistakes
    pub fn suggest_corrections(&self, url_str: &str) -> Vec<String> {
        let mut suggestions = Vec::new();

        // If no protocol, suggest adding https://
        if !url_str.contains("://") && !url_str.is_empty() {
            suggestions.push(format!("https://{}", url_str));
        }

        // If starts with http://, suggest https://
        if url_str.starts_with("http://") {
            suggestions.push(url_str.replace("http://", "https://"));
        }

        // Common domain corrections
        if url_str.contains("google") && !url_str.contains("google.com") {
            suggestions.push(url_str.replace("google", "google.com"));
        }

        suggestions
    }
}

impl Default for URLValidator {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait for testability
pub trait URLValidatorTrait {
    fn validate(&self, url: &str) -> URLValidationResult;
    fn quick_validate(&self, url: &str) -> bool;
}

impl URLValidatorTrait for URLValidator {
    fn validate(&self, url: &str) -> URLValidationResult {
        self.validate(url)
    }

    fn quick_validate(&self, url: &str) -> bool {
        self.quick_validate(url)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_urls() {
        let validator = URLValidator::new();

        let valid_urls = vec![
            "https://www.google.com",
            "http://example.com",
            "https://docs.google.com/document/d/abc123",
            "https://github.com/user/repo",
            "http://localhost:3000",
        ];

        for url in valid_urls {
            let result = validator.validate(url);
            assert!(result.is_valid, "Should be valid: {}", url);
        }
    }

    #[test]
    fn test_invalid_urls() {
        let validator = URLValidator::new();

        let invalid_urls = vec![
            "javascript:alert('xss')",
            "data:text/html,<script>alert('xss')</script>",
            "file:///etc/passwd",
            "ftp://example.com",
            "vbscript:alert('xss')",
        ];

        for url in invalid_urls {
            let result = validator.validate(url);
            assert!(!result.is_valid, "Should be invalid: {}", url);
        }
    }

    #[test]
    fn test_auto_protocol_addition() {
        let validator = URLValidator::new();

        let result = validator.validate("www.google.com");
        assert!(result.is_valid);
        assert_eq!(result.protocol, "https");
    }

    #[test]
    fn test_length_limit() {
        let validator = URLValidator::new();

        let long_url = format!("https://example.com/{}", "a".repeat(2048));
        let result = validator.validate(&long_url);
        assert!(!result.is_valid);
        assert!(result.error.unwrap().contains("too long"));
    }

    #[test]
    fn test_suggestions() {
        let validator = URLValidator::new();

        let suggestions = validator.suggest_corrections("google");
        assert!(suggestions.contains(&"https://google".to_string()));
        
        let suggestions = validator.suggest_corrections("http://example.com");
        assert!(suggestions.contains(&"https://example.com".to_string()));
    }

    #[test]
    fn test_dangerous_patterns() {
        let validator = URLValidator::new();

        let dangerous_urls = vec![
            "https://example.com?redirect=javascript:alert('xss')",
            "https://example.com<script>alert('xss')</script>",
        ];

        for url in dangerous_urls {
            let result = validator.validate(url);
            assert!(!result.is_valid, "Should detect dangerous pattern: {}", url);
        }
    }
}