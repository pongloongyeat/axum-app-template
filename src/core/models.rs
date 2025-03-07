use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PageRequest {
    pub take: u64,
    pub skip: u64,

    #[serde(default)]
    pub sort: Vec<SortingOrder>,
}

#[derive(Serialize, Deserialize, Clone, JsonSchema)]
pub enum SortingOrder {
    Ascending(String),
    Descending(String),
}

#[derive(Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Page<T> {
    pub content: Vec<T>,
    pub total: u64,
    pub has_next: bool,
    pub request: PageRequest,
}

impl<T> Page<T> {
    pub fn new(content: Vec<T>, total: u64, request: PageRequest) -> Self {
        Self {
            content,
            total,
            has_next: (total as i64 - request.take as i64 - request.skip as i64) > 0,
            request,
        }
    }
}

impl<T> Page<T> {
    pub fn map<F, R>(self, transform: F) -> Page<R>
    where
        F: FnMut(&T) -> R,
    {
        Page {
            content: self.content.iter().map(transform).collect(),
            total: self.total,
            has_next: self.has_next,
            request: self.request,
        }
    }
}
