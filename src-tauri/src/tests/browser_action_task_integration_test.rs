use crate::database::Database;
use crate::services::TaskService;
use crate::models::{CreateTaskRequest, UpdateTaskRequest, TaskStatus, TaskNotificationSettings};
use crate::models::browser_action::{BrowserAction, BrowserActionSettings};
use chrono::Utc;
use tempfile::tempdir;

#[tokio::test]
async fn test_create_task_with_browser_actions() {
    println!("=== Create Task with Browser Actions Test ===");
    
    // Setup test database
    let temp_dir = tempdir().unwrap();
    let db_path = temp_dir.path().join("test_browser_actions.db");
    let db_url = format!("sqlite:{}?mode=rwc", db_path.display());
    
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(1)
        .connect(&db_url)
        .await
        .unwrap();
    
    // Run migrations
    crate::database::migrations::run_migrations(&pool).await.unwrap();
    
    let db = Database { pool };
    let task_service = TaskService::new(db);
    
    // Create browser actions
    let browser_actions = vec![
        BrowserAction {
            id: "action-1".to_string(),
            label: "Google Search".to_string(),
            url: "https://www.google.com/search?q=rust".to_string(),
            enabled: true,
            order: 1,
            created_at: Utc::now(),
        },
        BrowserAction {
            id: "action-2".to_string(),
            label: "GitHub Repo".to_string(),
            url: "https://github.com/tauri-apps/tauri".to_string(),
            enabled: true,
            order: 2,
            created_at: Utc::now(),
        },
    ];
    
    let browser_action_settings = BrowserActionSettings {
        enabled: true,
        actions: browser_actions.clone(),
    };
    
    // Create task with browser actions
    let create_request = CreateTaskRequest {
        title: "Test Task with Browser Actions".to_string(),
        description: Some("This task has browser actions".to_string()),
        status: TaskStatus::Todo,
        parent_id: None,
        due_date: None,
        notification_settings: Some(TaskNotificationSettings {
            notification_type: "due_date_based".to_string(),
            days_before: Some(1),
            notification_time: Some("09:00".to_string()),
            days_of_week: None,
            level: 2,
        }),
        browser_actions: Some(browser_action_settings),
    };
    
    println!("Creating task with browser actions...");
    match task_service.create_task(create_request).await {
        Ok(created_task) => {
            println!("✅ SUCCESS: Task created with ID: {}", created_task.id);
            println!("   - Title: {}", created_task.title);
            
            // Verify browser actions were saved
            if let Some(browser_actions_json) = &created_task.browser_actions {
                println!("   - Browser Actions JSON: {}", browser_actions_json);
                
                // Parse the saved browser actions
                match serde_json::from_str::<BrowserActionSettings>(browser_actions_json) {
                    Ok(saved_settings) => {
                        println!("✅ SUCCESS: Browser actions parsed successfully");
                        println!("   - Enabled: {}", saved_settings.enabled);
                        println!("   - Actions count: {}", saved_settings.actions.len());
                        
                        for (i, action) in saved_settings.actions.iter().enumerate() {
                            println!("   - Action {}: {} -> {}", i + 1, action.label, action.url);
                        }
                        
                        // Verify the actions match what we saved
                        assert_eq!(saved_settings.enabled, true);
                        assert_eq!(saved_settings.actions.len(), 2);
                        assert_eq!(saved_settings.actions[0].label, "Google Search");
                        assert_eq!(saved_settings.actions[1].label, "GitHub Repo");
                        
                        println!("✅ SUCCESS: All browser action data verified correctly");
                    }
                    Err(e) => {
                        println!("❌ ERROR: Failed to parse saved browser actions: {}", e);
                        panic!("Browser actions parsing failed");
                    }
                }
            } else {
                println!("❌ ERROR: No browser actions found in saved task");
                panic!("Browser actions not saved");
            }
            
            // Test retrieval by ID
            println!("\nTesting task retrieval by ID...");
            match task_service.get_task_by_id(&created_task.id).await {
                Ok(retrieved_task) => {
                    println!("✅ SUCCESS: Task retrieved by ID");
                    
                    if let Some(browser_actions_json) = &retrieved_task.browser_actions {
                        match serde_json::from_str::<BrowserActionSettings>(browser_actions_json) {
                            Ok(retrieved_settings) => {
                                println!("✅ SUCCESS: Retrieved browser actions are valid");
                                assert_eq!(retrieved_settings.actions.len(), 2);
                            }
                            Err(e) => {
                                println!("❌ ERROR: Retrieved browser actions are invalid: {}", e);
                                panic!("Retrieved browser actions invalid");
                            }
                        }
                    } else {
                        println!("❌ ERROR: No browser actions in retrieved task");
                        panic!("Browser actions lost on retrieval");
                    }
                }
                Err(e) => {
                    println!("❌ ERROR: Failed to retrieve task: {}", e);
                    panic!("Task retrieval failed");
                }
            }
        }
        Err(e) => {
            println!("❌ ERROR: Failed to create task: {}", e);
            panic!("Task creation failed");
        }
    }
}

#[tokio::test]
async fn test_update_task_with_browser_actions() {
    println!("=== Update Task with Browser Actions Test ===");
    
    // Setup test database
    let temp_dir = tempdir().unwrap();
    let db_path = temp_dir.path().join("test_update_browser_actions.db");
    let db_url = format!("sqlite:{}?mode=rwc", db_path.display());
    
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(1)
        .connect(&db_url)
        .await
        .unwrap();
    
    // Run migrations
    crate::database::migrations::run_migrations(&pool).await.unwrap();
    
    let db = Database { pool };
    let task_service = TaskService::new(db);
    
    // Create a basic task first
    let create_request = CreateTaskRequest {
        title: "Task to Update".to_string(),
        description: Some("Will add browser actions later".to_string()),
        status: TaskStatus::Todo,
        parent_id: None,
        due_date: None,
        notification_settings: None,
        browser_actions: None,
    };
    
    println!("Creating initial task...");
    let created_task = task_service.create_task(create_request).await.unwrap();
    println!("✅ Initial task created with ID: {}", created_task.id);
    
    // Verify no browser actions initially
    assert!(created_task.browser_actions.is_none());
    println!("✅ Confirmed: No browser actions initially");
    
    // Now update the task with browser actions
    let new_browser_actions = vec![
        BrowserAction {
            id: "update-action-1".to_string(),
            label: "Stack Overflow".to_string(),
            url: "https://stackoverflow.com/questions/tagged/rust".to_string(),
            enabled: true,
            order: 1,
            created_at: Utc::now(),
        },
        BrowserAction {
            id: "update-action-2".to_string(),
            label: "Rust Documentation".to_string(),
            url: "https://doc.rust-lang.org/".to_string(),
            enabled: true,
            order: 2,
            created_at: Utc::now(),
        },
        BrowserAction {
            id: "update-action-3".to_string(),
            label: "Disabled Action".to_string(),
            url: "https://example.com".to_string(),
            enabled: false,
            order: 3,
            created_at: Utc::now(),
        },
    ];
    
    let update_browser_settings = BrowserActionSettings {
        enabled: true,
        actions: new_browser_actions.clone(),
    };
    
    let update_request = UpdateTaskRequest {
        title: Some("Updated Task with Browser Actions".to_string()),
        description: None,
        status: None,
        parent_id: None,
        due_date: None,
        notification_settings: Some(TaskNotificationSettings {
            notification_type: "recurring".to_string(),
            days_before: None,
            notification_time: Some("10:30".to_string()),
            days_of_week: Some(vec![1, 3, 5]), // Mon, Wed, Fri
            level: 3,
        }),
        browser_actions: Some(update_browser_settings),
        tags: None,
    };
    
    println!("Updating task with browser actions...");
    match task_service.update_task(&created_task.id, update_request).await {
        Ok(updated_task) => {
            println!("✅ SUCCESS: Task updated with ID: {}", updated_task.id);
            println!("   - Title: {}", updated_task.title);
            
            // Verify browser actions were saved
            if let Some(browser_actions_json) = &updated_task.browser_actions {
                println!("   - Browser Actions JSON: {}", browser_actions_json);
                
                match serde_json::from_str::<BrowserActionSettings>(browser_actions_json) {
                    Ok(saved_settings) => {
                        println!("✅ SUCCESS: Updated browser actions parsed successfully");
                        println!("   - Enabled: {}", saved_settings.enabled);
                        println!("   - Actions count: {}", saved_settings.actions.len());
                        
                        assert_eq!(saved_settings.enabled, true);
                        assert_eq!(saved_settings.actions.len(), 3);
                        
                        // Check each action
                        let action1 = &saved_settings.actions[0];
                        let action2 = &saved_settings.actions[1];
                        let action3 = &saved_settings.actions[2];
                        
                        assert_eq!(action1.label, "Stack Overflow");
                        assert_eq!(action1.enabled, true);
                        
                        assert_eq!(action2.label, "Rust Documentation");
                        assert_eq!(action2.enabled, true);
                        
                        assert_eq!(action3.label, "Disabled Action");
                        assert_eq!(action3.enabled, false);
                        
                        println!("✅ SUCCESS: All updated browser action data verified correctly");
                        
                        for (i, action) in saved_settings.actions.iter().enumerate() {
                            println!("   - Action {}: {} -> {} (enabled: {})", 
                                i + 1, action.label, action.url, action.enabled);
                        }
                    }
                    Err(e) => {
                        println!("❌ ERROR: Failed to parse updated browser actions: {}", e);
                        panic!("Updated browser actions parsing failed");
                    }
                }
            } else {
                println!("❌ ERROR: No browser actions found in updated task");
                panic!("Browser actions not saved during update");
            }
        }
        Err(e) => {
            println!("❌ ERROR: Failed to update task: {}", e);
            panic!("Task update failed");
        }
    }
}

#[tokio::test]
async fn test_task_list_includes_browser_actions() {
    println!("=== Task List Browser Actions Test ===");
    
    // Setup test database
    let temp_dir = tempdir().unwrap();
    let db_path = temp_dir.path().join("test_list_browser_actions.db");
    let db_url = format!("sqlite:{}?mode=rwc", db_path.display());
    
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(1)
        .connect(&db_url)
        .await
        .unwrap();
    
    // Run migrations
    crate::database::migrations::run_migrations(&pool).await.unwrap();
    
    let db = Database { pool };
    let task_service = TaskService::new(db);
    
    // Create multiple tasks with different browser action configurations
    let tasks_to_create = vec![
        ("Task with Browser Actions", true),
        ("Task without Browser Actions", false),
        ("Task with Disabled Browser Actions", true),
    ];
    
    let mut task_ids = Vec::new();
    
    for (i, (title, has_browser_actions)) in tasks_to_create.iter().enumerate() {
        let browser_actions = if *has_browser_actions {
            Some(BrowserActionSettings {
                enabled: i != 2, // Third task has disabled actions
                actions: vec![
                    BrowserAction {
                        id: format!("list-action-{}", i),
                        label: format!("Action for {}", title),
                        url: format!("https://example{}.com", i),
                        enabled: true,
                        order: 1,
                        created_at: Utc::now(),
                    },
                ],
            })
        } else {
            None
        };
        
        let create_request = CreateTaskRequest {
            title: title.to_string(),
            description: Some(format!("Description for {}", title)),
            status: TaskStatus::Todo,
            parent_id: None,
            due_date: None,
            notification_settings: None,
            browser_actions,
            };
        
        let created_task = task_service.create_task(create_request).await.unwrap();
        let task_id = created_task.id.clone();
        task_ids.push(created_task.id);
        println!("Created task: {} with ID: {}", title, task_id);
    }
    
    // Test get_all_tasks
    println!("\nTesting get_all_tasks...");
    match task_service.get_tasks().await {
        Ok(all_tasks) => {
            println!("✅ SUCCESS: Retrieved {} tasks", all_tasks.len());
            
            for task in &all_tasks {
                println!("Task: {} (ID: {})", task.title, task.id);
                
                if let Some(browser_actions_json) = &task.browser_actions {
                    match serde_json::from_str::<BrowserActionSettings>(browser_actions_json) {
                        Ok(settings) => {
                            println!("   - Has browser actions: enabled={}, count={}", 
                                settings.enabled, settings.actions.len());
                        }
                        Err(e) => {
                            println!("   - Invalid browser actions JSON: {}", e);
                        }
                    }
                } else {
                    println!("   - No browser actions");
                }
            }
            
            // Verify correct number of tasks
            assert_eq!(all_tasks.len(), 3);
            println!("✅ SUCCESS: All tasks retrieved with browser actions data");
        }
        Err(e) => {
            println!("❌ ERROR: Failed to get all tasks: {}", e);
            panic!("get_all_tasks failed");
        }
    }
    
    // Test get_tasks_by_status
    println!("\nTesting get_tasks_by_status...");
    match task_service.get_tasks_by_status("todo").await {
        Ok(todo_tasks) => {
            println!("✅ SUCCESS: Retrieved {} todo tasks", todo_tasks.len());
            
            for task in &todo_tasks {
                if let Some(browser_actions_json) = &task.browser_actions {
                    match serde_json::from_str::<BrowserActionSettings>(browser_actions_json) {
                        Ok(_) => {
                            println!("   - Task '{}' has valid browser actions", task.title);
                        }
                        Err(e) => {
                            println!("   - Task '{}' has invalid browser actions: {}", task.title, e);
                            panic!("Invalid browser actions in task list");
                        }
                    }
                } else {
                    println!("   - Task '{}' has no browser actions", task.title);
                }
            }
            
            assert_eq!(todo_tasks.len(), 3);
            println!("✅ SUCCESS: get_tasks_by_status includes browser actions correctly");
        }
        Err(e) => {
            println!("❌ ERROR: Failed to get tasks by status: {}", e);
            panic!("get_tasks_by_status failed");
        }
    }
}