use aide::OperationInput;
use axum::extract::FromRequestParts;

use crate::{
    account::{
        database::{session_repository, user_repository},
        entities::{session::SessionEntity, user::UserEntity},
        error::AccountError,
    },
    core::{constants::session::headers::SESSION_HEADER_KEY, error::AppError, AppState},
};

pub struct PossiblyExpiredSession(pub SessionEntity);

impl FromRequestParts<AppState> for PossiblyExpiredSession {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let Some(session_id) = parts.headers.get(SESSION_HEADER_KEY) else {
            return Err(AppError::AccountError(AccountError::MissingTokenInHeader));
        };

        let token = session_id.to_str().map_err(AppError::from)?;
        let mut connection = state.pool.acquire().await.map_err(AppError::from)?;

        if let Some(session) =
            session_repository::find_session_by_token_ignoring_expiration(&mut connection, token)
                .await?
        {
            Ok(Self(session))
        } else {
            Err(AppError::AccountError(AccountError::InvalidOrExpiredToken))
        }
    }
}

impl OperationInput for PossiblyExpiredSession {}

pub struct CurrentUser(pub UserEntity);

impl FromRequestParts<AppState> for CurrentUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let Some(session_id) = parts.headers.get(SESSION_HEADER_KEY) else {
            return Err(AppError::AccountError(AccountError::MissingTokenInHeader));
        };

        let token = session_id.to_str().map_err(AppError::from)?;
        let mut connection = state.pool.acquire().await.map_err(AppError::from)?;

        if let Some(user) = user_repository::find_user_by_token(&mut connection, token).await? {
            Ok(Self(user))
        } else {
            Err(AppError::AccountError(AccountError::InvalidOrExpiredToken))
        }
    }
}

impl OperationInput for CurrentUser {}
