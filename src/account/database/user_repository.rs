use sqlx::SqliteConnection;

use crate::{
    account::entities::user::{CreateUserEntity, FailedLoginAttempt, UserEntity, UserRole},
    core::{
        error::{AppError, AppResult},
        models::{Page, PageRequest},
        types::DbDateTime,
    },
};

pub async fn find_user_by_id(
    connection: &mut SqliteConnection,
    id: i64,
) -> AppResult<Option<UserEntity>> {
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
    .map_err(AppError::from)
}

pub async fn find_user_by_email(
    connection: &mut SqliteConnection,
    email: &str,
) -> AppResult<Option<UserEntity>> {
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
    .map_err(AppError::from)
}

pub async fn find_user_by_token(
    connection: &mut SqliteConnection,
    token: &str,
) -> AppResult<Option<UserEntity>> {
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
    .map_err(AppError::from)
}

pub async fn find_paginated_users(
    connection: &mut SqliteConnection,
    request: PageRequest,
) -> AppResult<Page<UserEntity>> {
    let take = request.take as i64;
    let skip = request.skip as i64;
    let users = sqlx::query_as!(
        UserEntity,
        r#"
        SELECT
            u.id,
            u.email,
            u.password,
            u.role as "role!: UserRole",
            u.login_attempts,
            u.last_failed_login_attempt as "last_failed_login_attempt!: Option<DbDateTime>"
        FROM users u
        LIMIT ?
        OFFSET ?
        "#,
        take,
        skip,
    )
    .fetch_all(&mut *connection)
    .await
    .map_err(AppError::from)?;

    let count = sqlx::query!("SELECT COUNT(1) as count FROM users")
        .fetch_one(connection)
        .await
        .map(|result| result.count as u64)
        .map_err(AppError::from)?;

    Ok(Page {
        content: users,
        total: count,
        request,
    })
}

pub async fn user_exists_by_email(
    connection: &mut SqliteConnection,
    email: &str,
) -> AppResult<bool> {
    sqlx::query!(
        "SELECT COUNT(1) as count FROM users u WHERE u.email = ?",
        email
    )
    .fetch_one(connection)
    .await
    .map(|result| result.count > 0)
    .map_err(AppError::from)
}

pub async fn create_user(
    connection: &mut SqliteConnection,
    user: CreateUserEntity,
) -> AppResult<UserEntity> {
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
    .map_err(AppError::from)
}

pub async fn update_failed_login(
    connection: &mut SqliteConnection,
    id: i64,
    failed_login_attempt: FailedLoginAttempt,
) -> AppResult<()> {
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
    .map_err(AppError::from)
}

pub async fn update_password_by_user_id(
    connection: &mut SqliteConnection,
    user_id: i64,
    password: &str,
) -> AppResult<()> {
    sqlx::query!(
        "UPDATE users SET password = ?, login_attempts = 0 WHERE id = ?",
        password,
        user_id
    )
    .execute(connection)
    .await
    .map(|_| ())
    .map_err(AppError::from)
}
