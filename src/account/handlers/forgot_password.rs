use axum::{extract::State, response::NoContent};

use crate::{
    account::{
        database::{forgot_password_repository, user_repository},
        entities::forgot_password::{
            CreateForgotPasswordTransactionEntity, VerifyForgotPasswordTransaction,
        },
        error::AccountError,
        models::{
            request::forgot_password::{RequestOtpRequest, ResetPasswordRequest, VerifyOtpRequest},
            response::forgot_password::VerifyOtpResponse,
        },
    },
    core::{
        error::{AppError, AppResult},
        extractors::{JsonRequest, JsonResponse, ValidJsonRequest},
        AppState,
    },
};

#[axum::debug_handler]
pub async fn request_otp(
    State(state): State<AppState>,
    JsonRequest(request): JsonRequest<RequestOtpRequest>,
) -> AppResult<()> {
    let mut connection = state.pool.acquire().await.map_err(AppError::from)?;
    let email = request.email;
    let Some(user_id) = user_repository::find_user_by_email(&mut connection, &email)
        .await?
        .map(|u| u.id)
    else {
        return Err(AppError::AccountError(
            AccountError::UserDoesNotExistByEmail(email),
        ));
    };

    let transaction = CreateForgotPasswordTransactionEntity::new(state.config, user_id);
    forgot_password_repository::request_otp(&mut connection, transaction).await?;

    Ok(())
}

#[axum::debug_handler]
pub async fn verify_otp(
    State(state): State<AppState>,
    JsonRequest(request): JsonRequest<VerifyOtpRequest>,
) -> AppResult<JsonResponse<VerifyOtpResponse>> {
    let email = request.email;
    let token = request.otp;

    let mut connection = state.pool.acquire().await.map_err(AppError::from)?;
    let Some(user_id) = user_repository::find_user_by_email(&mut connection, &email)
        .await?
        .map(|u| u.id)
    else {
        return Err(AppError::AccountError(
            AccountError::UserDoesNotExistByEmail(email),
        ));
    };

    let transaction = VerifyForgotPasswordTransaction { user_id, token };
    if let Some(verification_token) =
        forgot_password_repository::verify_otp_request(&mut connection, transaction).await?
    {
        Ok(JsonResponse(VerifyOtpResponse {
            token: verification_token,
        }))
    } else {
        Err(AppError::AccountError(AccountError::InvalidOrExpiredOtp))
    }
}

#[axum::debug_handler]
pub async fn reset_password(
    State(state): State<AppState>,
    ValidJsonRequest(request): ValidJsonRequest<ResetPasswordRequest>,
) -> AppResult<NoContent> {
    let token = request.token;
    let password = request.password;
    let hash = crate::account::utils::hash_password(&password)?;

    let mut connection = state.pool.begin().await.map_err(AppError::from)?;

    if let Some(user_id) =
        forgot_password_repository::consume_otp_request(&mut connection, &token).await?
    {
        user_repository::update_password_by_user_id(&mut connection, user_id, &hash).await?;
        connection.commit().await.map_err(AppError::from)?;
        Ok(NoContent)
    } else {
        connection.commit().await.map_err(AppError::from)?;
        Err(AppError::AccountError(AccountError::InvalidOrExpiredOtp))
    }
}
