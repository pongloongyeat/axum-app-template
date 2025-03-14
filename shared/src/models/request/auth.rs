use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{constants::admin::ADMIN_EMAIL, traits::Validatable, utils::validators};

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticateRequest {
    pub email: String,
    pub password: String,
}

impl Validatable for AuthenticateRequest {
    fn validated_properties() -> Vec<String> {
        vec!["email".into(), "password".into()]
    }

    fn validate_property(&self, property: &str) -> Option<Vec<String>> {
        match property {
            "email" => validators::is_email_valid(&self.email),
            "password" => {
                if self.email == ADMIN_EMAIL {
                    None
                } else {
                    validators::is_password_valid(&self.password)
                }
            }
            _ => None,
        }
    }
}
