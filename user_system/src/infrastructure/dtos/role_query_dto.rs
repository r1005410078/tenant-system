use sea_orm::prelude::DateTimeUtc;
use serde::{Deserialize, Serialize};

use crate::infrastructure::entitiy::role_detail_read_model;

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct RoleQueryReadModelDto {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub permissions: Option<serde_json::Value>,
    pub created_at: Option<DateTimeUtc>,
    pub updated_at: Option<DateTimeUtc>,
}

impl From<role_detail_read_model::Model> for RoleQueryReadModelDto {
    fn from(model: role_detail_read_model::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            description: model.description,
            permissions: model.permissions,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}
