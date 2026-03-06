use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;
use std::env;

pub async fn init_db() -> Result<SqlitePool, sqlx::Error> {
    let db_path = env::var("DATABASE_URL").unwrap_or_else(|_| {
        let data_dir = std::env::current_dir()
            .unwrap_or_else(|_| std::path::PathBuf::from("."))
            .join("data");
        std::fs::create_dir_all(&data_dir).ok();
        format!("sqlite:{}/pms.db", data_dir.display())
    });
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_path)
        .await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    Ok(pool)
}
