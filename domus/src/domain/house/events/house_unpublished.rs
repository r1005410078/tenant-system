use sea_orm::prelude::DateTimeUtc;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct HouseUnpublishedEvent {
    pub id: String,
    pub unpublish_at: DateTimeUtc,
    // 下架原因
    pub description: String,
}

impl HouseUnpublishedEvent {
    pub fn new(id: String, unpublish_at: DateTimeUtc, description: &str) -> Self {
        Self {
            id,
            unpublish_at,
            description: description.to_string(),
        }
    }
}
