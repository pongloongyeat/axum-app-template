use crate::core::types::DbDateTime;

pub struct UserEntity {
    pub id: i64,
    pub email: String,
    pub password: String,
    pub login_attempts: i64,
    pub last_failed_login_attempt: Option<DbDateTime>,
}

pub struct CreateUserEntity {
    pub email: String,
    pub password: String,
}

pub struct FailedLoginAttempt {
    pub login_attempts: i64,
    pub last_failed_login_attempt: Option<DbDateTime>,
}
