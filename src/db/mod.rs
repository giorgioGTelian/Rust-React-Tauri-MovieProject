// src/db/mod.rs
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

pub async fn init_db() -> Result<SqlitePool, sqlx::Error> {
    let db_url = "sqlite:media.db?mode=rwc";
    
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await?;
        
    // Esegue migrazioni se necessario
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;
        
    Ok(pool)
}
