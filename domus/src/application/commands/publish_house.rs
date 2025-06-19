use std::sync::Arc;

use event_bus::AsyncEventBus;

use crate::application::repositories::house_repository_aggregate::HouseRepositoryAggregate;

pub struct PublishHouseCommand {
    pub id: String,
}

impl PublishHouseCommand {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

pub struct PublishHouseCommandHandler {
    house_repository: Arc<dyn HouseRepositoryAggregate>,
    event_bus: Arc<AsyncEventBus>,
}

impl PublishHouseCommandHandler {
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

        let event = aggreagate.publish()?;
        self.event_bus.publish(event).await;

        Ok(())
    }
}
