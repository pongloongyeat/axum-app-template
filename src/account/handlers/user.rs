use axum::extract::State;

use crate::{
    account::{models::response::user::UserResponse, utils::extractors::CurrentUser},
    core::{extractors::JsonResponse, AppState},
};

pub async fn get_current_user(
    State(_): State<AppState>,
    CurrentUser(user): CurrentUser,
) -> JsonResponse<UserResponse> {
    JsonResponse(user.into())
}

pub mod admin {
    use axum::extract::State;
    use axum_extra::extract::Query;

    use crate::{
        account::{
            database::user_repository,
            models::response::user::UserResponse,
            utils::extractors::{Admin, CurrentRole},
        },
        core::{
            error::{ApiError, ApiResult},
            extractors::JsonResponse,
            models::{Page, PageRequest},
            AppState,
        },
    };

    #[axum::debug_handler]
    pub async fn get_paginated_users(
        State(state): State<AppState>,
        CurrentRole(_): CurrentRole<Admin>,
        Query(request): Query<PageRequest>,
    ) -> ApiResult<JsonResponse<Page<UserResponse>>> {
        let mut connection = state.pool.acquire().await.map_err(ApiError::from)?;
        user_repository::find_paginated_users(&mut connection, request)
            .await
            .map(|users| users.map(|user| user.to_owned().into()))
            .map(JsonResponse)
    }
}
