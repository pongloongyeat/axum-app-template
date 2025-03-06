use schemars::JsonSchema;
use serde::Deserialize;
use serde_valid::Validate;

use crate::core::validators;

#[derive(Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RequestOtpRequest {
    pub email: String,
}

#[derive(Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct VerifyOtpRequest {
    pub email: String,
    pub otp: String,
}

#[derive(Deserialize, Validate, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ResetPasswordRequest {
    pub token: String,

    #[validate(custom = validators::is_password_valid)]
    pub password: String,
}
