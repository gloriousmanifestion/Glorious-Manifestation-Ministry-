use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;
use std::env;
use std::path::Path;

/// Ensures the SQLite database file and its parent directory exist before connecting.
/// sqlx does not auto-create the file like rusqlite.
fn ensure_database_exists(db_url: &str) -> Result<(), std::io::Error> {
    let path_str = db_url
        .strip_prefix("sqlite://")
        .or_else(|| db_url.strip_prefix("sqlite:"))
        .unwrap_or(db_url);
    let db_path = Path::new(path_str);

    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    if !db_path.exists() {
        std::fs::File::create(db_path)?;
    }
    Ok(())
}

pub async fn init_db() -> Result<SqlitePool, sqlx::Error> {
    let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
        let data_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data");
        format!("sqlite://{}/pms.db", data_dir.display())
    });
    ensure_database_exists(&db_url).map_err(sqlx::Error::Io)?;
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    Ok(pool)
}
