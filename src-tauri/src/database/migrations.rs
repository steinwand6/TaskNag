use sqlx::{Pool, Sqlite};

pub async fn run_migrations(pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
    // Create tasks table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS tasks (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            description TEXT,
            status TEXT NOT NULL CHECK(status IN ('inbox', 'todo', 'in_progress', 'done')),
            priority TEXT NOT NULL CHECK(priority IN ('low', 'medium', 'high', 'required')),
            parent_id TEXT,
            due_date TEXT,
            completed_at TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            progress INTEGER DEFAULT 0 CHECK(progress >= 0 AND progress <= 100),
            FOREIGN KEY (parent_id) REFERENCES tasks(id) ON DELETE CASCADE
        )
        "#,
    )
    .execute(pool)
    .await?;
    
    // Create tags table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS tags (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL UNIQUE,
            color TEXT,
            created_at TEXT NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await?;
    
    // Create task_tags junction table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS task_tags (
            task_id TEXT NOT NULL,
            tag_id TEXT NOT NULL,
            PRIMARY KEY (task_id, tag_id),
            FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE CASCADE,
            FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
        )
        "#,
    )
    .execute(pool)
    .await?;
    
    // Create indexes for better performance
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_tasks_status ON tasks(status)")
        .execute(pool)
        .await?;
    
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_tasks_parent_id ON tasks(parent_id)")
        .execute(pool)
        .await?;
    
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_tasks_due_date ON tasks(due_date)")
        .execute(pool)
        .await?;
    
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_task_tags_task_id ON task_tags(task_id)")
        .execute(pool)
        .await?;
    
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_task_tags_tag_id ON task_tags(tag_id)")
        .execute(pool)
        .await?;
    
    
    // Add progress column if it doesn't exist (for existing databases)
    sqlx::query(
        r#"
        ALTER TABLE tasks ADD COLUMN progress INTEGER DEFAULT 0 CHECK(progress >= 0 AND progress <= 100)
        "#,
    )
    .execute(pool)
    .await
    .ok(); // Ignore error if column already exists
    
    // Update priority values from 'urgent' to 'required' if needed
    sqlx::query(
        r#"
        UPDATE tasks SET priority = 'required' WHERE priority = 'urgent'
        "#,
    )
    .execute(pool)
    .await
    .ok();
    
    // Add notification columns for new notification system
    sqlx::query(
        r#"
        ALTER TABLE tasks ADD COLUMN notification_type TEXT DEFAULT 'none' CHECK(notification_type IN ('none', 'due_date_based', 'recurring'))
        "#,
    )
    .execute(pool)
    .await
    .ok(); // Ignore error if column already exists
    
    sqlx::query(
        r#"
        ALTER TABLE tasks ADD COLUMN notification_days_before INTEGER DEFAULT NULL
        "#,
    )
    .execute(pool)
    .await
    .ok();
    
    sqlx::query(
        r#"
        ALTER TABLE tasks ADD COLUMN notification_time TEXT DEFAULT NULL
        "#,
    )
    .execute(pool)
    .await
    .ok();
    
    sqlx::query(
        r#"
        ALTER TABLE tasks ADD COLUMN notification_days_of_week TEXT DEFAULT NULL
        "#,
    )
    .execute(pool)
    .await
    .ok();
    
    sqlx::query(
        r#"
        ALTER TABLE tasks ADD COLUMN notification_level INTEGER DEFAULT 1 CHECK(notification_level IN (1, 2, 3))
        "#,
    )
    .execute(pool)
    .await
    .ok();
    
    // Create indexes for notification queries
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_tasks_notification_type ON tasks(notification_type)")
        .execute(pool)
        .await
        .ok();
    
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_tasks_status_notification ON tasks(status, notification_type)")
        .execute(pool)
        .await
        .ok();
    
    // MIGRATION: Remove priority system (as per .kiro/specs/notification-system-redesign)
    // Step 1: Create new table without priority column
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS tasks_new (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            description TEXT,
            status TEXT NOT NULL CHECK(status IN ('inbox', 'todo', 'in_progress', 'done')),
            parent_id TEXT,
            due_date TEXT,
            completed_at TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            progress INTEGER DEFAULT 0 CHECK(progress >= 0 AND progress <= 100),
            notification_type TEXT DEFAULT 'none' CHECK(notification_type IN ('none', 'due_date_based', 'recurring')),
            notification_days_before INTEGER DEFAULT NULL,
            notification_time TEXT DEFAULT NULL,
            notification_days_of_week TEXT DEFAULT NULL,
            notification_level INTEGER DEFAULT 1 CHECK(notification_level IN (1, 2, 3)),
            FOREIGN KEY (parent_id) REFERENCES tasks(id) ON DELETE CASCADE
        )
        "#,
    )
    .execute(pool)
    .await
    .ok();
    
    // Step 2: Copy data from old table to new table (excluding priority)
    sqlx::query(
        r#"
        INSERT OR IGNORE INTO tasks_new (
            id, title, description, status, parent_id, due_date, completed_at, 
            created_at, updated_at, progress, notification_type, notification_days_before, 
            notification_time, notification_days_of_week, notification_level
        )
        SELECT 
            id, title, description, status, parent_id, due_date, completed_at, 
            created_at, updated_at, 
            COALESCE(progress, 0) as progress,
            COALESCE(notification_type, 'none') as notification_type,
            notification_days_before, notification_time, notification_days_of_week,
            COALESCE(notification_level, 1) as notification_level
        FROM tasks
        WHERE id NOT IN (SELECT id FROM tasks_new)
        "#,
    )
    .execute(pool)
    .await
    .ok();
    
    // Step 3: Drop old table and rename new table (only if migration is needed)
    let has_priority_column: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM pragma_table_info('tasks') WHERE name = 'priority'"
    )
    .fetch_one(pool)
    .await
    .unwrap_or(0);
    
    if has_priority_column > 0 {
        sqlx::query("DROP TABLE IF EXISTS tasks_old")
            .execute(pool)
            .await
            .ok();
            
        sqlx::query("ALTER TABLE tasks RENAME TO tasks_old")
            .execute(pool)
            .await
            .ok();
            
        sqlx::query("ALTER TABLE tasks_new RENAME TO tasks")
            .execute(pool)
            .await
            .ok();
            
        // Recreate indexes for the new table
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_tasks_status ON tasks(status)")
            .execute(pool)
            .await
            .ok();
        
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_tasks_parent_id ON tasks(parent_id)")
            .execute(pool)
            .await
            .ok();
        
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_tasks_due_date ON tasks(due_date)")
            .execute(pool)
            .await
            .ok();
            
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_tasks_notification_type ON tasks(notification_type)")
            .execute(pool)
            .await
            .ok();
        
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_tasks_status_notification ON tasks(status, notification_type)")
            .execute(pool)
            .await
            .ok();
    } else {
        // Priority column doesn't exist, clean up the temporary table
        sqlx::query("DROP TABLE IF EXISTS tasks_new")
            .execute(pool)
            .await
            .ok();
    }
    
    Ok(())
}