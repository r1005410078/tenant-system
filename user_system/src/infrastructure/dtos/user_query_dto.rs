use sea_orm::prelude::DateTimeUtc;
use serde::{Deserialize, Serialize};
use serde_json::Value as Json;

use crate::{domain::user::aggregates::user::UserAggregate, infrastructure::entitiy::user_query};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserQueryDto {
    pub user_id: String,
    pub username: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub roles: Option<Json>,
    pub created_at: Option<DateTimeUtc>,
    pub updated_at: Option<DateTimeUtc>,
}

impl From<user_query::Model> for UserQueryDto {
    fn from(user: user_query::Model) -> Self {
        Self {
            user_id: user.user_id,
            username: user.username,
            email: user.email,
            phone: user.phone,
            roles: user.roles,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}
