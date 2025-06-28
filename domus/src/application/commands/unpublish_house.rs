use std::sync::Arc;

use event_bus::AsyncEventBus;

use crate::application::repositories::house_repository_aggregate::HouseRepositoryAggregate;

pub struct PublishHouseCommand {
    pub id: String,
    // 下架原因
    pub description: String,
}

#[allow(dead_code)]
impl PublishHouseCommand {
    pub fn new(id: String, description: String) -> Self {
        Self { id, description }
    }
}

pub struct UnpublishHouseCommandHandler {
    house_repository: Arc<dyn HouseRepositoryAggregate>,
    event_bus: Arc<AsyncEventBus>,
}

impl UnpublishHouseCommandHandler {
    pub fn new(
        house_repository: Arc<dyn HouseRepositoryAggregate>,
        event_bus: Arc<AsyncEventBus>,
    ) -> Self {
        Self {
            house_repository,
            event_bus,
        }
    }

    pub async fn handle(&self, command: PublishHouseCommand) -> anyhow::Result<()> {
        let mut aggreagate = self.house_repository.find_by_id(&command.id).await?;

        let event = aggreagate.unpublish(&command.description)?;

        self.event_bus.publish(event).await;

        Ok(())
    }
}
