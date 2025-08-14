use crate::database::Database;
use crate::models::{UpdateTaskRequest, Tag};
use crate::services::TaskService;
use sqlx::sqlite::SqlitePoolOptions;

/// 実際のデータベースでタグ追加をテスト
#[tokio::test]
async fn test_real_database_tag_update() {
    // 実際のデータベースファイルのパスを動的に取得
    let app_data_dir = if let Some(appdata) = std::env::var_os("APPDATA") {
        std::path::PathBuf::from(appdata).join("com.tasknag.app")
    } else {
        // フォールバック: 一般的なパス
        dirs::config_dir().unwrap_or_default().join("com.tasknag.app")
    };
    let db_path = app_data_dir.join("tasknag.db");
    
    if !db_path.exists() {
        println!("Database file does not exist!");
        return;
    }
    
    let db_url = format!("sqlite:{}?mode=rw", db_path.display());
    
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect(&db_url)
        .await
        .expect("Failed to connect to database");
    
    // FOREIGN KEY制約を有効化
    sqlx::query("PRAGMA foreign_keys = ON")
        .execute(&pool)
        .await
        .unwrap();
    
    println!("Connected to real database");
    
    let db = Database { pool: pool.clone() };
    let task_service = TaskService::new(db);
    
    // 実際のタスクとタグのID
    let task_id = "dab124b7-a6b8-4131-821d-28746b58834f";
    let tag_id = "d53c8d57-2fa6-4661-ad56-bfd3ab81dc44";
    
    // タスクの存在確認
    let task_exists: Option<(String,)> = sqlx::query_as(
        "SELECT id FROM tasks WHERE id = ?1"
    )
    .bind(task_id)
    .fetch_optional(&pool)
    .await
    .unwrap();
    
    println!("Task {} exists: {}", task_id, task_exists.is_some());
    
    // タグの存在確認
    let tag_row: Option<(String, String, String)> = sqlx::query_as(
        "SELECT id, name, color FROM tags WHERE id = ?1"
    )
    .bind(tag_id)
    .fetch_optional(&pool)
    .await
    .unwrap();
    
    if let Some((id, name, color)) = tag_row {
        println!("Tag exists: id={}, name={}, color={}", id, name, color);
        
        // タグオブジェクトを作成
        let tag = Tag {
            id: id.clone(),
            name: name.clone(),
            color: color.clone(),
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        };
        
        // タスク更新リクエストを作成
        let update_request = UpdateTaskRequest {
            title: None,
            description: None,
            status: None,
            parent_id: None,
            due_date: None,
            notification_settings: None,
            tags: Some(vec![tag]),
        };
        
        println!("Attempting to update task with tag...");
        
        // タスクを更新
        match task_service.update_task(task_id, update_request).await {
            Ok(_) => {
                println!("✅ Successfully updated task with tag!");
                
                // 確認: タグが関連付けられたか
                let tag_relations: Vec<(String, String)> = sqlx::query_as(
                    "SELECT task_id, tag_id FROM task_tags WHERE task_id = ?1"
                )
                .bind(task_id)
                .fetch_all(&pool)
                .await
                .unwrap();
                
                println!("Task-tag relations after update: {:?}", tag_relations);
            },
            Err(e) => {
                println!("❌ Failed to update task with tag: {:?}", e);
            }
        }
    } else {
        println!("Tag {} does not exist!", tag_id);
    }
}