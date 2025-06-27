use std::sync::Arc;

use event_bus::AsyncEventBus;
use serde::Deserialize;

use crate::{
    application::repositories::role::RoleRepository,
    domain::roles::{
        aggregates::role::RoleAggregate, events::permission_granted_to_role::Permission,
    },
};

#[derive(Debug, Deserialize, Clone)]
pub struct CreateRoleCommand {
    pub name: String,
    pub description: Option<String>,
    pub permissions: Option<Vec<Permission>>,
}

pub struct CreateRoleCommandHandler {
    role_repository: Arc<dyn RoleRepository>,
    event_bus: Arc<AsyncEventBus>,
}

impl CreateRoleCommandHandler {
    pub fn new(role_repository: Arc<dyn RoleRepository>, event_bus: Arc<AsyncEventBus>) -> Self {
        Self {
            role_repository,
            event_bus,
        }
    }

    pub async fn handle(&self, command: CreateRoleCommand) -> anyhow::Result<String> {
        let (role, event) = RoleAggregate::create(command.name, command.description);
        self.role_repository.create(&role).await?;
        self.event_bus.persist_and_publish(event).await?;

        Ok(role.id)
    }
}
