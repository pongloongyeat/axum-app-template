use axum::{extract::State, response::NoContent};

use crate::{
    account::{
        database::{session_repository, user_repository},
        entities::{
            session::{CreateSessionEntity, RevocationReason},
            user::{CreateUserEntity, FailedLoginAttempt},
        },
        error::AccountError,
        models::{
            request::auth::{AuthenticateRequest, CreateUserRequest, ExtendSessionRequest},
            response::{auth::AuthenticatedResponse, user::UserResponse},
        },
        utils::extractors::{CurrentUser, PossiblyExpiredSession},
    },
    core::{
        error::{ApiError, ApiResult},
        extractors::{JsonRequest, JsonResponse, ValidJsonRequest},
        types::DbDateTime,
        AppState,
    },
};

#[axum::debug_handler]
pub async fn register(
    State(state): State<AppState>,
    ValidJsonRequest(request): ValidJsonRequest<CreateUserRequest>,
) -> ApiResult<JsonResponse<UserResponse>> {
    let email = request.email;
    let mut connection = state.pool.acquire().await.map_err(ApiError::from)?;

    if user_repository::user_exists_by_email(&mut connection, &email).await? {
        return Err(ApiError::AccountError(AccountError::UserExistsByEmail(
            email,
        )));
    }

    let password = request.password;
    let hash = crate::account::utils::hash_password(&password)?;

    let user = CreateUserEntity {
        email,
        password: hash,
    };

    user_repository::create_user(&mut connection, user)
        .await
        .map(UserResponse::from)
        .map(JsonResponse)
}

#[axum::debug_handler]
pub async fn login(
    State(state): State<AppState>,
    ValidJsonRequest(request): ValidJsonRequest<AuthenticateRequest>,
) -> ApiResult<JsonResponse<AuthenticatedResponse>> {
    let mut connection = state.pool.acquire().await.map_err(ApiError::from)?;
    let email = request.email;

    let Some(user) = user_repository::find_user_by_email(&mut connection, &email).await? else {
        return Err(ApiError::AccountError(
            AccountError::UserDoesNotExistByEmail(email),
        ));
    };

    validators::validate_max_login_attempts(&user)?;
    let result = validators::validate_user_password(&request.password, &user.password).await;

    if let Err(error) = result {
        if let ApiError::AccountError(account_error) = &error {
            if account_error.is_invalid_credentials() {
                let failed_login_attempt = FailedLoginAttempt {
                    login_attempts: user.login_attempts + 1,
                    last_failed_login_attempt: Some(DbDateTime::now()),
                };

                user_repository::update_failed_login(
                    &mut connection,
                    user.id,
                    failed_login_attempt,
                )
                .await?;
            }
        }

        return Err(error);
    }

    let new_session = CreateSessionEntity::new(state.config);
    let mut connection = state.pool.begin().await.map_err(ApiError::from)?;

    session_repository::revoke_session_for_user_id(
        &mut connection,
        user.id,
        RevocationReason::NewSession,
    )
    .await?;
    let session = session_repository::create_session(&mut connection, user.id, new_session).await?;

    connection.commit().await.map_err(ApiError::from)?;

    Ok(JsonResponse(AuthenticatedResponse {
        session_token: session.token,
        refresh_token: session.refresh_token,
        session_token_expiry: session.token_expiry.into(),
        refresh_token_expiry: session.refresh_token_expiry.into(),
        user: UserResponse::from(user),
    }))
}

#[axum::debug_handler]
pub async fn extend_session(
    State(state): State<AppState>,
    PossiblyExpiredSession(session): PossiblyExpiredSession,
    JsonRequest(request): JsonRequest<ExtendSessionRequest>,
) -> ApiResult<JsonResponse<AuthenticatedResponse>> {
    if session.refresh_token != request.refresh_token {
        return Err(ApiError::AccountError(AccountError::TokenPairMismatch));
    }

    let new_session = CreateSessionEntity::new(state.config);
    let mut connection = state.pool.begin().await.map_err(ApiError::from)?;
    let Some(user) = user_repository::find_user_by_id(&mut connection, session.user_id).await?
    else {
        return Err(ApiError::AccountError(AccountError::UserDoesNotExistById(
            session.user_id,
        )));
    };

    session_repository::revoke_session_for_user_id(
        &mut connection,
        session.user_id,
        RevocationReason::SessionExtended,
    )
    .await?;
    let session =
        session_repository::create_session(&mut connection, session.user_id, new_session).await?;

    connection.commit().await.map_err(ApiError::from)?;

    Ok(JsonResponse(AuthenticatedResponse {
        session_token: session.token,
        refresh_token: session.refresh_token,
        session_token_expiry: session.token_expiry.into(),
        refresh_token_expiry: session.refresh_token_expiry.into(),
        user: UserResponse::from(user),
    }))
}

#[axum::debug_handler]
pub async fn logout(
    State(state): State<AppState>,
    CurrentUser(user): CurrentUser,
) -> ApiResult<NoContent> {
    let mut connection = state.pool.acquire().await.map_err(ApiError::from)?;

    session_repository::revoke_session_for_user_id(
        &mut connection,
        user.id,
        RevocationReason::LoggedOut,
    )
    .await?;

    Ok(NoContent)
}

mod validators {
    use argon2::{Argon2, PasswordHash, PasswordVerifier};

    use crate::{
        account::{entities::user::UserEntity, error::AccountError},
        core::error::{ApiError, ApiResult},
    };

    pub async fn validate_user_password(password: &str, user_password: &str) -> ApiResult<()> {
        let argon2 = Argon2::default();
        let hash = PasswordHash::new(user_password).map_err(ApiError::from)?;

        argon2
            .verify_password(password.as_bytes(), &hash)
            .map_err(|error| match error {
                argon2::password_hash::Error::Password => {
                    ApiError::AccountError(AccountError::InvalidCredentials)
                }
                _ => ApiError::Argon2PasswordHashError(error),
            })?;

        Ok(())
    }

    pub fn validate_max_login_attempts(user: &UserEntity) -> ApiResult<()> {
        if user.login_attempts >= 3 {
            Err(ApiError::AccountError(AccountError::MaxLoginAttempts))
        } else {
            Ok(())
        }
    }
}
