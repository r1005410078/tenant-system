use sea_orm::prelude::DateTimeUtc;
use serde::{Deserialize, Serialize};

use crate::infrastructure::entitiy::owner_query;

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

impl From<owner_query::Model> for OwnerQueryReadModelDto {
    fn from(value: owner_query::Model) -> Self {
        Self {
            id: value.id,
            name: value.name,
            phone: value.phone,
            id_card: value.id_card,
            id_card_images: value.id_card_images,
            description: value.description,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
