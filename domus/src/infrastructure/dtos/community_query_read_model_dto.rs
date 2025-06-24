use sea_orm::prelude::DateTimeUtc;
use serde::{Deserialize, Serialize};

use crate::infrastructure::entitiy::community_query;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityQueryReadModelDto {
    pub id: String,
    pub name: String,
    pub address: String,
    pub city: String,
    pub year_built: u16,
    pub community_type: String,
    pub description: Option<String>,
    pub image: Option<String>,
    pub location: Option<String>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

impl From<community_query::Model> for CommunityQueryReadModelDto {
    fn from(value: community_query::Model) -> Self {
        Self {
            id: value.id,
            name: value.name,
            address: value.address,
            city: value.city,
            year_built: value.year_built,
            community_type: value.community_type,
            description: value.description,
            image: value.image,
            location: value.location,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
