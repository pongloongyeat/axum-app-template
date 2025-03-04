use aide::axum::{
    routing::{get, post},
    ApiRouter,
};

use crate::core::{
    constants::openapi::{
        tags::{AUTH_TAG, FORGOT_PASSWORD_TAG, USER_TAG},
        DEFAULT_SECURITY_SCHEME,
    },
    AppState,
};

use super::handlers;

pub fn router(state: AppState) -> ApiRouter {
    ApiRouter::new().nest(
        "/account",
        ApiRouter::new()
            .nest(
                "/auth",
                ApiRouter::new()
                    .api_route_with("/register", post(handlers::auth::register), |op| {
                        op.tag(AUTH_TAG)
                    })
                    .api_route_with("/login", post(handlers::auth::login), |op| op.tag(AUTH_TAG))
                    .api_route_with("/extend", post(handlers::auth::extend_session), |op| {
                        op.tag(AUTH_TAG)
                            .security_requirement(DEFAULT_SECURITY_SCHEME)
                    })
                    .api_route_with("/logout", post(handlers::auth::logout), |op| {
                        op.tag(AUTH_TAG)
                            .security_requirement(DEFAULT_SECURITY_SCHEME)
                    }),
            )
            .nest(
                "/forgot-password",
                ApiRouter::new()
                    .api_route_with(
                        "/request-otp",
                        post(handlers::forgot_password::request_otp),
                        |op| op.tag(FORGOT_PASSWORD_TAG),
                    )
                    .api_route_with(
                        "/verify-otp",
                        post(handlers::forgot_password::verify_otp),
                        |op| op.tag(FORGOT_PASSWORD_TAG),
                    )
                    .api_route_with(
                        "/reset",
                        post(handlers::forgot_password::reset_password),
                        |op| op.tag(FORGOT_PASSWORD_TAG),
                    ),
            )
            .nest(
                "/users",
                ApiRouter::new().api_route_with(
                    "/me",
                    get(handlers::user::get_current_user),
                    |op| {
                        op.tag(USER_TAG)
                            .security_requirement(DEFAULT_SECURITY_SCHEME)
                    },
                ),
            )
            .with_state(state.clone()),
    )
}
