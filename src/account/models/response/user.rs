use schemars::JsonSchema;
use serde::Serialize;

use crate::account::entities::user::UserEntity;

#[derive(Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
    pub email: String,
}

impl From<UserEntity> for UserResponse {
    fn from(value: UserEntity) -> Self {
        Self { email: value.email }
    }
}
