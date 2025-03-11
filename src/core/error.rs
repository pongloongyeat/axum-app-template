use aide::OperationOutput;
use axum::{http::StatusCode, response::IntoResponse, Json};
use schemars::JsonSchema;
use serde::Serialize;
use thiserror::Error;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("An unknown error has occured.")]
    SqlxError(#[from] sqlx::error::Error),

    #[error("An unknown error has occured.")]
    Argon2PasswordHashError(argon2::password_hash::Error),

    #[error("An unknown error has occured.")]
    HeaderToStrError(#[from] axum::http::header::ToStrError),

    #[error(transparent)]
    JsonDeserializeError(#[from] axum::extract::rejection::JsonRejection),

    #[error("One or more validation errors has occured.")]
    ValidationError(Vec<crate::core::validators::ValidationError>),

    #[error(transparent)]
    AccountError(#[from] crate::account::error::AccountError),
}

pub trait ErrorCode {
    fn code(&self) -> &str;
}

impl From<argon2::password_hash::Error> for AppError {
    fn from(value: argon2::password_hash::Error) -> Self {
        AppError::Argon2PasswordHashError(value)
    }
}

impl From<Vec<crate::core::validators::ValidationError>> for AppError {
    fn from(value: Vec<crate::core::validators::ValidationError>) -> Self {
        AppError::ValidationError(value)
    }
}

impl ErrorCode for AppError {
    fn code(&self) -> &str {
        match self {
            AppError::SqlxError(_) => "GBL9999",
            AppError::Argon2PasswordHashError(_) => "GBL9998",
            AppError::HeaderToStrError(_) => "GBL0001",
            AppError::JsonDeserializeError(_) => "GBL0002",
            AppError::ValidationError(_) => "GBL0003",
            AppError::AccountError(error) => error.code(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match &self {
            AppError::SqlxError(error) => AppErrorResponse {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: self.code().into(),
                message: self.to_string(),
                debug_description: Some(error.to_string()),
                validation_errors: vec![],
            },
            AppError::Argon2PasswordHashError(error) => AppErrorResponse {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: self.code().into(),
                message: self.to_string(),
                debug_description: Some(error.to_string()),
                validation_errors: vec![],
            },
            AppError::HeaderToStrError(error) => AppErrorResponse {
                status_code: StatusCode::BAD_REQUEST,
                code: self.code().into(),
                message: self.to_string(),
                debug_description: Some(error.to_string()),
                validation_errors: vec![],
            },
            AppError::JsonDeserializeError(error) => AppErrorResponse {
                status_code: error.status(),
                code: self.code().into(),
                message: self.to_string(),
                debug_description: Some(error.to_string()),
                validation_errors: vec![],
            },
            AppError::ValidationError(errors) => AppErrorResponse {
                status_code: StatusCode::BAD_REQUEST,
                code: self.code().into(),
                message: self.to_string(),
                debug_description: None,
                validation_errors: AppValidationErrorResponse::from_errors(errors),
            },
            AppError::AccountError(error) => error.into_app_error_response(),
        }
        .into_response()
    }
}

impl OperationOutput for AppError {
    type Inner = Self;
}

pub trait IntoAppErrorResponse {
    fn into_app_error_response(&self) -> AppErrorResponse;
}

#[derive(Serialize, JsonSchema)]
pub struct AppErrorResponse {
    #[serde(skip_serializing)]
    #[schemars(skip)]
    pub status_code: StatusCode,
    pub code: String,
    pub message: String,

    #[serde(skip_serializing_if = "super::utils::is_prod")]
    pub debug_description: Option<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub validation_errors: Vec<AppValidationErrorResponse>,
}

#[derive(Serialize, JsonSchema)]
pub struct AppValidationErrorResponse {
    pub property: String,
    pub errors: Vec<String>,
}

impl From<crate::core::validators::ValidationError> for AppValidationErrorResponse {
    fn from(value: crate::core::validators::ValidationError) -> Self {
        Self {
            property: value.property,
            errors: value.errors,
        }
    }
}

impl AppValidationErrorResponse {
    pub fn from_errors(errors: &Vec<crate::core::validators::ValidationError>) -> Vec<Self> {
        errors.iter().map(|error| error.to_owned().into()).collect()
    }
}

impl IntoResponse for AppErrorResponse {
    fn into_response(self) -> axum::response::Response {
        (self.status_code, Json(self)).into_response()
    }
}
