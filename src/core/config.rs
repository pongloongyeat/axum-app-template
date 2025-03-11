use std::{
    env,
    fmt::{Debug, Display},
    ops::Range,
    str::FromStr,
};

use chrono::Duration;
use strum::{Display, EnumIs, EnumString};
use tracing::log::LevelFilter;

#[derive(Clone)]
pub struct AppConfig {
    pub host: String,
    pub port: String,
    pub db_url: String,
    pub sql_log_level: LevelFilter,
    pub status_code_range_for_error_logging: Range<u16>,
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
        dotenvy::dotenv().expect("Failed to initialise .env");

        let host = get_env("HOST");
        let port = get_env("PORT");

        let db_url = get_env("DATABASE_URL");
        let sql_log_level = get_env("SQL_LOG_LEVEL");

        let status_code_range_for_error_logging = get_range("STATUS_CODE_RANGE_FOR_ERROR_LOGGING");

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
            status_code_range_for_error_logging,
            session_duration,
            session_refresh_duration,
            otp_validity_duration,
        }
    }
}

fn parse<T, R>(value: T) -> R
where
    T: Into<String> + Clone + Display,
    R: FromStr,
    R::Err: Debug,
{
    let value: String = value.into();
    value.parse().expect(
        format!(
            "Failed to parse {} into type {}",
            value,
            std::any::type_name::<T>()
        )
        .as_str(),
    )
}

fn get_env<T>(key: &str) -> T
where
    T: FromStr,
    T::Err: Debug,
{
    let value = env::var(key).expect(format!("Missing {key} in environment.").as_str());
    parse(value)
}

fn get_range<T>(key: &str) -> Range<T>
where
    T: FromStr,
    T::Err: Debug,
{
    let value = get_env::<String>(key);
    let (start, end) = value.split_once("..").expect(
        format!("Wrong format for range. Expected {{start}}..{{end}}, got: {value}").as_str(),
    );

    let start = parse(start);
    let end = parse(end);

    Range { start, end }
}
