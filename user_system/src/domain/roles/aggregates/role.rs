use chrono::Utc;
use sea_orm::prelude::DateTimeUtc;

use crate::domain::roles::events::{
    permission_granted_to_role::{self, Permission, PermissionGrantedToRoleEvent},
    role::RoleEvent,
    role_created::RoleCreatedEvent,
    role_deleted::RoleDeletedEvent,
    role_updated::RoleUpdatedEvent,
};

#[derive(Debug, Clone)]
pub struct RoleAggregate {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    // 授权信息
    // vec["p, admin, data1, read", "p, admin, data1, write", "p, admin, data2, red"]
    pub permissions: Vec<Permission>,
    pub deleted_at: Option<DateTimeUtc>,
}

impl RoleAggregate {
    pub fn new(id: String, name: String, description: Option<String>) -> Self {
        Self {
            id,
            name,
            description,
            permissions: Vec::new(),
            deleted_at: None,
        }
    }

    pub fn create(name: String, description: Option<String>) -> (Self, RoleEvent) {
        let role = RoleAggregate::new(uuid::Uuid::new_v4().to_string(), name, description);
        (
            role.clone(),
            RoleEvent::RoleCreated(RoleCreatedEvent::new(
                role.id.clone(),
                role.name.clone(),
                role.description.clone(),
                Some(vec![]),
            )),
        )
    }

    pub fn delete(&mut self) -> RoleEvent {
        self.deleted_at = Some(Utc::now());

        RoleEvent::RoleDeleted(RoleDeletedEvent::new(self.id.clone()))
    }

    pub fn update(&mut self, name: Option<String>, description: Option<String>) -> RoleEvent {
        self.name = name.unwrap_or(self.name.clone());
        self.description = description.clone();

        RoleEvent::RoleUpdated(RoleUpdatedEvent::new(
            self.id.clone(),
            self.name.clone(),
            self.description.clone(),
            Some(self.permissions.clone()),
        ))
    }

    // 为角色授权
    pub fn grant_permissions(&mut self, permissions: Vec<Permission>) -> RoleEvent {
        self.permissions = permissions;

        RoleEvent::PermissionGrantedToRole(PermissionGrantedToRoleEvent::new(
            self.id.clone(),
            self.permissions.clone(),
        ))
    }
}
