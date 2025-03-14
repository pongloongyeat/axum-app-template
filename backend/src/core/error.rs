use aide::OperationOutput;
use axum::{http::StatusCode, response::IntoResponse, Json};
use schemars::JsonSchema;
use serde::Serialize;
use thiserror::Error;

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Error, Debug)]
pub enum ApiError {
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

impl From<argon2::password_hash::Error> for ApiError {
    fn from(value: argon2::password_hash::Error) -> Self {
        ApiError::Argon2PasswordHashError(value)
    }
}

impl From<Vec<crate::core::validators::ValidationError>> for ApiError {
    fn from(value: Vec<crate::core::validators::ValidationError>) -> Self {
        ApiError::ValidationError(value)
    }
}

impl ErrorCode for ApiError {
    fn code(&self) -> &str {
        match self {
            ApiError::SqlxError(_) => "GBL9999",
            ApiError::Argon2PasswordHashError(_) => "GBL9998",
            ApiError::HeaderToStrError(_) => "GBL0001",
            ApiError::JsonDeserializeError(_) => "GBL0002",
            ApiError::ValidationError(_) => "GBL0003",
            ApiError::AccountError(error) => error.code(),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        match &self {
            ApiError::SqlxError(error) => ApiErrorResponse {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: self.code().into(),
                message: self.to_string(),
                debug_description: Some(error.to_string()),
                validation_errors: vec![],
            },
            ApiError::Argon2PasswordHashError(error) => ApiErrorResponse {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: self.code().into(),
                message: self.to_string(),
                debug_description: Some(error.to_string()),
                validation_errors: vec![],
            },
            ApiError::HeaderToStrError(error) => ApiErrorResponse {
                status_code: StatusCode::BAD_REQUEST,
                code: self.code().into(),
                message: self.to_string(),
                debug_description: Some(error.to_string()),
                validation_errors: vec![],
            },
            ApiError::JsonDeserializeError(error) => ApiErrorResponse {
                status_code: error.status(),
                code: self.code().into(),
                message: self.to_string(),
                debug_description: Some(error.to_string()),
                validation_errors: vec![],
            },
            ApiError::ValidationError(errors) => ApiErrorResponse {
                status_code: StatusCode::BAD_REQUEST,
                code: self.code().into(),
                message: self.to_string(),
                debug_description: None,
                validation_errors: AppValidationErrorResponse::from_errors(errors),
            },
            ApiError::AccountError(error) => error.into_app_error_response(),
        }
        .into_response()
    }
}

impl OperationOutput for ApiError {
    type Inner = Self;
}

pub trait IntoApiErrorResponse {
    fn into_app_error_response(&self) -> ApiErrorResponse;
}

#[derive(Serialize, JsonSchema)]
pub struct ApiErrorResponse {
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

impl IntoResponse for ApiErrorResponse {
    fn into_response(self) -> axum::response::Response {
        (self.status_code, Json(self)).into_response()
    }
}
