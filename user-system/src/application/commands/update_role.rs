use std::sync::Arc;

use event_bus::AsyncEventBus;
use serde::Deserialize;

use crate::application::repositories::role::RoleRepository;

#[derive(Debug, Deserialize)]
pub struct UpdateRoleCommand {
    pub id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub permissions: Option<Vec<String>>,
}

impl UpdateRoleCommand {
    pub fn new(
        id: String,
        name: Option<String>,
        description: Option<String>,
        permissions: Option<Vec<String>>,
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
        println!("11111 {:?}", command);
        let mut role = self.role_repository.find_by_id(&command.id).await?;
        println!("2222 {:?}", role);
        role.update(command.name, command.description, command.permissions);
        self.role_repository.save(&role).await?;
        self.event_bus.publish(role).await;

        Ok(())
    }
}
