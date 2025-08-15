use crate::database::Database;
use sqlx::Row;
use tempfile::tempdir;

#[tokio::test]
async fn test_database_schema_validation() {
    println!("=== Database Schema Validation Test ===");
    
    // Setup test database
    let temp_dir = tempdir().unwrap();
    let db_path = temp_dir.path().join("test_schema_validation.db");
    let db_url = format!("sqlite:{}?mode=rwc", db_path.display());
    
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(1)
        .connect(&db_url)
        .await
        .unwrap();
    
    // Run migrations
    crate::database::migrations::run_migrations(&pool).await.unwrap();
    
    let db = Database { pool };
    
    // Verify tasks table schema includes all required columns
    println!("Checking tasks table schema...");
    let column_info: Vec<String> = 
        sqlx::query("PRAGMA table_info(tasks)")
        .map(|row: sqlx::sqlite::SqliteRow| {
            row.get::<String, _>("name")
        })
        .fetch_all(&db.pool)
        .await
        .unwrap();
    
    println!("Found columns: {:?}", column_info);
    
    // Essential columns that must exist
    let required_columns = vec![
        "id",
        "title", 
        "description",
        "status",
        "parent_id",
        "due_date",
        "completed_at",
        "created_at",
        "updated_at",
        "progress",
        "notification_type",
        "notification_days_before", 
        "notification_time",
        "notification_days_of_week",
        "notification_level",
        "browser_actions" // This was the missing column that caused the issue
    ];
    
    for required_col in &required_columns {
        assert!(
            column_info.contains(&required_col.to_string()),
            "Required column '{}' is missing from tasks table. Found columns: {:?}",
            required_col,
            column_info
        );
        println!("✅ Required column '{}' exists", required_col);
    }
    
    // Verify tags table schema
    println!("\nChecking tags table schema...");
    let tags_columns: Vec<String> = sqlx::query("PRAGMA table_info(tags)")
        .map(|row: sqlx::sqlite::SqliteRow| row.get::<String, _>("name"))
        .fetch_all(&db.pool)
        .await
        .unwrap();
    
    let required_tags_columns = vec!["id", "name", "color", "created_at", "updated_at"];
    for required_col in &required_tags_columns {
        assert!(
            tags_columns.contains(&required_col.to_string()),
            "Required column '{}' is missing from tags table. Found columns: {:?}",
            required_col,
            tags_columns
        );
        println!("✅ Required column '{}' exists in tags table", required_col);
    }
    
    // Verify task_tags junction table schema
    println!("\nChecking task_tags table schema...");
    let task_tags_columns: Vec<String> = sqlx::query("PRAGMA table_info(task_tags)")
        .map(|row: sqlx::sqlite::SqliteRow| row.get::<String, _>("name"))
        .fetch_all(&db.pool)
        .await
        .unwrap();
    
    let required_task_tags_columns = vec!["task_id", "tag_id", "created_at"];
    for required_col in &required_task_tags_columns {
        assert!(
            task_tags_columns.contains(&required_col.to_string()),
            "Required column '{}' is missing from task_tags table. Found columns: {:?}",
            required_col,
            task_tags_columns
        );
        println!("✅ Required column '{}' exists in task_tags table", required_col);
    }
    
    // Test that we can actually perform a query that uses browser_actions column
    println!("\nTesting actual browser_actions column usage...");
    let result = sqlx::query("SELECT browser_actions FROM tasks LIMIT 0")
        .fetch_all(&db.pool)
        .await;
    
    match result {
        Ok(_) => println!("✅ browser_actions column is queryable"),
        Err(e) => {
            panic!("Failed to query browser_actions column: {}", e);
        }
    }
    
    // Verify foreign key constraints are properly set up
    println!("\nChecking foreign key constraints...");
    let foreign_keys: Vec<String> = sqlx::query("PRAGMA foreign_key_list(task_tags)")
        .map(|row: sqlx::sqlite::SqliteRow| format!("{}:{}", row.get::<String, _>("table"), row.get::<String, _>("from")))
        .fetch_all(&db.pool)
        .await
        .unwrap();
    
    assert!(foreign_keys.len() >= 2, "task_tags table should have at least 2 foreign key constraints");
    println!("✅ Foreign key constraints found: {:?}", foreign_keys);
    
    println!("\n🎉 All database schema validations passed!");
}

#[tokio::test]
async fn test_browser_actions_column_in_all_queries() {
    println!("=== Browser Actions Column Query Test ===");
    
    // Setup test database
    let temp_dir = tempdir().unwrap();
    let db_path = temp_dir.path().join("test_browser_actions_queries.db");
    let db_url = format!("sqlite:{}?mode=rwc", db_path.display());
    
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(1)
        .connect(&db_url)
        .await
        .unwrap();
    
    // Run migrations
    crate::database::migrations::run_migrations(&pool).await.unwrap();
    
    // Test all the main query patterns that should include browser_actions
    let test_queries = vec![
        // Main task queries
        "SELECT id, title, description, status, parent_id, due_date, completed_at, created_at, updated_at, progress, notification_type, notification_days_before, notification_time, notification_days_of_week, notification_level, browser_actions FROM tasks",
        
        // Status-based queries
        "SELECT id, title, description, status, parent_id, due_date, completed_at, created_at, updated_at, progress, notification_type, notification_days_before, notification_time, notification_days_of_week, notification_level, browser_actions FROM tasks WHERE status = 'todo'",
        
        // Notification queries
        "SELECT id, title, description, status, parent_id, due_date, completed_at, created_at, updated_at, progress, notification_type, notification_days_before, notification_time, notification_days_of_week, notification_level, browser_actions FROM tasks WHERE status != 'done' AND notification_type IS NOT NULL AND notification_type != 'none'",
        
        // Single task query
        "SELECT id, title, description, status, parent_id, due_date, completed_at, created_at, updated_at, progress, notification_type, notification_days_before, notification_time, notification_days_of_week, notification_level, browser_actions FROM tasks WHERE id = 'test-id'"
    ];
    
    for (i, query) in test_queries.iter().enumerate() {
        println!("Testing query {}: {}", i + 1, query);
        
        let result = sqlx::query(query)
            .fetch_all(&pool)
            .await;
        
        match result {
            Ok(_) => println!("✅ Query {} executed successfully", i + 1),
            Err(e) => {
                panic!("Query {} failed: {}\nQuery: {}", i + 1, e, query);
            }
        }
    }
    
    println!("\n🎉 All browser_actions column queries executed successfully!");
}