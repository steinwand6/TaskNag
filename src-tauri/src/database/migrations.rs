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
    
    Ok(())
}