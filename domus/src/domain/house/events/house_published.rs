use sea_orm::prelude::DateTimeUtc;

#[derive(Debug, Clone)]
pub struct HousePublishedEvent {
    pub house_id: String,
    pub published_at: chrono::DateTime<chrono::Utc>,
}

impl HousePublishedEvent {
    pub fn new(house_id: String, published_at: DateTimeUtc) -> Self {
        Self {
            house_id,
            published_at,
        }
    }
}
