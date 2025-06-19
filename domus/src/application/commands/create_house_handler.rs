use std::sync::Arc;

use crate::{
    application::{
        commands::create_house::CreateHouseCommand,
        repositories::house_repository_aggregate::HouseRepositoryAggregate,
    },
    domain::house::aggregates::house::HouseAggregate,
};
use event_bus::AsyncEventBus;

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

    pub async fn handle(&self, command: CreateHouseCommand) -> anyhow::Result<()> {
        let (aggregate, event) = HouseAggregate::create(&command.to_data());

        self.house_repository.create(aggregate).await?;
        self.event_bus.publish(event).await;

        Ok(())
    }
}
