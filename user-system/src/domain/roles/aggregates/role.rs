use chrono::Utc;
use sea_orm::prelude::DateTimeUtc;

use crate::domain::roles::events::{
    role_created::RoleCreatedEvent, role_deleted::RoleDeletedEvent, role_updated::RoleUpdatedEvent,
};

#[derive(Debug, Clone)]
pub struct RoleAggregate {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    // 授权信息
    pub permissions: Vec<String>,
    // 绑定用户
    pub users: Vec<String>,
    pub deleted_at: Option<DateTimeUtc>,
}

impl RoleAggregate {
    pub fn new(id: String, name: String, description: Option<String>) -> Self {
        Self {
            id,
            name,
            description,
            permissions: Vec::new(),
            users: Vec::new(),
            deleted_at: None,
        }
    }

    pub fn create(name: String, description: Option<String>) -> (Self, RoleCreatedEvent) {
        let role = RoleAggregate::new(uuid::Uuid::new_v4().to_string(), name, description);
        (
            role.clone(),
            RoleCreatedEvent::new(role.id.clone(), role.name.clone(), role.description.clone()),
        )
    }

    pub fn delete(&mut self) -> RoleDeletedEvent {
        self.deleted_at = Some(Utc::now());
        RoleDeletedEvent::new(self.id.clone())
    }

    pub fn is_deleted(&self) -> bool {
        self.deleted_at.is_some()
    }

    pub fn update(
        &mut self,
        name: Option<String>,
        description: Option<String>,
        permissions: Option<Vec<String>>,
    ) -> RoleUpdatedEvent {
        self.name = name.unwrap_or(self.name.clone());
        self.description = description.clone();
        self.permissions = permissions.unwrap_or(self.permissions.clone());

        RoleUpdatedEvent::new(
            self.id.clone(),
            Some(self.name.clone()),
            self.description.clone(),
        )
    }
}
