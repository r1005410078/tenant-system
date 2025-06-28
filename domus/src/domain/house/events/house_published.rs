use sea_orm::prelude::DateTimeUtc;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct HousePublishedEvent {
    pub house_id: String,
    pub published_at: chrono::DateTime<chrono::Utc>,
}

#[allow(dead_code)]
impl HousePublishedEvent {
    pub fn new(house_id: String, published_at: DateTimeUtc) -> Self {
        Self {
            house_id,
            published_at,
        }
    }
}
