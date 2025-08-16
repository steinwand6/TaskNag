use sqlx::{sqlite::SqlitePoolOptions, Row};

#[tokio::test]
async fn check_real_database_schema() {
    println!("=== Checking REAL Database Schema ===");
    
    // Connect to the actual database file used by the application
    let app_data_dir = "C:\\Users\\stone\\AppData\\Roaming\\com.tasknag.app";
    let db_path = format!("{}\\tasknag.db", app_data_dir);
    let db_url = format!("sqlite:{}?mode=ro", db_path);
    
    println!("Connecting to real database: {}", db_url);
    
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect(&db_url)
        .await
        .unwrap();
    
    // Check tasks table schema
    println!("\n=== Tasks Table Schema ===");
    let column_info: Vec<(String, String, i32, Option<String>, i32)> = 
        sqlx::query("PRAGMA table_info(tasks)")
        .map(|row: sqlx::sqlite::SqliteRow| {
            (
                row.get::<String, _>("name"),
                row.get::<String, _>("type"),
                row.get::<i32, _>("notnull"),
                row.get::<Option<String>, _>("dflt_value"),
                row.get::<i32, _>("pk")
            )
        })
        .fetch_all(&pool)
        .await
        .unwrap();
    
    println!("Found {} columns in tasks table:", column_info.len());
    for (name, type_name, not_null, default_val, is_pk) in &column_info {
        let pk_text = if *is_pk == 1 { " (PRIMARY KEY)" } else { "" };
        let null_text = if *not_null == 1 { "NOT NULL" } else { "NULL" };
        let default_text = match default_val {
            Some(val) => format!(" DEFAULT {}", val),
            None => String::new(),
        };
        println!("  {}: {} {} {}{}", name, type_name, null_text, default_text, pk_text);
    }
    
    // Check if browser_actions column exists
    let browser_actions_exists = column_info.iter().any(|(name, _, _, _, _)| name == "browser_actions");
    println!("\n🔍 browser_actions column exists: {}", if browser_actions_exists { "✅ YES" } else { "❌ NO" });
    
    if browser_actions_exists {
        let browser_actions_info = column_info.iter()
            .find(|(name, _, _, _, _)| name == "browser_actions")
            .unwrap();
        println!("   Type: {}", browser_actions_info.1);
        println!("   Nullable: {}", if browser_actions_info.2 == 0 { "YES" } else { "NO" });
        println!("   Default: {:?}", browser_actions_info.3);
    }
    
    // Check migration history
    println!("\n=== Migration History ===");
    match sqlx::query("SELECT * FROM _sqlx_migrations ORDER BY version")
        .fetch_all(&pool)
        .await 
    {
        Ok(migrations) => {
            println!("Found {} applied migrations:", migrations.len());
            for row in migrations {
                let version: i64 = row.get("version");
                let description: String = row.get("description");
                let success: bool = row.get("success");
                let checksum: Vec<u8> = row.get("checksum");
                let execution_time: i64 = row.get("execution_time");
                
                println!("  Version {}: {} (success: {}, time: {}ms)", 
                    version, description, success, execution_time);
                println!("    Checksum length: {} bytes", checksum.len());
            }
        }
        Err(e) => {
            println!("❌ No migration table found or error reading migrations: {}", e);
            
            // Check if there are any tables at all
            let tables: Vec<String> = sqlx::query("SELECT name FROM sqlite_master WHERE type='table'")
                .map(|row: sqlx::sqlite::SqliteRow| row.get::<String, _>("name"))
                .fetch_all(&pool)
                .await
                .unwrap();
            
            println!("Available tables: {:?}", tables);
        }
    }
    
    // Test a simple query that includes browser_actions
    println!("\n=== Testing browser_actions Query ===");
    match sqlx::query("SELECT browser_actions FROM tasks LIMIT 1")
        .fetch_all(&pool)
        .await 
    {
        Ok(rows) => {
            println!("✅ SUCCESS: browser_actions column is queryable, found {} rows", rows.len());
        }
        Err(e) => {
            println!("❌ ERROR: Cannot query browser_actions column: {}", e);
        }
    }
    
    // Count total tasks
    let task_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM tasks")
        .fetch_one(&pool)
        .await
        .unwrap();
    println!("\nTotal tasks in database: {}", task_count);
    
    // Check what the actual query that's failing looks like
    println!("\n=== Testing Different Query Patterns ===");
    
    // Pattern 1: Without browser_actions
    match sqlx::query("SELECT id, title, description, status, parent_id, due_date, completed_at, created_at, updated_at, progress, notification_type, notification_days_before, notification_time, notification_days_of_week, notification_level FROM tasks LIMIT 1")
        .fetch_all(&pool)
        .await 
    {
        Ok(_) => println!("✅ Query WITHOUT browser_actions works"),
        Err(e) => println!("❌ Query WITHOUT browser_actions fails: {}", e),
    }
    
    // Pattern 2: With browser_actions
    match sqlx::query("SELECT id, title, description, status, parent_id, due_date, completed_at, created_at, updated_at, progress, notification_type, notification_days_before, notification_time, notification_days_of_week, notification_level, browser_actions FROM tasks LIMIT 1")
        .fetch_all(&pool)
        .await 
    {
        Ok(_) => println!("✅ Query WITH browser_actions works"),
        Err(e) => println!("❌ Query WITH browser_actions fails: {}", e),
    }
}