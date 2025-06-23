use std::sync::Arc;

use crate::{
    application::repositories::role::RoleRepository,
    domain::roles::events::permission_granted_to_role::Permission,
};
use event_bus::AsyncEventBus;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct PermissionGrantedToRoleCommand {
    pub role_id: String,
    pub permissions: Vec<Permission>,
}

impl PermissionGrantedToRoleCommand {
    pub fn new(role_id: String, permissions: Vec<Permission>) -> Self {
        Self {
            role_id,
            permissions,
        }
    }
}

pub struct PermissionGrantedToRoleCommandHandler {
    role_repository: Arc<dyn RoleRepository>,
    event_bus: Arc<AsyncEventBus>,
}

impl PermissionGrantedToRoleCommandHandler {
    pub fn new(role_repository: Arc<dyn RoleRepository>, event_bus: Arc<AsyncEventBus>) -> Self {
        Self {
            role_repository,
            event_bus,
        }
    }

    pub async fn handle(&self, command: PermissionGrantedToRoleCommand) -> anyhow::Result<()> {
        let mut role_aggregate = self
            .role_repository
            .find_by_id(&command.role_id)
            .await
            .unwrap();

        let event = role_aggregate.grant_permissions(command.permissions);

        self.role_repository.save(&role_aggregate).await?;
        self.event_bus.persist_and_publish(event).await?;
        Ok(())
    }
}
