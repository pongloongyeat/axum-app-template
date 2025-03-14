use schemars::JsonSchema;
use serde::Deserialize;
use shared::traits::Validatable;

use crate::core::validators::{self};

#[derive(Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserRequest {
    pub email: String,
    pub password: String,
}

impl Validatable for CreateUserRequest {
    fn validated_properties() -> Vec<String> {
        vec!["email".into(), "password".into()]
    }

    fn validate_property(&self, property: &str) -> Option<Vec<String>> {
        match property {
            "email" => validators::is_email_valid(&self.email),
            "password" => validators::is_password_valid(&self.password),
            _ => None,
        }
    }
}

#[derive(Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ExtendSessionRequest {
    pub refresh_token: String,
}
