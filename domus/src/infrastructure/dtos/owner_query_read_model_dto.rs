use sea_orm::prelude::DateTimeUtc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnerQueryReadModelDto {
    pub id: String,
    pub name: String,
    pub phone: String,
    pub id_card: Option<String>,
    pub id_card_images: Option<serde_json::Value>,
    pub description: Option<String>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}
