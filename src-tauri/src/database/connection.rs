use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use tauri::{AppHandle, Manager};

pub struct Database {
    pub pool: Pool<Sqlite>,
}

impl Database {
    pub async fn new(app_handle: &AppHandle) -> Result<Self, sqlx::Error> {
        let app_dir = app_handle
            .path()
            .app_data_dir()
            .expect("Failed to get app data dir");
        
        // Ensure directory exists
        std::fs::create_dir_all(&app_dir).ok();
        
        let db_path = app_dir.join("tasknag.db");
        let db_url = format!("sqlite:{}?mode=rwc", db_path.display());
        
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&db_url)
            .await?;
        
        let db = Self { pool };
        
        // Run migrations manually since we're not using sqlx migrate macro
        crate::database::migrations::run_migrations(&db.pool).await?;
        
        Ok(db)
    }
}