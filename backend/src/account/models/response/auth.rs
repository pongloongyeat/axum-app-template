use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::Serialize;

use super::user::UserResponse;

#[derive(Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticatedResponse {
    pub session_token: String,
    pub refresh_token: String,
    pub session_token_expiry: DateTime<Utc>,
    pub refresh_token_expiry: DateTime<Utc>,
    pub user: UserResponse,
}
