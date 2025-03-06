use aide::{OperationInput, OperationOutput};
use axum::{extract::FromRequest, response::IntoResponse, Json};
use schemars::JsonSchema;
use serde::{de::DeserializeOwned, Serialize};
use serde_valid::Validate;

use super::error::AppError;

pub struct JsonRequest<T>(pub T);

impl<T, S> FromRequest<S> for JsonRequest<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: axum::extract::Request, state: &S) -> Result<Self, Self::Rejection> {
        <Json<T> as FromRequest<S>>::from_request(req, state)
            .await
            .map(|json| JsonRequest(json.0))
            .map_err(AppError::from)
    }
}

impl<T> OperationInput for JsonRequest<T>
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

pub struct ValidJsonRequest<T>(pub T);

impl<T, S> FromRequest<S> for ValidJsonRequest<T>
where
    T: Validate + DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: axum::extract::Request, state: &S) -> Result<Self, Self::Rejection> {
        let json = <Json<T> as FromRequest<S>>::from_request(req, state)
            .await
            .map(|json| json.0)
            .map_err(AppError::from)?;
        json.validate().map_err(AppError::from)?;

        Ok(ValidJsonRequest(json))
    }
}

impl<T> OperationInput for ValidJsonRequest<T>
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

pub struct JsonResponse<T>(pub T);

impl<T> IntoResponse for JsonResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        Json(self.0).into_response()
    }
}

impl<T> OperationOutput for JsonResponse<T>
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
