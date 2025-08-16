use sqlx::{Pool, Sqlite};

pub async fn run_migrations(pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
    // Run migrations from SQL files
    sqlx::migrate!("./migrations")
        .run(pool)
        .await?;
    
    Ok(())
}