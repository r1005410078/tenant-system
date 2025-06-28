use crate::application::repositories::owner_repository_aggregate::OwnerRepositoryAggregate;
use event_bus::AsyncEventBus;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteOwnerCommand {
    // 业主ID
    pub id: String,
}

#[allow(dead_code)]
impl DeleteOwnerCommand {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

pub struct DeleteOwnerCommandHandler {
    owner_repository: Arc<dyn OwnerRepositoryAggregate>,
    event_bus: Arc<AsyncEventBus>,
}

impl DeleteOwnerCommandHandler {
    pub fn new(
        owner_repository: Arc<dyn OwnerRepositoryAggregate>,
        event_bus: Arc<AsyncEventBus>,
    ) -> Self {
        Self {
            owner_repository,
            event_bus,
        }
    }

    pub async fn handle(&self, command: DeleteOwnerCommand) -> anyhow::Result<()> {
        let mut aggregate = self.owner_repository.find_by_id(&command.id).await?;

        let event = aggregate.delete();
        self.owner_repository.save(&aggregate).await?;
        self.event_bus.publish(event).await;

        Ok(())
    }
}
