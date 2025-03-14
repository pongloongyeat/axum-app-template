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
        let mut options = SqliteConnectOptions::from_str(&db_url)
            .unwrap()
            .log_statements(tracing::log::LevelFilter::Debug);

        if !config.show_sql {
            options = options.disable_statement_logging();
        }

        let pool = SqlitePoolOptions::new()
            .connect_with(options)
            .await
            .expect(format!("Failed to connect to database at {db_url}").as_str());

        AppState { config, pool }
    }
}
