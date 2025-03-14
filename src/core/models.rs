use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PageRequest {
    pub take: u64,
    pub skip: u64,

    #[serde(default)]
    pub sort: Vec<String>,
}

impl PageRequest {
    pub fn to_sql_string(&self, sql: &str) -> String {
        let mut sql = sql.to_string();

        let sort = self
            .sort
            .iter()
            .map(SortingOrder::from)
            .filter(|order| order != &SortingOrder::NoOrder)
            .filter_map(|order| order.to_sql_string())
            .collect::<Vec<String>>()
            .join(", ");
        if !sort.is_empty() {
            let sort = format!("ORDER BY {sort}\n");
            sql.push_str(&sort);
        }

        let limit = self.take;
        let offset = self.skip;
        sql.push_str(format!("LIMIT {limit} OFFSET {offset}").as_str());

        sql
    }
}

#[derive(PartialEq)]
pub enum SortingOrder {
    Ascending(String),
    Descending(String),
    NoOrder,
}

impl<T> From<T> for SortingOrder
where
    T: Into<String>,
{
    fn from(value: T) -> Self {
        let value: String = value.into();
        if let Some((key, order)) = value.split_once(",") {
            let key = key.trim().to_string();
            let order = <&str as Into<String>>::into(order).to_lowercase();
            let order = order.trim();

            match order {
                "asc" | "ascending" => Self::Ascending(key),
                "desc" | "dsc" | "descending" => Self::Descending(key),
                _ => {
                    tracing::warn!("Unable to determine sorting order: {value}. Skipping");
                    Self::NoOrder
                }
            }
        } else {
            tracing::warn!("Unable to deserialize sorting order: {value}. Skipping");
            Self::NoOrder
        }
    }
}

impl SortingOrder {
    pub fn to_sql_string(&self) -> Option<String> {
        match self {
            SortingOrder::Ascending(key) => Some(format!("{key} ASC")),
            SortingOrder::Descending(key) => Some(format!("{key} DESC")),
            SortingOrder::NoOrder => None,
        }
    }
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
