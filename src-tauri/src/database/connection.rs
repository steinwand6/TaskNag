use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use tauri::{AppHandle, Manager};

#[derive(Clone)]
pub struct Database {
    pub pool: Pool<Sqlite>,
}

impl Database {
    pub async fn new(app_handle: &AppHandle) -> Result<Self, sqlx::Error> {
        log::info!("Initializing database connection");
        
        let app_dir = app_handle
            .path()
            .app_data_dir()
            .expect("Failed to get app data dir");
        
        log::info!("App data directory: {}", app_dir.display());
        
        // Ensure directory exists
        std::fs::create_dir_all(&app_dir)
            .map_err(|e| {
                log::error!("Failed to create app data directory: {}", e);
                sqlx::Error::Io(e)
            })?;
        
        let db_path = app_dir.join("tasknag.db");
        let db_url = format!("sqlite:{}?mode=rwc", db_path.display());
        log::info!("Database URL: {}", db_url);
        
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&db_url)
            .await
            .map_err(|e| {
                log::error!("Failed to connect to database: {}", e);
                e
            })?;
        
        log::info!("Database connection established successfully");
        
        // FOREIGN KEY制約を有効化（デバッグ用に一時的に確認）
        sqlx::query("PRAGMA foreign_keys = ON")
            .execute(&pool)
            .await
            .map_err(|e| {
                log::error!("Failed to enable foreign keys: {}", e);
                e
            })?;
        
        log::info!("Foreign key constraints enabled");
        
        let db = Self { pool };
        
        // Run migrations manually since we're not using sqlx migrate macro
        log::info!("Running database migrations");
        crate::database::migrations::run_migrations(&db.pool).await
            .map_err(|e| {
                log::error!("Database migration failed: {}", e);
                e
            })?;
        
        log::info!("Database initialization completed successfully");
        Ok(db)
    }

    /// Create a placeholder Database for testing (requires a real pool to be set later)
    pub fn new_placeholder() -> Self {
        // Create a dummy pool that will be replaced in real usage
        // This is only for Default trait implementation and testing
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect_lazy("sqlite::memory:")
            .unwrap();
        
        Self { pool }
    }
}