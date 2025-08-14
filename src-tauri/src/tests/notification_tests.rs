use crate::models::{Task, TaskNotificationSettings, CreateTaskRequest, UpdateTaskRequest, TaskStatus, Priority};
use futures;
use crate::tests::mock_database::{MockDatabase, create_test_task_with_notifications, create_test_task_due_date_based};
use crate::services::TaskService;
use crate::database::Database;
use uuid::Uuid;

#[tokio::test]
async fn test_notification_settings_mapping() {
    let mock_db = MockDatabase::new();
    
    // Test 1: Create task with recurring notifications
    let test_task = create_test_task_with_notifications();
    let inserted_task = mock_db.insert_task(test_task.clone()).unwrap();
    
    println!("✅ Test 1: Task created with ID: {}", inserted_task.id);
    
    // Verify notification settings are preserved
    assert_eq!(inserted_task.notification_type, Some("recurring".to_string()));
    assert_eq!(inserted_task.notification_time, Some("09:00".to_string()));
    assert_eq!(inserted_task.notification_days_of_week, Some("[1,2,3,4,5]".to_string()));
    assert_eq!(inserted_task.notification_level, Some(2));
    
    println!("✅ Recurring notification settings verified");
    
    // Test 2: Retrieve task and verify settings
    let retrieved_task = mock_db.get_task_by_id(&inserted_task.id).unwrap();
    
    assert_eq!(retrieved_task.notification_type, Some("recurring".to_string()));
    assert_eq!(retrieved_task.notification_time, Some("09:00".to_string()));
    assert_eq!(retrieved_task.notification_level, Some(2));
    
    println!("✅ Task retrieval and settings verification passed");
    
    // Test 3: Update notification settings
    let mut updated_task = retrieved_task.clone();
    updated_task.notification_type = Some("due_date_based".to_string());
    updated_task.notification_days_before = Some(3);
    updated_task.notification_time = Some("10:30".to_string());
    updated_task.notification_days_of_week = None;
    updated_task.notification_level = Some(3);
    
    let updated_result = mock_db.update_task(&updated_task.id, updated_task.clone()).unwrap();
    
    assert_eq!(updated_result.notification_type, Some("due_date_based".to_string()));
    assert_eq!(updated_result.notification_days_before, Some(3));
    assert_eq!(updated_result.notification_time, Some("10:30".to_string()));
    assert_eq!(updated_result.notification_level, Some(3));
    
    println!("✅ Notification settings update verified");
    
    // Test 4: Create due date based task
    let due_date_task = create_test_task_due_date_based();
    let inserted_due_task = mock_db.insert_task(due_date_task).unwrap();
    
    assert_eq!(inserted_due_task.notification_type, Some("due_date_based".to_string()));
    assert_eq!(inserted_due_task.notification_days_before, Some(3));
    assert_eq!(inserted_due_task.notification_level, Some(3));
    
    println!("✅ Due date based notification task verified");
    
    // Test 5: Delete tasks (cleanup)
    mock_db.delete_task(&inserted_task.id).unwrap();
    mock_db.delete_task(&inserted_due_task.id).unwrap();
    
    // Verify deletion
    assert!(mock_db.get_task_by_id(&inserted_task.id).is_err());
    assert!(mock_db.get_task_by_id(&inserted_due_task.id).is_err());
    
    println!("✅ Task deletion verified");
    
    println!("🎉 All notification settings tests passed!");
}

#[tokio::test]
async fn test_notification_settings_validation() {
    let mock_db = MockDatabase::new();
    
    // Test notification level validation
    let mut task = create_test_task_with_notifications();
    task.notification_level = Some(1);
    let result1 = mock_db.insert_task(task.clone()).unwrap();
    assert_eq!(result1.notification_level, Some(1));
    
    task.id = Uuid::new_v4().to_string();
    task.notification_level = Some(2);
    let result2 = mock_db.insert_task(task.clone()).unwrap();
    assert_eq!(result2.notification_level, Some(2));
    
    task.id = Uuid::new_v4().to_string();
    task.notification_level = Some(3);
    let result3 = mock_db.insert_task(task.clone()).unwrap();
    assert_eq!(result3.notification_level, Some(3));
    
    println!("✅ Notification level validation passed");
    
    // Test notification type validation
    task.id = Uuid::new_v4().to_string();
    task.notification_type = Some("none".to_string());
    let none_result = mock_db.insert_task(task.clone()).unwrap();
    assert_eq!(none_result.notification_type, Some("none".to_string()));
    
    task.id = Uuid::new_v4().to_string();
    task.notification_type = Some("recurring".to_string());
    let recurring_result = mock_db.insert_task(task.clone()).unwrap();
    assert_eq!(recurring_result.notification_type, Some("recurring".to_string()));
    
    task.id = Uuid::new_v4().to_string();
    task.notification_type = Some("due_date_based".to_string());
    let due_date_result = mock_db.insert_task(task.clone()).unwrap();
    assert_eq!(due_date_result.notification_type, Some("due_date_based".to_string()));
    
    println!("✅ Notification type validation passed");
    
    // Test JSON parsing for days of week
    task.id = Uuid::new_v4().to_string();
    task.notification_days_of_week = Some("[0,6]".to_string()); // Weekend
    let weekend_result = mock_db.insert_task(task.clone()).unwrap();
    assert_eq!(weekend_result.notification_days_of_week, Some("[0,6]".to_string()));
    
    task.id = Uuid::new_v4().to_string();
    task.notification_days_of_week = Some("[1,2,3,4,5]".to_string()); // Weekdays
    let weekday_result = mock_db.insert_task(task.clone()).unwrap();
    assert_eq!(weekday_result.notification_days_of_week, Some("[1,2,3,4,5]".to_string()));
    
    println!("✅ Days of week JSON validation passed");
    
    println!("🎉 All validation tests passed!");
}

#[tokio::test]
async fn test_notification_settings_edge_cases() {
    let mock_db = MockDatabase::new();
    
    // Test with null/empty notification settings
    let mut task = create_test_task_with_notifications();
    task.notification_type = None;
    task.notification_days_before = None;
    task.notification_time = None;
    task.notification_days_of_week = None;
    task.notification_level = None;
    
    let null_result = mock_db.insert_task(task.clone()).unwrap();
    assert_eq!(null_result.notification_type, None);
    assert_eq!(null_result.notification_days_before, None);
    assert_eq!(null_result.notification_time, None);
    assert_eq!(null_result.notification_days_of_week, None);
    assert_eq!(null_result.notification_level, None);
    
    println!("✅ Null notification settings handled correctly");
    
    // Test updating from null to actual settings
    let mut updated_task = null_result.clone();
    updated_task.notification_type = Some("recurring".to_string());
    updated_task.notification_time = Some("15:30".to_string());
    updated_task.notification_days_of_week = Some("[1,3,5]".to_string());
    updated_task.notification_level = Some(2);
    
    let updated_result = mock_db.update_task(&updated_task.id, updated_task.clone()).unwrap();
    assert_eq!(updated_result.notification_type, Some("recurring".to_string()));
    assert_eq!(updated_result.notification_time, Some("15:30".to_string()));
    assert_eq!(updated_result.notification_level, Some(2));
    
    println!("✅ Update from null to actual settings verified");
    
    // Test updating from actual settings to null
    let mut nullify_task = updated_result.clone();
    nullify_task.notification_type = Some("none".to_string());
    nullify_task.notification_days_before = None;
    nullify_task.notification_time = None;
    nullify_task.notification_days_of_week = None;
    nullify_task.notification_level = Some(1);
    
    let nullified_result = mock_db.update_task(&nullify_task.id, nullify_task.clone()).unwrap();
    assert_eq!(nullified_result.notification_type, Some("none".to_string()));
    assert_eq!(nullified_result.notification_time, None);
    assert_eq!(nullified_result.notification_days_of_week, None);
    
    println!("✅ Update to null settings verified");
    
    println!("🎉 All edge case tests passed!");
}

// Run all tests function - simplified version that uses MockDatabase directly
pub fn run_all_notification_tests() -> String {
    let mut results = Vec::new();
    
    results.push("🧪 Starting comprehensive notification settings tests...".to_string());
    
    // Test 1: Basic notification settings mapping
    match std::panic::catch_unwind(|| {
        let mock_db = MockDatabase::new();
        
        // Create task with recurring notifications
        let test_task = create_test_task_with_notifications();
        let inserted_task = mock_db.insert_task(test_task.clone()).unwrap();
        
        // Verify notification settings are preserved
        assert_eq!(inserted_task.notification_type, Some("recurring".to_string()));
        assert_eq!(inserted_task.notification_time, Some("09:00".to_string()));
        assert_eq!(inserted_task.notification_days_of_week, Some("[1,2,3,4,5]".to_string()));
        assert_eq!(inserted_task.notification_level, Some(2));
        
        // Retrieve task and verify settings
        let retrieved_task = mock_db.get_task_by_id(&inserted_task.id).unwrap();
        assert_eq!(retrieved_task.notification_type, Some("recurring".to_string()));
        assert_eq!(retrieved_task.notification_time, Some("09:00".to_string()));
        assert_eq!(retrieved_task.notification_level, Some(2));
        
        // Update notification settings
        let mut updated_task = retrieved_task.clone();
        updated_task.notification_type = Some("due_date_based".to_string());
        updated_task.notification_days_before = Some(3);
        updated_task.notification_time = Some("10:30".to_string());
        updated_task.notification_days_of_week = None;
        updated_task.notification_level = Some(3);
        
        let updated_result = mock_db.update_task(&updated_task.id, updated_task.clone()).unwrap();
        assert_eq!(updated_result.notification_type, Some("due_date_based".to_string()));
        assert_eq!(updated_result.notification_days_before, Some(3));
        assert_eq!(updated_result.notification_time, Some("10:30".to_string()));
        assert_eq!(updated_result.notification_level, Some(3));
        
        // Cleanup
        mock_db.delete_task(&inserted_task.id).unwrap();
    }) {
        Ok(_) => results.push("✅ Basic notification settings mapping test PASSED".to_string()),
        Err(_) => results.push("❌ Basic notification settings mapping test FAILED".to_string()),
    }
    
    // Test 2: Validation tests
    match std::panic::catch_unwind(|| {
        let mock_db = MockDatabase::new();
        
        // Test notification level validation
        let mut task = create_test_task_with_notifications();
        for level in [1, 2, 3] {
            task.id = uuid::Uuid::new_v4().to_string();
            task.notification_level = Some(level);
            let result = mock_db.insert_task(task.clone()).unwrap();
            assert_eq!(result.notification_level, Some(level));
        }
        
        // Test notification type validation
        for ntype in ["none", "recurring", "due_date_based"] {
            task.id = uuid::Uuid::new_v4().to_string();
            task.notification_type = Some(ntype.to_string());
            let result = mock_db.insert_task(task.clone()).unwrap();
            assert_eq!(result.notification_type, Some(ntype.to_string()));
        }
        
        // Test JSON parsing for days of week
        for days in ["[0,6]", "[1,2,3,4,5]"] {
            task.id = uuid::Uuid::new_v4().to_string();
            task.notification_days_of_week = Some(days.to_string());
            let result = mock_db.insert_task(task.clone()).unwrap();
            assert_eq!(result.notification_days_of_week, Some(days.to_string()));
        }
    }) {
        Ok(_) => results.push("✅ Notification settings validation test PASSED".to_string()),
        Err(_) => results.push("❌ Notification settings validation test FAILED".to_string()),
    }
    
    // Test 3: Edge cases
    match std::panic::catch_unwind(|| {
        let mock_db = MockDatabase::new();
        
        // Test with null/empty notification settings
        let mut task = create_test_task_with_notifications();
        task.notification_type = None;
        task.notification_days_before = None;
        task.notification_time = None;
        task.notification_days_of_week = None;
        task.notification_level = None;
        
        let null_result = mock_db.insert_task(task.clone()).unwrap();
        assert_eq!(null_result.notification_type, None);
        assert_eq!(null_result.notification_days_before, None);
        assert_eq!(null_result.notification_time, None);
        assert_eq!(null_result.notification_days_of_week, None);
        assert_eq!(null_result.notification_level, None);
        
        // Test updating from null to actual settings
        let mut updated_task = null_result.clone();
        updated_task.notification_type = Some("recurring".to_string());
        updated_task.notification_time = Some("15:30".to_string());
        updated_task.notification_days_of_week = Some("[1,3,5]".to_string());
        updated_task.notification_level = Some(2);
        
        let updated_result = mock_db.update_task(&updated_task.id, updated_task.clone()).unwrap();
        assert_eq!(updated_result.notification_type, Some("recurring".to_string()));
        assert_eq!(updated_result.notification_time, Some("15:30".to_string()));
        assert_eq!(updated_result.notification_level, Some(2));
        
        // Test updating from actual settings to null
        let mut nullify_task = updated_result.clone();
        nullify_task.notification_type = Some("none".to_string());
        nullify_task.notification_days_before = None;
        nullify_task.notification_time = None;
        nullify_task.notification_days_of_week = None;
        nullify_task.notification_level = Some(1);
        
        let nullified_result = mock_db.update_task(&nullify_task.id, nullify_task.clone()).unwrap();
        assert_eq!(nullified_result.notification_type, Some("none".to_string()));
        assert_eq!(nullified_result.notification_time, None);
        assert_eq!(nullified_result.notification_days_of_week, None);
    }) {
        Ok(_) => results.push("✅ Edge cases test PASSED".to_string()),
        Err(_) => results.push("❌ Edge cases test FAILED".to_string()),
    }
    
    results.push("🎉 All notification tests completed!".to_string());
    
    results.join("\n")
}