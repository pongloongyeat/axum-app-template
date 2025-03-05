use schemars::JsonSchema;
use serde::Deserialize;
use serde_valid::Validate;

use crate::core::validators;

#[derive(Deserialize, Validate, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticateRequest {
    #[validate(custom = validators::is_email_valid)]
    pub email: String,

    #[validate(custom = validators::is_password_valid)]
    pub password: String,
}

#[derive(Deserialize, Validate, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserRequest {
    #[validate(custom = validators::is_email_valid)]
    pub email: String,

    #[validate(custom = validators::is_password_valid)]
    pub password: String,
}

#[derive(Deserialize, Validate, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ExtendSessionRequest {
    #[validate(min_length = 36)]
    #[validate(max_length = 36)]
    pub refresh_token: String,
}
