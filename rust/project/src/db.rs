use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};

pub async fn connect_db() -> Result<SqlitePool, sqlx::Error> {
    let database_url = "sqlite:src/bank.db";

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    println!("✅ Connected to SQLite database");

    Ok(pool)
}