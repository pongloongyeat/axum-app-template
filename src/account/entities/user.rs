use sqlx::{FromRow, Type};

use crate::core::types::DbDateTime;

#[derive(PartialEq, Clone, Type)]
pub enum UserRole {
    User,
    Admin,
}

#[derive(FromRow, Clone)]
pub struct UserEntity {
    pub id: i64,
    pub email: String,
    pub password: String,
    pub role: UserRole,
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
