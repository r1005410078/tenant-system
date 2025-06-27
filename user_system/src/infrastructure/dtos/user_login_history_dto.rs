use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::infrastructure::entitiy::{sea_orm_active_enums::Status, user_login_history};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserLoginHistoryDto {
    pub id: Option<u64>,
    pub user_id: String,
    pub username: String,
    pub status: Status,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub login_at: NaiveDateTime,
    pub fail_reason: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl UserLoginHistoryDto {
    pub fn new(user_id: &str, username: &str, status: Status) -> Self {
        Self {
            id: None,
            user_id: user_id.to_string(),
            username: username.to_string(),
            status,
            ip_address: None,
            user_agent: None,
            login_at: chrono::Utc::now().naive_utc(),
            fail_reason: None,
            created_at: None,
            updated_at: None,
        }
    }
}

impl From<user_login_history::Model> for UserLoginHistoryDto {
    fn from(value: user_login_history::Model) -> Self {
        Self {
            id: Some(value.id),
            user_id: value.user_id,
            username: value.username,
            status: value.status,
            ip_address: value.ip_address,
            user_agent: value.user_agent,
            login_at: value.login_at,
            fail_reason: value.fail_reason,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
