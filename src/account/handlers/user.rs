use axum::extract::State;

use crate::{
    account::{models::response::user::UserResponse, utils::extractors::CurrentUser},
    core::{extractors::JsonResponse, AppState},
};

#[axum::debug_handler]
pub async fn get_current_user(
    State(_): State<AppState>,
    CurrentUser(user): CurrentUser,
) -> JsonResponse<UserResponse> {
    JsonResponse(user.into())
}
