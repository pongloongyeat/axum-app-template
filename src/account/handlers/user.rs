use axum::extract::State;

use crate::{
    account::{models::response::user::UserResponse, utils::extractors::CurrentUser},
    utils::extractors::AppJson,
    AppState,
};

#[axum::debug_handler]
pub async fn get_current_user(
    State(_): State<AppState>,
    CurrentUser(user): CurrentUser,
) -> AppJson<UserResponse> {
    AppJson(user.into())
}
