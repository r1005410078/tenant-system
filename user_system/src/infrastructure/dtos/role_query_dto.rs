use sea_orm::prelude::DateTimeUtc;
use serde::{Deserialize, Serialize};

use crate::infrastructure::entitiy::{permissions_detail, role_detail_read_model};

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct RoleQueryReadModelDto {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub permissions: Option<Vec<permissions_detail::Model>>,
    pub created_at: Option<DateTimeUtc>,
    pub updated_at: Option<DateTimeUtc>,
}
