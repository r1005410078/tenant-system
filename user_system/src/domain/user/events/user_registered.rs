use serde::Serialize;
use uuid::Uuid;

use crate::infrastructure::dtos::user_query_dto::UserQueryDto;

#[derive(Debug, Clone, Serialize)]
pub struct UserRegisteredEvent {
    pub id: Uuid,
    pub username: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub roles: Vec<String>,
    pub account_status: String,
}

impl UserRegisteredEvent {
    pub fn to_user_query_dto(&self) -> UserQueryDto {
        UserQueryDto {
            user_id: self.id.to_string(),
            username: self.username.clone(),
            email: self.email.clone(),
            phone: self.phone.clone(),
            rules: Some(serde_json::to_value(self.roles.clone()).unwrap()),
            created_at: None,
            updated_at: None,
        }
    }
}
