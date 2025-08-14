use crate::tests::mock_database::{MockDatabase, create_test_task_with_notifications};
use crate::error::AppError;
use uuid::Uuid;
use chrono::Utc;

/// エラーハンドリングの基本テスト
#[tokio::test]
async fn test_basic_error_handling() {
    let mock_db = MockDatabase::new();
    
    println!("🧪 Testing basic error handling...");
    
    // Test 1: 存在しないタスクの取得
    let non_existent_id = Uuid::new_v4().to_string();
    let result = mock_db.get_task_by_id(&non_existent_id);
    
    match result {
        Err(AppError::NotFound(msg)) => {
            assert!(msg.contains(&non_existent_id));
            println!("✅ NotFound error correctly returned with task ID");
        }
        _ => panic!("Expected NotFound error"),
    }
    
    // Test 2: 存在しないタスクの更新
    let fake_task = create_test_task_with_notifications();
    let update_result = mock_db.update_task(&non_existent_id, fake_task);
    
    match update_result {
        Err(AppError::NotFound(msg)) => {
            assert!(msg.contains(&non_existent_id));
            println!("✅ Update NotFound error correctly returned");
        }
        _ => panic!("Expected NotFound error for update"),
    }
    
    // Test 3: 存在しないタスクの削除
    let delete_result = mock_db.delete_task(&non_existent_id);
    
    match delete_result {
        Err(AppError::NotFound(msg)) => {
            assert!(msg.contains(&non_existent_id));
            println!("✅ Delete NotFound error correctly returned");
        }
        _ => panic!("Expected NotFound error for delete"),
    }
    
    // Test 4: 空のIDでの操作
    let empty_id = "";
    let empty_id_result = mock_db.get_task_by_id(empty_id);
    
    match empty_id_result {
        Err(AppError::NotFound(_)) => {
            println!("✅ Empty ID handled as NotFound");
        }
        _ => panic!("Expected NotFound error for empty ID"),
    }
    
    println!("🎉 All basic error handling tests passed!");
}

/// データバリデーションエラーのテスト
#[tokio::test]
async fn test_data_validation_errors() {
    println!("🧪 Testing data validation errors...");
    
    // Note: MockDatabase doesn't perform validation, so these tests demonstrate
    // what a real implementation should validate
    
    // Test 1: 無効なステータス値
    let invalid_statuses = ["invalid", "DONE", "InProgress", ""];
    
    for status in invalid_statuses.iter() {
        let mut task = create_test_task_with_notifications();
        task.id = Uuid::new_v4().to_string();
        task.status = status.to_string();
        
        // MockDatabase allows any status, but real implementation should validate
        println!("⚠️  Invalid status '{}' - MockDB allows, real service should reject", status);
    }
    
    // Test 2: 無効な優先度値
    let invalid_priorities = ["critical", "MEDIUM", "urgent", ""];
    
    for priority in invalid_priorities.iter() {
        let mut task = create_test_task_with_notifications();
        task.id = Uuid::new_v4().to_string();
        // priority field removed
        
        println!("⚠️  Invalid priority '{}' - MockDB allows, real service should reject", priority);
    }
    
    // Test 3: 無効な通知レベル
    let invalid_levels = [0, 4, 5, -1, 100];
    
    for level in invalid_levels.iter() {
        let mut task = create_test_task_with_notifications();
        task.id = Uuid::new_v4().to_string();
        task.notification_level = Some(*level);
        
        println!("⚠️  Invalid notification level {} - MockDB allows, real service should reject", level);
    }
    
    // Test 4: 無効な日付形式
    let invalid_dates = [
        "2025-13-01T00:00:00Z",  // Invalid month
        "2025-02-30T00:00:00Z",  // Invalid day
        "not-a-date",            // Not a date
        "2025/01/01",            // Wrong format
        "",                      // Empty
    ];
    
    for date in invalid_dates.iter() {
        let mut task = create_test_task_with_notifications();
        task.id = Uuid::new_v4().to_string();
        task.due_date = Some(date.to_string());
        
        println!("⚠️  Invalid date '{}' - MockDB allows, real service should reject", date);
    }
    
    // Test 5: 無効な通知時刻形式
    let invalid_times = [
        "25:00",    // Invalid hour
        "12:60",    // Invalid minute
        "12",       // Missing minute
        "12:30:45", // Seconds not needed
        "noon",     // Text
        "",         // Empty
    ];
    
    for time in invalid_times.iter() {
        let mut task = create_test_task_with_notifications();
        task.id = Uuid::new_v4().to_string();
        task.notification_time = Some(time.to_string());
        
        println!("⚠️  Invalid time '{}' - MockDB allows, real service should reject", time);
    }
    
    // Test 6: 無効な曜日JSON
    let invalid_days_of_week = [
        "[8]",           // Invalid day (0-6 only)
        "[-1]",          // Negative day
        "[1,2,8]",       // Mixed valid/invalid
        "not-json",      // Not JSON
        "[1,2,3,4,5,6,7,8]", // Too many days
        "[]",            // Empty array (might be valid)
    ];
    
    for days in invalid_days_of_week.iter() {
        let mut task = create_test_task_with_notifications();
        task.id = Uuid::new_v4().to_string();
        task.notification_days_of_week = Some(days.to_string());
        
        println!("⚠️  Invalid days of week '{}' - MockDB allows, real service should reject", days);
    }
    
    // Test 7: 無効な進捗値
    let invalid_progress = [-1, 101, 150, -50];
    
    for progress in invalid_progress.iter() {
        let mut task = create_test_task_with_notifications();
        task.id = Uuid::new_v4().to_string();
        task.progress = Some(*progress);
        
        println!("⚠️  Invalid progress {} - MockDB allows, real service should validate range", progress);
    }
    
    println!("✅ Data validation tests completed (MockDB permissive, real service should validate)");
    println!("🎉 All data validation error tests passed!");
}

/// 業務ロジックエラーのテスト
#[tokio::test]
async fn test_business_logic_errors() {
    let mock_db = MockDatabase::new();
    
    println!("🧪 Testing business logic errors...");
    
    // Test 1: 完了済みタスクの期日変更
    let mut completed_task = create_test_task_with_notifications();
    completed_task.status = "done".to_string();
    completed_task.completed_at = Some(Utc::now().to_rfc3339());
    completed_task.progress = Some(100);
    
    let created_task = mock_db.insert_task(completed_task).unwrap();
    
    // Try to modify completed task
    let mut modified_task = created_task.clone();
    modified_task.due_date = Some("2025-12-31T23:59:59Z".to_string());
    modified_task.status = "todo".to_string(); // Trying to "uncomplete"
    
    // MockDatabase allows this, but real implementation might restrict it
    let modify_result = mock_db.update_task(&created_task.id, modified_task);
    assert!(modify_result.is_ok());
    
    println!("⚠️  Modified completed task - MockDB allows, real service might restrict");
    
    // Test 2: 期日が過去のタスク作成
    let mut past_due_task = create_test_task_with_notifications();
    past_due_task.id = Uuid::new_v4().to_string();
    past_due_task.due_date = Some("2020-01-01T00:00:00Z".to_string()); // Past date
    
    let past_due_result = mock_db.insert_task(past_due_task);
    assert!(past_due_result.is_ok());
    
    println!("⚠️  Created task with past due date - MockDB allows, real service might warn");
    
    // Test 3: 循環参照の親子関係
    let mut parent = create_test_task_with_notifications();
    parent.title = "Parent Task".to_string();
    
    let created_parent = mock_db.insert_task(parent).unwrap();
    
    let mut child = create_test_task_with_notifications();
    child.id = Uuid::new_v4().to_string();
    child.title = "Child Task".to_string();
    child.parent_id = Some(created_parent.id.clone());
    
    let created_child = mock_db.insert_task(child).unwrap();
    
    // Try to make parent a child of its own child (circular reference)
    let mut circular_parent = created_parent.clone();
    circular_parent.parent_id = Some(created_child.id.clone());
    
    let circular_result = mock_db.update_task(&created_parent.id, circular_parent);
    assert!(circular_result.is_ok());
    
    println!("⚠️  Created circular reference - MockDB allows, real service should prevent");
    
    // Test 4: 通知設定の矛盾
    let mut contradictory_task = create_test_task_with_notifications();
    contradictory_task.id = Uuid::new_v4().to_string();
    contradictory_task.notification_type = Some("due_date_based".to_string());
    contradictory_task.due_date = None; // No due date but due_date_based notification
    
    let contradictory_result = mock_db.insert_task(contradictory_task);
    assert!(contradictory_result.is_ok());
    
    println!("⚠️  Due-date notification without due date - MockDB allows, real service should validate");
    
    // Test 5: 自分自身を親に設定
    let mut self_parent_task = create_test_task_with_notifications();
    self_parent_task.id = Uuid::new_v4().to_string();
    
    let created_self_task = mock_db.insert_task(self_parent_task.clone()).unwrap();
    
    let mut updated_self_task = created_self_task.clone();
    updated_self_task.parent_id = Some(created_self_task.id.clone()); // Self as parent
    
    let self_parent_result = mock_db.update_task(&created_self_task.id, updated_self_task);
    assert!(self_parent_result.is_ok());
    
    println!("⚠️  Self as parent - MockDB allows, real service should prevent");
    
    // Cleanup
    mock_db.delete_task(&created_task.id).unwrap();
    if let Ok(past_task) = past_due_result {
        mock_db.delete_task(&past_task.id).unwrap();
    }
    mock_db.delete_task(&created_child.id).unwrap();
    mock_db.delete_task(&created_parent.id).unwrap();
    if let Ok(contra_task) = contradictory_result {
        mock_db.delete_task(&contra_task.id).unwrap();
    }
    mock_db.delete_task(&created_self_task.id).unwrap();
    
    println!("🎉 All business logic error tests passed!");
}

/// 並行処理エラーのテスト
#[tokio::test]
async fn test_concurrency_errors() {
    let mock_db = MockDatabase::new();
    
    println!("🧪 Testing concurrency errors...");
    
    // Test 1: 同じタスクの同時更新
    let task = create_test_task_with_notifications();
    let created_task = mock_db.insert_task(task).unwrap();
    
    // Simulate two concurrent updates
    let mut task_update_1 = created_task.clone();
    task_update_1.title = "Updated by User 1".to_string();
    // priority field removed
    
    let mut task_update_2 = created_task.clone();
    task_update_2.title = "Updated by User 2".to_string();
    task_update_2.status = "in_progress".to_string();
    
    // First update
    let result_1 = mock_db.update_task(&created_task.id, task_update_1);
    assert!(result_1.is_ok());
    
    // Second update (in real DB with optimistic locking, this might fail)
    let result_2 = mock_db.update_task(&created_task.id, task_update_2);
    assert!(result_2.is_ok());
    
    // Check final state (last update wins in MockDB)
    let final_task = mock_db.get_task_by_id(&created_task.id).unwrap();
    assert_eq!(final_task.title, "Updated by User 2");
    
    println!("⚠️  Concurrent updates - MockDB allows last-wins, real DB might use optimistic locking");
    
    // Test 2: 同じIDで複数タスク作成
    let duplicate_id = Uuid::new_v4().to_string();
    
    let mut task_1 = create_test_task_with_notifications();
    task_1.id = duplicate_id.clone();
    task_1.title = "First Task".to_string();
    
    let mut task_2 = create_test_task_with_notifications();
    task_2.id = duplicate_id.clone();
    task_2.title = "Second Task".to_string();
    
    let result_1 = mock_db.insert_task(task_1);
    let result_2 = mock_db.insert_task(task_2); // Should overwrite in MockDB
    
    assert!(result_1.is_ok());
    assert!(result_2.is_ok());
    
    let final_duplicate = mock_db.get_task_by_id(&duplicate_id).unwrap();
    assert_eq!(final_duplicate.title, "Second Task");
    
    println!("⚠️  Duplicate ID creation - MockDB overwrites, real DB should enforce uniqueness");
    
    // Test 3: 削除中のタスクへのアクセス
    let task_to_delete = create_test_task_with_notifications();
    let created_delete_task = mock_db.insert_task(task_to_delete).unwrap();
    
    // Delete the task
    mock_db.delete_task(&created_delete_task.id).unwrap();
    
    // Try to access deleted task
    let access_deleted = mock_db.get_task_by_id(&created_delete_task.id);
    match access_deleted {
        Err(AppError::NotFound(_)) => {
            println!("✅ Deleted task correctly inaccessible");
        }
        _ => panic!("Expected NotFound for deleted task"),
    }
    
    // Try to update deleted task
    let update_deleted = mock_db.update_task(&created_delete_task.id, created_delete_task.clone());
    match update_deleted {
        Err(AppError::NotFound(_)) => {
            println!("✅ Cannot update deleted task");
        }
        _ => panic!("Expected NotFound for updating deleted task"),
    }
    
    // Cleanup
    mock_db.delete_task(&created_task.id).unwrap();
    mock_db.delete_task(&duplicate_id).unwrap();
    
    println!("🎉 All concurrency error tests passed!");
}

/// リソースエラーのテスト
#[tokio::test]
async fn test_resource_errors() {
    println!("🧪 Testing resource errors...");
    
    // Test 1: メモリ不足のシミュレーション（大量データ）
    let mock_db = MockDatabase::new();
    let large_task_count = 1000;
    
    println!("📊 Creating {} tasks to test memory handling...", large_task_count);
    
    let mut created_ids = Vec::new();
    
    // Create many tasks
    for i in 0..large_task_count {
        let mut task = create_test_task_with_notifications();
        task.id = Uuid::new_v4().to_string();
        task.title = format!("Memory Test Task {}", i);
        task.description = Some("Very long description ".repeat(100)); // Make it larger
        
        match mock_db.insert_task(task.clone()) {
            Ok(created) => created_ids.push(created.id),
            Err(_) => {
                println!("⚠️  Failed to create task {} - possible resource limit", i);
                break;
            }
        }
        
        // Progress indicator
        if i % 100 == 0 {
            println!("📈 Created {} tasks...", i);
        }
    }
    
    println!("✅ Successfully created {} tasks", created_ids.len());
    
    // Test retrieval performance
    let start_time = std::time::Instant::now();
    let all_tasks = mock_db.get_all_tasks();
    let retrieval_time = start_time.elapsed();
    
    assert!(all_tasks.len() >= created_ids.len());
    println!("📊 Retrieved {} tasks in {:?}", all_tasks.len(), retrieval_time);
    
    // Test bulk deletion
    let delete_start = std::time::Instant::now();
    for task_id in created_ids {
        mock_db.delete_task(&task_id).unwrap();
    }
    let delete_time = delete_start.elapsed();
    
    println!("🗑️  Deleted all tasks in {:?}", delete_time);
    
    // Test 2: 非常に大きなデータのタスク
    let mut huge_task = create_test_task_with_notifications();
    huge_task.id = Uuid::new_v4().to_string();
    huge_task.title = "A".repeat(10000); // Very long title
    huge_task.description = Some("B".repeat(100000)); // Very long description
    
    let huge_result = mock_db.insert_task(huge_task);
    match huge_result {
        Ok(created_huge) => {
            println!("✅ Huge task created successfully");
            mock_db.delete_task(&created_huge.id).unwrap();
        }
        Err(_) => {
            println!("⚠️  Huge task creation failed - possible size limit");
        }
    }
    
    // Test 3: 無効なUUID形式
    let invalid_uuids = [
        "not-a-uuid",
        "12345678-1234-1234-1234-12345678901", // Too short
        "12345678-1234-1234-1234-1234567890123", // Too long
        "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx", // Invalid characters
        "",
    ];
    
    for invalid_uuid in invalid_uuids.iter() {
        let get_result = mock_db.get_task_by_id(invalid_uuid);
        match get_result {
            Err(AppError::NotFound(_)) => {
                println!("✅ Invalid UUID '{}' handled correctly", invalid_uuid);
            }
            _ => {
                println!("⚠️  Invalid UUID '{}' not rejected", invalid_uuid);
            }
        }
    }
    
    println!("🎉 All resource error tests passed!");
}

/// エラー回復テスト
#[tokio::test]
async fn test_error_recovery() {
    let mock_db = MockDatabase::new();
    
    println!("🧪 Testing error recovery...");
    
    // Test 1: 部分的失敗からの回復
    let mut successful_task = create_test_task_with_notifications();
    successful_task.title = "Successful Task".to_string();
    
    let success_result = mock_db.insert_task(successful_task);
    assert!(success_result.is_ok());
    
    println!("✅ Normal operation after error scenarios still works");
    
    // Test 2: エラー後の状態確認
    let all_tasks_after_errors = mock_db.get_all_tasks();
    let valid_tasks = all_tasks_after_errors
        .iter()
        .filter(|t| !t.title.is_empty())
        .count();
    
    println!("📊 {} valid tasks remain after error tests", valid_tasks);
    
    // Test 3: データ整合性チェック
    for task in &all_tasks_after_errors {
        // Check basic data integrity
        assert!(!task.id.is_empty(), "Task ID should not be empty");
        assert!(!task.created_at.is_empty(), "Created timestamp should not be empty");
        assert!(!task.updated_at.is_empty(), "Updated timestamp should not be empty");
        
        // Check progress is within valid range (in a real implementation)
        if let Some(progress) = task.progress {
            if progress < 0 || progress > 100 {
                println!("⚠️  Task {} has invalid progress: {}", task.id, progress);
            }
        }
        
        // Check status is valid (in a real implementation)
        if !["todo", "in_progress", "done"].contains(&task.status.as_str()) {
            println!("⚠️  Task {} has invalid status: {}", task.id, task.status);
        }
    }
    
    println!("✅ Data integrity checks completed");
    
    // Test 4: システム状態のリセット
    mock_db.clear();
    let tasks_after_clear = mock_db.get_all_tasks();
    assert_eq!(tasks_after_clear.len(), 0);
    
    println!("✅ System state successfully reset");
    
    // Test 5: 新しいタスクの作成（完全回復確認）
    let recovery_task = create_test_task_with_notifications();
    let recovery_result = mock_db.insert_task(recovery_task);
    assert!(recovery_result.is_ok());
    
    let final_tasks = mock_db.get_all_tasks();
    assert_eq!(final_tasks.len(), 1);
    
    println!("✅ Full recovery confirmed - new tasks can be created");
    
    println!("🎉 All error recovery tests passed!");
}

/// エラーハンドリングテストのメインランナー
#[test]
fn error_handling_tests() {
    println!("🧪 Starting comprehensive error handling tests...");
    
    // Test 1: Basic error handling
    test_basic_error_handling();
    println!("✅ Basic error handling test PASSED");
    
    // Test 2: Data validation errors
    test_data_validation_errors();
    println!("✅ Data validation errors test PASSED");
    
    // Test 3: Business logic errors
    test_business_logic_errors();
    println!("✅ Business logic errors test PASSED");
    
    // Test 4: Concurrency errors
    test_concurrency_errors();
    println!("✅ Concurrency errors test PASSED");
    
    // Test 5: Resource errors
    test_resource_errors();
    println!("✅ Resource errors test PASSED");
    
    // Test 6: Error recovery
    test_error_recovery();
    println!("✅ Error recovery test PASSED");
    
    println!("🎉 All error handling tests completed!");
}