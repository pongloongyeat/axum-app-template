use sqlx::SqliteConnection;

use crate::{
    account::entities::session::{CreateSessionEntity, RevocationReason, SessionEntity},
    core::{
        error::{ApiError, ApiResult},
        types::DbDateTime,
    },
};

pub async fn find_session_by_token_ignoring_expiration(
    connection: &mut SqliteConnection,
    token: &str,
) -> ApiResult<Option<SessionEntity>> {
    let now = DbDateTime::now();
    sqlx::query_as!(
        SessionEntity,
        r#"
        SELECT
            id,
            user_id,
            token,
            refresh_token,
            token_expiry,
            refresh_token_expiry,
            revoked_at as "revoked_at!: Option<DbDateTime>",
            revocation_reason as "revocation_reason!: Option<RevocationReason>"
        FROM sessions
        WHERE token = ?
        AND ? > refresh_token_expiry
        AND revoked_at IS NULL
        "#,
        token,
        now
    )
    .fetch_optional(connection)
    .await
    .map_err(ApiError::from)
}

pub async fn revoke_session_for_user_id(
    connection: &mut SqliteConnection,
    user_id: i64,
    reason: RevocationReason,
) -> ApiResult<()> {
    let now = DbDateTime::now();
    sqlx::query!(
        "
        UPDATE sessions
        SET
            revoked_at = ?,
            revocation_reason = ?
        WHERE user_id = ?
          AND revoked_at IS NULL
        ",
        now,
        reason,
        user_id
    )
    .execute(connection)
    .await
    .map(|_| ())
    .map_err(ApiError::from)
}

pub async fn create_session(
    connection: &mut SqliteConnection,
    user_id: i64,
    session: CreateSessionEntity,
) -> ApiResult<SessionEntity> {
    sqlx::query_as!(
        SessionEntity,
        r#"
        INSERT INTO sessions (
            user_id,
            token,
            refresh_token,
            token_expiry,
            refresh_token_expiry
        )
        VALUES (?, ?, ?, ?, ?)
        RETURNING
            id,
            user_id,
            token,
            refresh_token,
            token_expiry,
            refresh_token_expiry,
            revoked_at as "revoked_at!: Option<DbDateTime>",
            revocation_reason as "revocation_reason!: Option<RevocationReason>"
        "#,
        user_id,
        *session.token(),
        *session.refresh_token(),
        *session.token_expiry(),
        *session.refresh_token_expiry(),
    )
    .fetch_one(connection)
    .await
    .map_err(ApiError::from)
}
