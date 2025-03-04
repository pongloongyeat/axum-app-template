use chrono::Utc;
use uuid::Uuid;

use crate::{types::DbDateTime, AppConfig};

const DEFAULT_OTP_LENGTH: usize = 6;

pub struct CreateForgotPasswordTransactionEntity {
    pub user_id: i64,
    pub token: String,
    pub reset_password_token: String,
    pub expires_at: DbDateTime,
}

impl CreateForgotPasswordTransactionEntity {
    pub fn new(config: AppConfig, user_id: i64) -> Self {
        Self {
            user_id,
            token: crate::account::utils::generate_otp(DEFAULT_OTP_LENGTH),
            reset_password_token: Uuid::new_v4().to_string(),
            expires_at: (Utc::now() + config.otp_validity_duration).into(),
        }
    }
}

pub struct VerifyForgotPasswordTransaction {
    pub user_id: i64,
    pub token: String,
}
