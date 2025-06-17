use chrono::Utc;
use sea_orm::prelude::DateTimeUtc;

use crate::domain::roles::events::{
    permission_granted_to_role::{self, Permission, PermissionGrantedToRoleEvent},
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

    pub fn update(
        &mut self,
        name: Option<String>,
        description: Option<String>,
    ) -> RoleUpdatedEvent {
        self.name = name.unwrap_or(self.name.clone());
        self.description = description.clone();

        RoleUpdatedEvent::new(
            self.id.clone(),
            Some(self.name.clone()),
            self.description.clone(),
        )
    }

    // 为角色授权
    pub fn grant_permissions(
        &mut self,
        permissions: Vec<Permission>,
    ) -> PermissionGrantedToRoleEvent {
        self.permissions = permissions;
        PermissionGrantedToRoleEvent::new(self.id.clone(), self.permissions.clone())
    }
}
