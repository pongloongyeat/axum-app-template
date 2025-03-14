use axum::http::StatusCode;
use strum::EnumIs;
use thiserror::Error;

use crate::core::{
    constants::session::headers::SESSION_HEADER_KEY,
    error::{ApiErrorResponse, ErrorCode, IntoApiErrorResponse},
};

#[derive(Error, Debug, EnumIs)]
pub enum AccountError {
    #[error("User {0} already exists.")]
    UserExistsByEmail(String),

    #[error("User {0} does not exist.")]
    UserDoesNotExistByEmail(String),

    #[error("User {0} does not exist.")]
    UserDoesNotExistById(i64),

    #[error("Email or password is incorrect.")]
    InvalidCredentials,

    #[error("Account locked. You have entered an invalid password too many times.")]
    MaxLoginAttempts,

    #[error("Missing session token in header.")]
    MissingTokenInHeader,

    #[error("Session token is invalid or expired.")]
    InvalidOrExpiredToken,

    #[error("OTP is invalid or expired.")]
    InvalidOrExpiredOtp,

    #[error("Session and refresh tokens do not match.")]
    TokenPairMismatch,

    #[error("Insufficient privilege to access this resource.")]
    InsufficientPrivilege,
}

impl ErrorCode for AccountError {
    fn code(&self) -> &str {
        match self {
            AccountError::UserExistsByEmail(_) => "ACC0001",
            AccountError::UserDoesNotExistByEmail(_) => "ACC0002",
            AccountError::UserDoesNotExistById(_) => "ACC0003",
            AccountError::InvalidCredentials => "ACC0004",
            AccountError::MaxLoginAttempts => "ACC0005",
            AccountError::MissingTokenInHeader => "ACC0006",
            AccountError::InvalidOrExpiredToken => "ACC0007",
            AccountError::InvalidOrExpiredOtp => "ACC0008",
            AccountError::TokenPairMismatch => "ACC0009",
            AccountError::InsufficientPrivilege => "ACC0010",
        }
    }
}

impl IntoApiErrorResponse for AccountError {
    fn into_app_error_response(&self) -> ApiErrorResponse {
        match &self {
            AccountError::UserExistsByEmail(_)
            | AccountError::InvalidCredentials
            | AccountError::MaxLoginAttempts
            | AccountError::InvalidOrExpiredOtp => ApiErrorResponse {
                status_code: StatusCode::BAD_REQUEST,
                code: self.code().into(),
                message: self.to_string(),
                debug_description: None,
                validation_errors: vec![],
            },
            AccountError::UserDoesNotExistByEmail(_) | AccountError::UserDoesNotExistById(_) => {
                ApiErrorResponse {
                    status_code: StatusCode::NOT_FOUND,
                    code: self.code().into(),
                    message: self.to_string(),
                    debug_description: None,
                    validation_errors: vec![],
                }
            }
            AccountError::MissingTokenInHeader => ApiErrorResponse {
                status_code: StatusCode::UNAUTHORIZED,
                code: self.code().into(),
                message: self.to_string(),
                debug_description: Some(format!(
                    "Missing token ({}) in header.",
                    SESSION_HEADER_KEY
                )),
                validation_errors: vec![],
            },
            AccountError::InvalidOrExpiredToken => ApiErrorResponse {
                status_code: StatusCode::UNAUTHORIZED,
                code: self.code().into(),
                message: self.to_string(),
                debug_description: None,
                validation_errors: vec![],
            },
            AccountError::TokenPairMismatch => ApiErrorResponse {
                status_code: StatusCode::UNAUTHORIZED,
                code: self.code().into(),
                message: self.to_string(),
                debug_description: None,
                validation_errors: vec![],
            },
            AccountError::InsufficientPrivilege => ApiErrorResponse {
                status_code: StatusCode::FORBIDDEN,
                code: self.code().into(),
                message: self.to_string(),
                debug_description: None,
                validation_errors: vec![],
            },
        }
    }
}
