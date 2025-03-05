use std::{env, fmt::Debug, str::FromStr};

use chrono::Duration;
use strum::{Display, EnumIs, EnumString};
use tracing::log::LevelFilter;

#[derive(Clone)]
pub struct AppConfig {
    pub host: String,
    pub port: String,
    pub db_url: String,
    pub sql_log_level: LevelFilter,
    pub session_duration: Duration,
    pub session_refresh_duration: Duration,
    pub otp_validity_duration: Duration,
}

#[derive(EnumString, EnumIs, Display)]
pub enum AppEnv {
    Dev,
    Prod,
}

impl AppConfig {
    pub fn new() -> Self {
        fn get_env<T>(key: &str) -> T
        where
            T: FromStr,
            T::Err: Debug,
        {
            let value = env::var(key).expect(format!("Missing {key} in environment.").as_str());
            value.parse().expect(
                format!(
                    "Failed to parse {} into type {}",
                    value,
                    std::any::type_name::<T>()
                )
                .as_str(),
            )
        }

        dotenvy::dotenv().expect("Failed to initialise .env");

        let host = get_env("HOST");
        let port = get_env("PORT");

        let db_url = get_env("DATABASE_URL");
        let sql_log_level = get_env("SQL_LOG_LEVEL");

        let session_duration = get_env("SESSION_DURATION");
        let session_duration = Duration::seconds(session_duration);

        let session_refresh_duration = get_env("SESSION_REFRESH_DURATION");
        let session_refresh_duration = Duration::seconds(session_refresh_duration);

        let otp_validity_duration = get_env("OTP_VALIDITY_DURATION");
        let otp_validity_duration = Duration::seconds(otp_validity_duration);

        Self {
            host,
            port,
            db_url,
            sql_log_level,
            session_duration,
            session_refresh_duration,
            otp_validity_duration,
        }
    }
}
