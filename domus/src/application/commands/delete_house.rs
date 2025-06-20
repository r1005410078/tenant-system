use std::sync::Arc;

use event_bus::AsyncEventBus;

use crate::application::repositories::house_repository_aggregate::HouseRepositoryAggregate;

pub struct DeleteHouseCommand {
    pub id: String,
}

impl DeleteHouseCommand {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

pub struct DeleteHouseCommandHandler {
    pub house_repository: Arc<dyn HouseRepositoryAggregate>,
    pub event_bus: Arc<AsyncEventBus>,
}

impl DeleteHouseCommandHandler {
    pub fn new(
        house_repository: Arc<dyn HouseRepositoryAggregate>,
        event_bus: Arc<AsyncEventBus>,
    ) -> Self {
        Self {
            house_repository,
            event_bus,
        }
    }

    pub async fn handle(&self, command: DeleteHouseCommand) -> anyhow::Result<()> {
        let mut aggreagate = self.house_repository.find_by_id(&command.id).await?;

        let event = aggreagate.delete();

        self.house_repository.save(&aggreagate).await?;
        self.event_bus.publish(event).await;

        Ok(())
    }
}
