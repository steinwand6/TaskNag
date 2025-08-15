use crate::services::browser_action_service::BrowserActionService;
use crate::models::browser_action::BrowserAction;
use chrono::Utc;

#[tokio::test]
async fn test_manual_browser_action_execution() {
    println!("=== Manual Browser Action Test ===");
    
    let service = BrowserActionService::new();
    
    // Test URL validation first
    let test_url = "https://www.google.com";
    let validation_result = service.validate_url(test_url);
    println!("URL validation result for {}: {:?}", test_url, validation_result);
    
    if !validation_result.is_valid {
        println!("URL validation failed, skipping browser test");
        return;
    }
    
    // Create a test browser action
    let test_action = BrowserAction {
        id: "test-action-1".to_string(),
        label: "Test Google".to_string(),
        url: test_url.to_string(),
        enabled: true,
        order: 1,
        created_at: Utc::now(),
    };
    
    println!("Testing single browser action execution...");
    match service.execute_single_action(&test_action).await {
        Ok(_) => {
            println!("✅ SUCCESS: Browser action executed successfully!");
            println!("   - URL: {}", test_action.url);
            println!("   - Label: {}", test_action.label);
        }
        Err(e) => {
            println!("❌ ERROR: Failed to execute browser action: {}", e);
        }
    }
    
    // Test multiple actions
    let test_actions = vec![
        BrowserAction {
            id: "test-action-2".to_string(),
            label: "Test GitHub".to_string(),
            url: "https://github.com".to_string(),
            enabled: true,
            order: 1,
            created_at: Utc::now(),
        },
        BrowserAction {
            id: "test-action-3".to_string(),
            label: "Test Stack Overflow".to_string(),
            url: "https://stackoverflow.com".to_string(),
            enabled: true,
            order: 2,
            created_at: Utc::now(),
        },
    ];
    
    println!("\nTesting multiple browser actions execution...");
    match service.execute_actions(&test_actions).await {
        Ok(_) => {
            println!("✅ SUCCESS: Multiple browser actions executed successfully!");
            for action in &test_actions {
                println!("   - {}: {}", action.label, action.url);
            }
        }
        Err(e) => {
            println!("❌ ERROR: Failed to execute multiple browser actions: {}", e);
        }
    }
    
    // Test service availability
    println!("\nTesting service availability...");
    let is_available = service.is_available().await;
    println!("Browser action service available: {}", is_available);
}

#[tokio::test]
async fn test_url_test_command_simulation() {
    println!("=== URL Test Command Simulation ===");
    
    let service = BrowserActionService::new();
    let test_url = "https://www.google.com";
    
    println!("Testing URL: {}", test_url);
    
    match service.test_url(test_url).await {
        Ok(_) => {
            println!("✅ SUCCESS: URL test completed successfully!");
            println!("   Browser should have opened: {}", test_url);
        }
        Err(e) => {
            println!("❌ ERROR: URL test failed: {}", e);
        }
    }
}

#[tokio::test]
async fn test_dangerous_url_blocking() {
    println!("=== Dangerous URL Blocking Test ===");
    
    let service = BrowserActionService::new();
    let dangerous_urls = vec![
        "javascript:alert('xss')",
        "data:text/html,<script>alert('xss')</script>",
        "file:///etc/passwd",
    ];
    
    for url in dangerous_urls {
        println!("Testing dangerous URL: {}", url);
        match service.test_url(url).await {
            Ok(_) => {
                println!("❌ SECURITY ISSUE: Dangerous URL was allowed: {}", url);
            }
            Err(e) => {
                println!("✅ SECURITY OK: Dangerous URL blocked: {} - Error: {}", url, e);
            }
        }
    }
}