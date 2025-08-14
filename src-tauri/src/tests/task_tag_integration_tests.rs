use crate::database::Database;
use crate::database::migrations::run_migrations;
use crate::models::{CreateTaskRequest, UpdateTaskRequest, TaskStatus, CreateTagRequest};
use crate::services::{TaskService, TagService};
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};

/// Create a test SQLite database pool (in-memory)
async fn create_test_pool() -> Pool<Sqlite> {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .expect("Failed to create test database");
    
    // Run migrations to set up the schema
    run_migrations(&pool).await.expect("Failed to run migrations");
    
    pool
}

/// タスク更新時のタグ追加・削除テスト
#[tokio::test]
async fn test_task_update_with_tags() {
    let pool = create_test_pool().await;
    let db = Database { pool: pool.clone() };
    let task_service = TaskService::new(db);
    
    println!("🧪 Testing task update with tags...");
    
    // Step 1: タスクを作成
    let create_request = CreateTaskRequest {
        title: "タグテスト用タスク".to_string(),
        description: Some("タグ機能のテスト".to_string()),
        status: TaskStatus::Todo,
        parent_id: None,
        due_date: None,
        notification_settings: None,
    };
    
    let task = task_service.create_task(create_request).await.unwrap();
    println!("✅ Task created: {}", task.id);
    
    // Step 2: タグを作成
    let tag1_request = CreateTagRequest {
        name: "テストタグ1".to_string(),
        color: "#FF5733".to_string(),
    };
    let tag1 = TagService::create_tag(&pool, tag1_request).await.unwrap();
    println!("✅ Tag 1 created: {}", tag1.id);
    
    let tag2_request = CreateTagRequest {
        name: "テストタグ2".to_string(),
        color: "#33FF57".to_string(),
    };
    let tag2 = TagService::create_tag(&pool, tag2_request).await.unwrap();
    println!("✅ Tag 2 created: {}", tag2.id);
    
    // Step 3: タスクにタグを追加（update_taskメソッドを使用）
    let update_request = UpdateTaskRequest {
        title: Some("更新されたタスク".to_string()),
        description: None,
        status: None,
        parent_id: None,
        due_date: None,
        notification_settings: None,
        tags: Some(vec![tag1.clone(), tag2.clone()]),
    };
    
    let _updated_task = task_service.update_task(&task.id, update_request).await.unwrap();
    println!("✅ Task updated with tags");
    
    // Step 4: タグが正しく関連付けられたか確認
    let tags_for_task = TagService::get_tags_for_task(&pool, &task.id).await.unwrap();
    assert_eq!(tags_for_task.len(), 2);
    println!("✅ Task has {} tags attached", tags_for_task.len());
    
    // Step 5: タグを1つに減らす
    let update_request2 = UpdateTaskRequest {
        title: None,
        description: None,
        status: None,
        parent_id: None,
        due_date: None,
        notification_settings: None,
        tags: Some(vec![tag1.clone()]),
    };
    
    let _updated_task2 = task_service.update_task(&task.id, update_request2).await.unwrap();
    
    let tags_for_task2 = TagService::get_tags_for_task(&pool, &task.id).await.unwrap();
    assert_eq!(tags_for_task2.len(), 1);
    assert_eq!(tags_for_task2[0].id, tag1.id);
    println!("✅ Task tags reduced to 1");
    
    // Step 6: すべてのタグを削除
    let update_request3 = UpdateTaskRequest {
        title: None,
        description: None,
        status: None,
        parent_id: None,
        due_date: None,
        notification_settings: None,
        tags: Some(vec![]),
    };
    
    let _updated_task3 = task_service.update_task(&task.id, update_request3).await.unwrap();
    
    let tags_for_task3 = TagService::get_tags_for_task(&pool, &task.id).await.unwrap();
    assert_eq!(tags_for_task3.len(), 0);
    println!("✅ All tags removed from task");
    
    // Cleanup
    task_service.delete_task(&task.id).await.unwrap();
    TagService::delete_tag(&pool, &tag1.id).await.unwrap();
    TagService::delete_tag(&pool, &tag2.id).await.unwrap();
    
    println!("🎉 All task-tag integration tests passed!");
}

/// 新しいタグを作成して即座にタスクに追加するテスト
#[tokio::test]
async fn test_create_tag_and_add_to_task() {
    let pool = create_test_pool().await;
    let db = Database { pool: pool.clone() };
    let task_service = TaskService::new(db);
    
    println!("🧪 Testing create tag and immediately add to task...");
    
    // Step 1: タスクを作成
    let create_request = CreateTaskRequest {
        title: "新規タグテスト用タスク".to_string(),
        description: Some("新規タグを即座に追加".to_string()),
        status: TaskStatus::Todo,
        parent_id: None,
        due_date: None,
        notification_settings: None,
    };
    
    let task = task_service.create_task(create_request).await.unwrap();
    println!("✅ Task created: {}", task.id);
    
    // Step 2: 新しいタグを作成
    let new_tag_request = CreateTagRequest {
        name: "新規タグ".to_string(),
        color: "#3b82f6".to_string(),
    };
    let new_tag = TagService::create_tag(&pool, new_tag_request).await.unwrap();
    println!("✅ New tag created: {} ({})", new_tag.name, new_tag.id);
    
    // Step 3: 作成したタグを即座にタスクに追加
    let update_request = UpdateTaskRequest {
        title: None,
        description: None,
        status: None,
        parent_id: None,
        due_date: None,
        notification_settings: None,
        tags: Some(vec![new_tag.clone()]),
    };
    
    let updated_task = task_service.update_task(&task.id, update_request).await;
    
    // このテストでエラーが発生することを確認
    match updated_task {
        Ok(_) => {
            println!("✅ Successfully added new tag to task");
            
            // 確認: タグが正しく関連付けられたか
            let tags_for_task = TagService::get_tags_for_task(&pool, &task.id).await.unwrap();
            assert_eq!(tags_for_task.len(), 1);
            assert_eq!(tags_for_task[0].id, new_tag.id);
            println!("✅ Verified: Task has the new tag attached");
        },
        Err(e) => {
            panic!("❌ Failed to add new tag to task: {:?}", e);
        }
    }
    
    // Cleanup
    task_service.delete_task(&task.id).await.unwrap();
    TagService::delete_tag(&pool, &new_tag.id).await.unwrap();
    
    println!("🎉 Create tag and add to task test passed!");
}