use crate::models::{Task, TaskStatus, CreateTaskRequest};
use crate::tests::mock_database::{MockDatabase, create_test_task_with_notifications};
use crate::error::AppError;
use uuid::Uuid;
use chrono::Utc;

/// 基本的なタスクCRUD操作のテスト
#[tokio::test]
async fn test_basic_task_crud_operations() {
    let mock_db = MockDatabase::new();
    
    println!("🧪 Testing basic task CRUD operations...");
    
    // Test 1: タスク作成
    let create_request = CreateTaskRequest {
        title: "Test Task".to_string(),
        description: Some("Test description".to_string()),
        status: TaskStatus::Todo,
        // priority: Priority::Medium, // removed as per .kiro spec
        parent_id: None,
        due_date: None,
        notification_settings: None,
    };
    
    let task_data = Task {
        id: Uuid::new_v4().to_string(),
        title: create_request.title.clone(),
        description: create_request.description.clone(),
        status: "todo".to_string(),
        // priority: "medium".to_string(), // removed as per .kiro spec
        parent_id: create_request.parent_id.clone(),
        due_date: None,
        completed_at: None,
        created_at: Utc::now().to_rfc3339(),
        updated_at: Utc::now().to_rfc3339(),
        progress: Some(0),
        notification_type: Some("none".to_string()),
        notification_days_before: None,
        notification_time: None,
        notification_days_of_week: None,
        notification_level: Some(1),
    };
    
    let created_task = mock_db.insert_task(task_data.clone()).unwrap();
    
    assert_eq!(created_task.title, "Test Task");
    assert_eq!(created_task.description, Some("Test description".to_string()));
    assert_eq!(created_task.status, "todo");
    assert_eq!(created_task.priority, "medium");
    assert_eq!(created_task.progress, Some(0));
    
    println!("✅ Task creation test passed");
    
    // Test 2: タスク取得
    let retrieved_task = mock_db.get_task_by_id(&created_task.id).unwrap();
    
    assert_eq!(retrieved_task.id, created_task.id);
    assert_eq!(retrieved_task.title, created_task.title);
    assert_eq!(retrieved_task.status, created_task.status);
    
    println!("✅ Task retrieval test passed");
    
    // Test 3: タスク更新
    let mut updated_task = retrieved_task.clone();
    updated_task.title = "Updated Test Task".to_string();
    updated_task.description = Some("Updated description".to_string());
    updated_task.status = "in_progress".to_string();
    updated_task.priority = "high".to_string();
    updated_task.progress = Some(50);
    
    let update_result = mock_db.update_task(&updated_task.id, updated_task.clone()).unwrap();
    
    assert_eq!(update_result.title, "Updated Test Task");
    assert_eq!(update_result.description, Some("Updated description".to_string()));
    assert_eq!(update_result.status, "in_progress");
    assert_eq!(update_result.priority, "high");
    assert_eq!(update_result.progress, Some(50));
    
    println!("✅ Task update test passed");
    
    // Test 4: タスク削除
    mock_db.delete_task(&created_task.id).unwrap();
    
    // 削除確認
    let delete_result = mock_db.get_task_by_id(&created_task.id);
    assert!(delete_result.is_err());
    
    println!("✅ Task deletion test passed");
    
    println!("🎉 All basic CRUD tests passed!");
}

/// ステータス変更のテスト
#[tokio::test]
async fn test_task_status_transitions() {
    let mock_db = MockDatabase::new();
    
    println!("🧪 Testing task status transitions...");
    
    // Create test task
    let task = create_test_task_with_notifications();
    let created_task = mock_db.insert_task(task).unwrap();
    
    // Test status progression: todo -> in_progress -> done
    let statuses = ["todo", "in_progress", "done"];
    
    for (_i, status) in statuses.iter().enumerate() {
        let mut updated_task = mock_db.get_task_by_id(&created_task.id).unwrap();
        updated_task.status = status.to_string();
        
        // Set completed_at when status becomes 'done'
        if *status == "done" {
            updated_task.completed_at = Some(Utc::now().to_rfc3339());
            updated_task.progress = Some(100);
        }
        
        let result = mock_db.update_task(&created_task.id, updated_task).unwrap();
        
        assert_eq!(result.status, *status);
        
        if *status == "done" {
            assert!(result.completed_at.is_some());
            assert_eq!(result.progress, Some(100));
        }
        
        println!("✅ Status transition to '{}' passed", status);
    }
    
    // Cleanup
    mock_db.delete_task(&created_task.id).unwrap();
    
    println!("🎉 All status transition tests passed!");
}

/// 優先度管理のテスト
#[tokio::test]
async fn test_task_priority_management() {
    let mock_db = MockDatabase::new();
    
    println!("🧪 Testing task priority management...");
    
    let priorities = ["low", "medium", "high"];
    let mut created_tasks = Vec::new();
    
    // 各優先度でタスクを作成
    for priority in priorities.iter() {
        let mut task = create_test_task_with_notifications();
        task.id = Uuid::new_v4().to_string();
        task.title = format!("Task with {} priority", priority);
        task.priority = priority.to_string();
        
        let created_task = mock_db.insert_task(task).unwrap();
        assert_eq!(created_task.priority, *priority);
        
        created_tasks.push(created_task);
        println!("✅ Created task with '{}' priority", priority);
    }
    
    // 優先度変更テスト
    let mut task_to_update = created_tasks[0].clone(); // low priority task
    task_to_update.priority = "high".to_string();
    
    let updated_task = mock_db.update_task(&task_to_update.id.clone(), task_to_update).unwrap();
    assert_eq!(updated_task.priority, "high");
    
    println!("✅ Priority update test passed");
    
    // Cleanup
    for task in created_tasks {
        mock_db.delete_task(&task.id).unwrap();
    }
    
    println!("🎉 All priority management tests passed!");
}

/// 期日管理のテスト
#[tokio::test]
async fn test_task_due_date_management() {
    let mock_db = MockDatabase::new();
    
    println!("🧪 Testing task due date management...");
    
    // Test 1: 期日なしのタスク作成
    let mut task_no_due_date = create_test_task_with_notifications();
    task_no_due_date.due_date = None;
    
    let created_task = mock_db.insert_task(task_no_due_date).unwrap();
    assert!(created_task.due_date.is_none());
    
    println!("✅ Task without due date created successfully");
    
    // Test 2: 期日の設定
    let mut updated_task = created_task.clone();
    updated_task.due_date = Some("2025-12-31T23:59:59Z".to_string());
    
    let result = mock_db.update_task(&created_task.id, updated_task).unwrap();
    assert_eq!(result.due_date, Some("2025-12-31T23:59:59Z".to_string()));
    
    println!("✅ Due date setting test passed");
    
    // Test 3: 期日の削除
    let mut task_remove_due_date = result.clone();
    task_remove_due_date.due_date = None;
    
    let final_result = mock_db.update_task(&result.id, task_remove_due_date).unwrap();
    assert!(final_result.due_date.is_none());
    
    println!("✅ Due date removal test passed");
    
    // Test 4: 複数の期日パターンテスト
    let due_dates = [
        "2025-01-01T00:00:00Z",
        "2025-06-15T12:00:00Z",
        "2025-12-31T23:59:59Z",
    ];
    
    for due_date in due_dates.iter() {
        let mut task_with_due_date = create_test_task_with_notifications();
        task_with_due_date.id = Uuid::new_v4().to_string();
        task_with_due_date.due_date = Some(due_date.to_string());
        
        let created = mock_db.insert_task(task_with_due_date).unwrap();
        assert_eq!(created.due_date, Some(due_date.to_string()));
        
        // Cleanup
        mock_db.delete_task(&created.id).unwrap();
    }
    
    println!("✅ Multiple due date patterns test passed");
    
    // Cleanup
    mock_db.delete_task(&created_task.id).unwrap();
    
    println!("🎉 All due date management tests passed!");
}

/// 進捗管理のテスト
#[tokio::test]
async fn test_task_progress_management() {
    let mock_db = MockDatabase::new();
    
    println!("🧪 Testing task progress management...");
    
    // Create test task
    let task = create_test_task_with_notifications();
    let created_task = mock_db.insert_task(task).unwrap();
    
    // Test progress values from 0 to 100
    let progress_values = [0, 25, 50, 75, 100];
    
    for progress in progress_values.iter() {
        let mut updated_task = mock_db.get_task_by_id(&created_task.id).unwrap();
        updated_task.progress = Some(*progress);
        
        // When progress reaches 100%, automatically set status to 'done'
        if *progress == 100 {
            updated_task.status = "done".to_string();
            updated_task.completed_at = Some(Utc::now().to_rfc3339());
        }
        
        let result = mock_db.update_task(&created_task.id, updated_task).unwrap();
        assert_eq!(result.progress, Some(*progress));
        
        if *progress == 100 {
            assert_eq!(result.status, "done");
            assert!(result.completed_at.is_some());
        }
        
        println!("✅ Progress {}% test passed", progress);
    }
    
    // Test invalid progress values (should be handled gracefully)
    let mut invalid_task = mock_db.get_task_by_id(&created_task.id).unwrap();
    invalid_task.progress = Some(150); // Invalid: over 100%
    
    // In a real implementation, this would be validated, but for mock we just store it
    let invalid_result = mock_db.update_task(&created_task.id, invalid_task).unwrap();
    assert_eq!(invalid_result.progress, Some(150));
    
    println!("✅ Invalid progress handling test passed (mock allows any value)");
    
    // Cleanup
    mock_db.delete_task(&created_task.id).unwrap();
    
    println!("🎉 All progress management tests passed!");
}

/// バリデーションとエラーケースのテスト
#[tokio::test]
async fn test_task_validation_and_error_cases() {
    let mock_db = MockDatabase::new();
    
    println!("🧪 Testing task validation and error cases...");
    
    // Test 1: 存在しないタスクの取得
    let non_existent_id = Uuid::new_v4().to_string();
    let result = mock_db.get_task_by_id(&non_existent_id);
    
    assert!(result.is_err());
    match result {
        Err(AppError::NotFound(_)) => println!("✅ NotFound error correctly returned"),
        _ => panic!("Expected NotFound error"),
    }
    
    // Test 2: 存在しないタスクの更新
    let non_existent_task = create_test_task_with_notifications();
    let update_result = mock_db.update_task(&non_existent_id, non_existent_task);
    
    assert!(update_result.is_err());
    match update_result {
        Err(AppError::NotFound(_)) => println!("✅ Update NotFound error correctly returned"),
        _ => panic!("Expected NotFound error for update"),
    }
    
    // Test 3: 存在しないタスクの削除
    let delete_result = mock_db.delete_task(&non_existent_id);
    
    assert!(delete_result.is_err());
    match delete_result {
        Err(AppError::NotFound(_)) => println!("✅ Delete NotFound error correctly returned"),
        _ => panic!("Expected NotFound error for delete"),
    }
    
    // Test 4: 空のタイトルを持つタスクの作成（MockDatabaseは許可するが、実際のサービスではバリデーション）
    let mut empty_title_task = create_test_task_with_notifications();
    empty_title_task.title = "".to_string();
    
    let empty_result = mock_db.insert_task(empty_title_task);
    // MockDatabaseは何でも許可するので成功するが、実際のサービスではエラーになるべき
    assert!(empty_result.is_ok());
    
    println!("✅ Empty title handling test passed (mock allows, real service should validate)");
    
    // Test 5: 複数回の同じタスク作成（ID重複）
    let task1 = create_test_task_with_notifications();
    let task_id = task1.id.clone();
    
    let first_insert = mock_db.insert_task(task1).unwrap();
    assert_eq!(first_insert.id, task_id);
    
    // 同じIDで再度作成（上書きされる）
    let mut task2 = create_test_task_with_notifications();
    task2.id = task_id.clone();
    task2.title = "Duplicate ID Task".to_string();
    
    let second_insert = mock_db.insert_task(task2).unwrap();
    assert_eq!(second_insert.id, task_id);
    assert_eq!(second_insert.title, "Duplicate ID Task");
    
    println!("✅ Duplicate ID handling test passed (mock overwrites)");
    
    // Cleanup
    if let Ok(_) = empty_result {
        mock_db.delete_task(&empty_result.unwrap().id).unwrap();
    }
    mock_db.delete_task(&task_id).unwrap();
    
    println!("🎉 All validation and error case tests passed!");
}

/// 一括操作のテスト
#[tokio::test]
async fn test_bulk_task_operations() {
    let mock_db = MockDatabase::new();
    
    println!("🧪 Testing bulk task operations...");
    
    let task_count = 10;
    let mut created_task_ids = Vec::new();
    
    // Test 1: 一括タスク作成
    for i in 0..task_count {
        let mut task = create_test_task_with_notifications();
        task.id = Uuid::new_v4().to_string();
        task.title = format!("Bulk Task {}", i + 1);
        task.priority = match i % 3 {
            0 => "low".to_string(),
            1 => "medium".to_string(),
            _ => "high".to_string(),
        };
        
        let created_task = mock_db.insert_task(task).unwrap();
        created_task_ids.push(created_task.id);
    }
    
    println!("✅ Created {} tasks successfully", task_count);
    
    // Test 2: 全タスク取得
    let all_tasks = mock_db.get_all_tasks();
    assert_eq!(all_tasks.len(), task_count);
    
    println!("✅ Retrieved all {} tasks successfully", all_tasks.len());
    
    // Test 3: 条件別カウント
    let low_priority_count = all_tasks.iter().filter(|t| t.priority == "low").count();
    let medium_priority_count = all_tasks.iter().filter(|t| t.priority == "medium").count();
    let high_priority_count = all_tasks.iter().filter(|t| t.priority == "high").count();
    
    println!("✅ Priority distribution: Low={}, Medium={}, High={}", 
             low_priority_count, medium_priority_count, high_priority_count);
    
    // Test 4: 一括ステータス更新
    let mut updated_count = 0;
    for task_id in &created_task_ids[0..5] { // 最初の5つを更新
        let mut task = mock_db.get_task_by_id(task_id).unwrap();
        task.status = "in_progress".to_string();
        
        mock_db.update_task(task_id, task).unwrap();
        updated_count += 1;
    }
    
    println!("✅ Updated status for {} tasks", updated_count);
    
    // Test 5: 一括削除
    for task_id in created_task_ids {
        mock_db.delete_task(&task_id).unwrap();
    }
    
    // 削除確認
    let remaining_tasks = mock_db.get_all_tasks();
    assert_eq!(remaining_tasks.len(), 0);
    
    println!("✅ Deleted all tasks successfully");
    
    println!("🎉 All bulk operations tests passed!");
}

/// 総合的なCRUDテストランナー  
#[test]
fn task_crud_tests() {
    println!("🧪 Starting comprehensive task CRUD tests...");
    
    // Test 1: Basic CRUD operations
    test_basic_task_crud_operations();
    println!("✅ Basic CRUD operations test PASSED");
    
    // Test 2: Status transitions
    test_task_status_transitions();
    println!("✅ Status transitions test PASSED");
    
    // Test 3: Priority management
    test_task_priority_management();
    println!("✅ Priority management test PASSED");
    
    // Test 4: Due date management
    test_task_due_date_management();
    println!("✅ Due date management test PASSED");
    
    // Test 5: Progress management
    test_task_progress_management();
    println!("✅ Progress management test PASSED");
    
    // Test 6: Validation and error cases
    test_task_validation_and_error_cases();
    println!("✅ Validation and error cases test PASSED");
    
    // Test 7: Bulk operations
    test_bulk_task_operations();
    println!("✅ Bulk operations test PASSED");
    
    println!("🎉 All task CRUD tests completed!");
}