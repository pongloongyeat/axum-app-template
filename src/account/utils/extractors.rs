use aide::OperationInput;
use axum::extract::FromRequestParts;

use crate::{
    account::{
        database::{session_repository, user_repository},
        entities::{
            session::SessionEntity,
            user::{UserEntity, UserRole},
        },
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

pub trait Role {
    fn get_self() -> Self;

    fn role() -> UserRole;
}

pub struct CurrentRole<T: Role>(pub T);

impl<T> FromRequestParts<AppState> for CurrentRole<T>
where
    T: Role,
{
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

        let Some(user) = user_repository::find_user_by_token(&mut connection, token).await? else {
            return Err(AppError::AccountError(AccountError::InvalidOrExpiredToken));
        };

        let role = user.role;
        if role == T::role() {
            Ok(CurrentRole(T::get_self()))
        } else {
            Err(AppError::AccountError(AccountError::InsufficientPrivilege))
        }
    }
}

impl<T> OperationInput for CurrentRole<T> where T: Role {}

pub struct User;

impl Role for User {
    fn get_self() -> Self {
        User
    }

    fn role() -> UserRole {
        UserRole::User
    }
}
pub struct Admin;

impl Role for Admin {
    fn get_self() -> Self {
        Admin
    }

    fn role() -> UserRole {
        UserRole::Admin
    }
}
