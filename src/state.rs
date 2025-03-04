use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

use crate::AppConfig;

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub pool: SqlitePool,
}

impl AppState {
    pub async fn new(config: AppConfig) -> Self {
        let db_url = &config.db_url;
        let pool = SqlitePoolOptions::new()
            .connect(db_url)
            .await
            .expect(format!("Failed to connect to database at {db_url}").as_str());

        AppState { config, pool }
    }
}
