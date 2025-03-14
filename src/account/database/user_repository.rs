use sqlx::{Sqlite, SqliteConnection};

use crate::{
    account::entities::user::{CreateUserEntity, FailedLoginAttempt, UserEntity, UserRole},
    core::{
        error::{ApiError, ApiResult},
        models::{Page, PageRequest},
        types::DbDateTime,
    },
};

pub async fn find_user_by_id(
    connection: &mut SqliteConnection,
    id: i64,
) -> ApiResult<Option<UserEntity>> {
    sqlx::query_as!(
        UserEntity,
        r#"
        SELECT
            u.id,
            u.email,
            u.password,
            u.role as "role: UserRole",
            u.login_attempts,
            u.last_failed_login_attempt as "last_failed_login_attempt!: Option<DbDateTime>"
        FROM users u
        WHERE u.id = ?
        "#,
        id
    )
    .fetch_optional(connection)
    .await
    .map_err(ApiError::from)
}

pub async fn find_user_by_email(
    connection: &mut SqliteConnection,
    email: &str,
) -> ApiResult<Option<UserEntity>> {
    sqlx::query_as!(
        UserEntity,
        r#"
        SELECT
            u.id,
            u.email,
            u.password,
            u.role as "role: UserRole",
            u.login_attempts,
            u.last_failed_login_attempt as "last_failed_login_attempt!: Option<DbDateTime>"
        FROM users u
        WHERE u.email = ?
        "#,
        email
    )
    .fetch_optional(connection)
    .await
    .map_err(ApiError::from)
}

pub async fn find_user_by_token(
    connection: &mut SqliteConnection,
    token: &str,
) -> ApiResult<Option<UserEntity>> {
    let now = DbDateTime::now();
    sqlx::query_as!(
        UserEntity,
        r#"
        SELECT
            u.id,
            u.email,
            u.password,
            u.role as "role: UserRole",
            u.login_attempts,
            u.last_failed_login_attempt as "last_failed_login_attempt!: Option<DbDateTime>"
        FROM sessions s
        INNER JOIN users u ON u.id = s.user_id
        WHERE s.token = ?
          AND s.token_expiry > ?
          AND s.revoked_at IS NULL
        "#,
        token,
        now,
    )
    .fetch_optional(connection)
    .await
    .map_err(ApiError::from)
}

pub async fn find_paginated_users(
    connection: &mut SqliteConnection,
    request: PageRequest,
) -> ApiResult<Page<UserEntity>> {
    let sql = request.to_sql_string(
        r#"
        SELECT
            u.id,
            u.email,
            u.password,
            u.role,
            u.login_attempts,
            u.last_failed_login_attempt
        FROM users u
        "#,
    );

    let users = sqlx::query_as::<Sqlite, UserEntity>(&sql)
        .fetch_all(&mut *connection)
        .await
        .map_err(ApiError::from)?;

    let count = sqlx::query!("SELECT COUNT(1) as count FROM users")
        .fetch_one(connection)
        .await
        .map(|result| result.count as u64)
        .map_err(ApiError::from)?;

    Ok(Page::new(users, count, request))
}

pub async fn user_exists_by_email(
    connection: &mut SqliteConnection,
    email: &str,
) -> ApiResult<bool> {
    sqlx::query!(
        "SELECT COUNT(1) as count FROM users u WHERE u.email = ?",
        email
    )
    .fetch_one(connection)
    .await
    .map(|result| result.count > 0)
    .map_err(ApiError::from)
}

pub async fn create_user(
    connection: &mut SqliteConnection,
    user: CreateUserEntity,
) -> ApiResult<UserEntity> {
    sqlx::query_as!(
        UserEntity,
        r#"
        INSERT INTO users (email, password)
        VALUES (?, ?)
        RETURNING
            id,
            email,
            password,
            role as "role!: UserRole",
            login_attempts,
            last_failed_login_attempt as "last_failed_login_attempt!: Option<DbDateTime>"
        "#,
        user.email,
        user.password
    )
    .fetch_one(connection)
    .await
    .map_err(ApiError::from)
}

pub async fn update_failed_login(
    connection: &mut SqliteConnection,
    id: i64,
    failed_login_attempt: FailedLoginAttempt,
) -> ApiResult<()> {
    sqlx::query!(
        "
        UPDATE users
        SET login_attempts = ?, last_failed_login_attempt = ?
        WHERE id = ?
        ",
        failed_login_attempt.login_attempts,
        failed_login_attempt.last_failed_login_attempt,
        id
    )
    .execute(connection)
    .await
    .map(|_| ())
    .map_err(ApiError::from)
}

pub async fn update_password_by_user_id(
    connection: &mut SqliteConnection,
    user_id: i64,
    password: &str,
) -> ApiResult<()> {
    sqlx::query!(
        "UPDATE users SET password = ?, login_attempts = 0 WHERE id = ?",
        password,
        user_id
    )
    .execute(connection)
    .await
    .map(|_| ())
    .map_err(ApiError::from)
}
