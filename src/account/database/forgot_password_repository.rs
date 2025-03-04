use sqlx::SqliteConnection;

use crate::{
    account::entities::forgot_password::{
        CreateForgotPasswordTransactionEntity, VerifyForgotPasswordTransaction,
    },
    core::{
        error::{AppError, AppResult},
        types::DbDateTime,
    },
};

pub async fn request_otp(
    connection: &mut SqliteConnection,
    transaction: CreateForgotPasswordTransactionEntity,
) -> AppResult<String> {
    sqlx::query!(
        "
        INSERT INTO forgot_password_transactions
            (user_id, token, reset_password_token, expires_at)
        VALUES
            (?, ?, ?, ?)
        RETURNING token
        ",
        transaction.user_id,
        transaction.token,
        transaction.reset_password_token,
        transaction.expires_at,
    )
    .fetch_one(connection)
    .await
    .map(|result| result.token)
    .map_err(AppError::from)
}

pub async fn verify_otp_request(
    connection: &mut SqliteConnection,
    transaction: VerifyForgotPasswordTransaction,
) -> AppResult<Option<String>> {
    let now = DbDateTime::now();
    sqlx::query!(
        "
        UPDATE forgot_password_transactions
        SET verified_at = ?
        WHERE user_id = ?
          AND token = ?
          AND ? < expires_at
          AND verified_at IS NULL
          AND used_at IS NULL
        RETURNING reset_password_token
        ",
        now,
        transaction.user_id,
        transaction.token,
        now
    )
    .fetch_optional(connection)
    .await
    .map(|result| result.map(|result| result.reset_password_token))
    .map_err(AppError::from)
}

pub async fn consume_otp_request(
    connection: &mut SqliteConnection,
    token: &str,
) -> AppResult<Option<i64>> {
    let now = DbDateTime::now();
    sqlx::query!(
        "
        UPDATE forgot_password_transactions
        SET used_at = ?
        WHERE reset_password_token = ?
          AND ? < expires_at
          AND verified_at IS NOT NULL
          AND used_at IS NULL
        RETURNING user_id
        ",
        now,
        token,
        now
    )
    .fetch_optional(connection)
    .await
    .map(|result| result.map(|result| result.user_id))
    .map_err(AppError::from)
}
