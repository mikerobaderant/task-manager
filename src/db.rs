use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

const DATABASE_URL: &str = "sqlite:tasks.db?mode=rwc";

pub async fn init_pool() -> Result<SqlitePool, sqlx::Error> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(DATABASE_URL)
        .await?;

    // Create the tasks table if it doesn't exist
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            completed BOOLEAN DEFAULT FALSE,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(&pool)
    .await?;

    println!("Database initialized successfully");
    Ok(pool)
}

