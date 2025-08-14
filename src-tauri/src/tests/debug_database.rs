use sqlx::sqlite::SqlitePoolOptions;

#[tokio::test]
async fn debug_real_database() {
    // 実際のデータベースファイルのパスを動的に取得
    let app_data_dir = if let Some(appdata) = std::env::var_os("APPDATA") {
        std::path::PathBuf::from(appdata).join("com.tasknag.app")
    } else {
        // フォールバック: 一般的なパス
        dirs::config_dir().unwrap_or_default().join("com.tasknag.app")
    };
    
    let db_path = app_data_dir.join("tasknag.db");
    println!("Database path: {:?}", db_path);
    
    if !db_path.exists() {
        println!("Database file does not exist!");
        return;
    }
    
    let db_url = format!("sqlite:{}?mode=ro", db_path.display());
    
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect(&db_url)
        .await
        .expect("Failed to connect to database");
    
    // タスクの確認
    let task_id = "dab124b7-a6b8-4131-821d-28746b58834f";
    let task_exists: Option<(String,)> = sqlx::query_as(
        "SELECT id FROM tasks WHERE id = ?1"
    )
    .bind(task_id)
    .fetch_optional(&pool)
    .await
    .unwrap();
    
    println!("Task {} exists: {}", task_id, task_exists.is_some());
    
    // タグの確認
    let tag_id = "d53c8d57-2fa6-4661-ad56-bfd3ab81dc44";
    let tag_exists: Option<(String,)> = sqlx::query_as(
        "SELECT id FROM tags WHERE id = ?1"
    )
    .bind(tag_id)
    .fetch_optional(&pool)
    .await
    .unwrap();
    
    println!("Tag {} exists: {}", tag_id, tag_exists.is_some());
    
    // 既存のタスク-タグ関連の確認
    let existing_relations: Vec<(String, String)> = sqlx::query_as(
        "SELECT task_id, tag_id FROM task_tags WHERE task_id = ?1"
    )
    .bind(task_id)
    .fetch_all(&pool)
    .await
    .unwrap();
    
    println!("Existing task_tags for task {}: {:?}", task_id, existing_relations);
    
    // すべてのタスクを表示
    let all_tasks: Vec<(String, String)> = sqlx::query_as(
        "SELECT id, title FROM tasks LIMIT 10"
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    
    println!("\nAll tasks:");
    for (id, title) in all_tasks {
        println!("  - {}: {}", id, title);
    }
    
    // すべてのタグを表示
    let all_tags: Vec<(String, String)> = sqlx::query_as(
        "SELECT id, name FROM tags"
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    
    println!("\nAll tags:");
    for (id, name) in all_tags {
        println!("  - {}: {}", id, name);
    }
}