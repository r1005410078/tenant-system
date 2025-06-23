use sea_orm::prelude::DateTimeUtc;
use serde::{Deserialize, Serialize};

use crate::{
    domain::roles::events::permission_granted_to_role::Permission,
    infrastructure::entitiy::role_detail_read_model,
};

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct RoleQueryReadModelDto {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub permissions: Option<serde_json::Value>,
    pub created_at: Option<DateTimeUtc>,
    pub updated_at: Option<DateTimeUtc>,
}

impl RoleQueryReadModelDto {
    pub fn new(
        id: String,
        name: String,
        description: Option<String>,
        permissions: Option<Vec<Permission>>,
    ) -> Self {
        Self {
            id,
            name,
            description,
            permissions: serde_json::to_value(permissions).ok(),
            created_at: None,
            updated_at: None,
        }
    }
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
