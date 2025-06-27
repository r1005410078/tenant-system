use std::sync::Arc;

use event_bus::AsyncEventBus;
use serde::Deserialize;

use crate::{
    application::repositories::role::RoleRepository, domain::roles::aggregates::role::RoleAggregate,
};

#[derive(Debug, Deserialize)]
pub struct CreateRoleCommand {
    pub name: String,
    pub description: Option<String>,
}

impl CreateRoleCommand {
    pub fn new(name: String, description: Option<String>) -> Self {
        Self { name, description }
    }
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
