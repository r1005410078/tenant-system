use std::sync::Arc;

use event_bus::AsyncEventBus;

use crate::application::{
    commands::update_house::UpdateHouseCommand,
    repositories::house_repository_aggregate::HouseRepositoryAggregate,
};

pub struct UpdateCommunityHandler {
    pub house_repository: Arc<dyn HouseRepositoryAggregate>,
    pub event_bus: Arc<AsyncEventBus>,
}

impl UpdateCommunityHandler {
    pub fn new(
        house_repository: Arc<dyn HouseRepositoryAggregate>,
        event_bus: Arc<AsyncEventBus>,
    ) -> Self {
        Self {
            house_repository,
            event_bus,
        }
    }

    pub async fn handle(&self, command: UpdateHouseCommand) -> anyhow::Result<()> {
        let mut aggreagate = self.house_repository.find_by_id(&command.house_id).await?;

        let event = aggreagate.update(&command.to_data())?;

        self.house_repository.save(&aggreagate).await?;
        self.event_bus.publish(event).await;

        Ok(())
    }
}
