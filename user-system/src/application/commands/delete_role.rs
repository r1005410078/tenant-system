use event_bus::AsyncEventBus;
use std::sync::Arc;

use crate::application::repositories::role::RoleRepository;

pub struct DeleteRoleCommand {
    pub id: String,
}

impl DeleteRoleCommand {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

pub struct DeleteRoleCommandHandler {
    event_bus: Arc<AsyncEventBus>,
    role_repository: Arc<dyn RoleRepository>,
}

impl DeleteRoleCommandHandler {
    pub fn new(role_repository: Arc<dyn RoleRepository>, event_bus: Arc<AsyncEventBus>) -> Self {
        Self {
            role_repository,
            event_bus,
        }
    }

    pub async fn handle(&self, command: DeleteRoleCommand) -> anyhow::Result<()> {
        let mut role = self.role_repository.find_by_id(&command.id).await?;
        let event = role.delete();

        self.role_repository.save(&role).await?;
        self.event_bus.persist_and_publish(event).await?;

        Ok(())
    }
}
