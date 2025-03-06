use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PageRequest {
    pub take: u64,
    pub skip: u64,
}

#[derive(Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Page<T> {
    pub content: Vec<T>,
    pub total: u64,
    pub request: PageRequest,
}

impl<T> Page<T> {
    pub fn map<F, R>(&self, transform: F) -> Page<R>
    where
        F: FnMut(&T) -> R,
    {
        Page {
            content: self.content.iter().map(transform).collect(),
            total: self.total,
            request: self.request,
        }
    }
}
