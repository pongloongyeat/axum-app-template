use chrono::Utc;
use derive_getters::Getters;
use sqlx::prelude::Type;
use uuid::Uuid;

use crate::{types::DbDateTime, AppConfig};

#[derive(Type)]
pub enum RevocationReason {
    SessionExtended,
    LoggedOut,
    NewSession,
}

pub struct SessionEntity {
    pub id: i64,
    pub user_id: i64,
    pub token: String,
    pub refresh_token: String,
    pub token_expiry: DbDateTime,
    pub refresh_token_expiry: DbDateTime,
    pub revoked_at: Option<DbDateTime>,
    pub revocation_reason: Option<RevocationReason>,
}

#[derive(Getters)]
pub struct CreateSessionEntity {
    token: String,
    refresh_token: String,
    token_expiry: DbDateTime,
    refresh_token_expiry: DbDateTime,
}

impl CreateSessionEntity {
    pub fn new(config: AppConfig) -> Self {
        let now = Utc::now();

        Self {
            token: Uuid::new_v4().to_string(),
            refresh_token: Uuid::new_v4().to_string(),
            token_expiry: DbDateTime(now + config.session_duration),
            refresh_token_expiry: DbDateTime(now + config.session_refresh_duration),
        }
    }
}
