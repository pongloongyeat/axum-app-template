use std::env;

use crate::AppEnv;

pub fn is_prod<T>(_: T) -> bool {
    env::var("ENV")
        .unwrap_or(AppEnv::Dev.to_string())
        .parse::<AppEnv>()
        .unwrap_or(AppEnv::Dev)
        .is_prod()
}

pub fn should_add_openapi_routes() -> bool {
    !is_prod(())
}

pub mod extractors {
    use aide::{OperationInput, OperationOutput};
    use axum::{extract::FromRequest, response::IntoResponse, Json};
    use schemars::JsonSchema;
    use serde::{de::DeserializeOwned, Serialize};

    use crate::error::AppError;

    pub struct AppJson<T>(pub T);

    impl<T, S> FromRequest<S> for AppJson<T>
    where
        T: DeserializeOwned,
        S: Send + Sync,
    {
        type Rejection = AppError;

        async fn from_request(
            req: axum::extract::Request,
            state: &S,
        ) -> Result<Self, Self::Rejection> {
            <Json<T> as FromRequest<S>>::from_request(req, state)
                .await
                .map(|json| AppJson(json.0))
                .map_err(AppError::from)
        }
    }

    impl<T> IntoResponse for AppJson<T>
    where
        T: Serialize,
    {
        fn into_response(self) -> axum::response::Response {
            Json(self.0).into_response()
        }
    }

    impl<T> OperationInput for AppJson<T>
    where
        T: JsonSchema,
    {
        fn operation_input(
            ctx: &mut aide::generate::GenContext,
            operation: &mut aide::openapi::Operation,
        ) {
            <Json<T> as OperationInput>::operation_input(ctx, operation);
        }
    }

    impl<T> OperationOutput for AppJson<T>
    where
        T: JsonSchema,
    {
        type Inner = T;

        fn operation_response(
            ctx: &mut aide::generate::GenContext,
            operation: &mut aide::openapi::Operation,
        ) -> Option<aide::openapi::Response> {
            <Json<T> as OperationOutput>::operation_response(ctx, operation)
        }

        fn inferred_responses(
            ctx: &mut aide::generate::GenContext,
            operation: &mut aide::openapi::Operation,
        ) -> Vec<(Option<u16>, aide::openapi::Response)> {
            <Json<T> as OperationOutput>::inferred_responses(ctx, operation)
        }
    }
}
