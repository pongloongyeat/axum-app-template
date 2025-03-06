use schemars::JsonSchema;
use serde::Serialize;

use crate::account::entities::user::UserEntity;

#[derive(Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
    pub id: i64,
    pub email: String,
}

impl From<UserEntity> for UserResponse {
    fn from(value: UserEntity) -> Self {
        Self {
            id: value.id,
            email: value.email,
        }
    }
}
