use std::sync::Arc;

use event_bus::AsyncEventBus;
use serde::Deserialize;

use crate::{
    application::repositories::role::RoleRepository,
    domain::roles::events::permission_granted_to_role::Permission,
};

#[derive(Debug, Deserialize, Clone)]
pub struct UpdateRoleCommand {
    pub id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub permissions: Option<Vec<Permission>>,
}

impl UpdateRoleCommand {
    pub fn new(
        id: String,
        name: Option<String>,
        description: Option<String>,
        permissions: Option<Vec<Permission>>,
    ) -> Self {
        Self {
            id,
            name,
            description,
            permissions,
        }
    }
}

pub struct UpdateRoleCommandHandler {
    role_repository: Arc<dyn RoleRepository>,
    event_bus: Arc<AsyncEventBus>,
}

impl UpdateRoleCommandHandler {
    pub fn new(role_repository: Arc<dyn RoleRepository>, event_bus: Arc<AsyncEventBus>) -> Self {
        Self {
            role_repository,
            event_bus,
        }
    }

    pub async fn handle(&self, command: UpdateRoleCommand) -> anyhow::Result<()> {
        let mut role = self.role_repository.find_by_id(&command.id).await?;
        role.update(command.name, command.description);
        self.role_repository.save(&role).await?;
        self.event_bus.publish(role).await;

        Ok(())
    }
}
