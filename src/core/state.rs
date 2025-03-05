use std::str::FromStr;

use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    ConnectOptions, SqlitePool,
};

use super::AppConfig;

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub pool: SqlitePool,
}

impl AppState {
    pub async fn new(config: AppConfig) -> Self {
        let db_url = config.clone().db_url;
        let options = SqliteConnectOptions::from_str(&db_url)
            .unwrap()
            // TODO: It ain't working
            .log_statements(config.sql_log_level);

        let pool = SqlitePoolOptions::new()
            .connect_with(options)
            .await
            .expect(format!("Failed to connect to database at {db_url}").as_str());

        AppState { config, pool }
    }
}
