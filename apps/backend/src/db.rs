use sqlx::any::{AnyConnectOptions, AnyPoolOptions};
use sqlx::AnyPool;
use std::env;
use std::path::Path;
use std::str::FromStr;

/// Ensures the SQLite database file and its parent directory exist before connecting.
fn ensure_sqlite_database_exists(db_url: &str) -> Result<(), std::io::Error> {
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

pub async fn init_db() -> Result<AnyPool, sqlx::Error> {
    // Required when using sqlx::any: install compiled-in drivers (sqlite, postgres) so
    // AnyConnectOptions::from_str() can connect. Safe to call multiple times.
    sqlx::any::install_default_drivers();

    let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
        let data_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data");
        format!("sqlite://{}/pms.db", data_dir.display())
    });

    if db_url.starts_with("sqlite:") {
        ensure_sqlite_database_exists(&db_url).map_err(sqlx::Error::Io)?;
    }

    let options = AnyConnectOptions::from_str(&db_url)?;
    let pool = AnyPoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;
    Ok(pool)
}
