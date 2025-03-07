use schemars::JsonSchema;
use serde::Deserialize;

use crate::core::validators::{self, Validatable};

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

#[derive(Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ResetPasswordRequest {
    pub token: String,

    #[validate(custom = validators::is_password_valid)]
    pub password: String,
}

impl Validatable for ResetPasswordRequest {
    fn validated_properties() -> Vec<String> {
        vec!["password".into()]
    }

    fn validate_property(&self, property: &str) -> Option<Vec<String>> {
        match property {
            "password" => validators::is_password_valid(&self.password),
            _ => None,
        }
    }
}
