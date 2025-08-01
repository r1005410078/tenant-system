use std::sync::Arc;

use crate::{
    application::{
        commands::save_house::SaveHouseCommand,
        repositories::house_repository_aggregate::HouseRepositoryAggregate,
    },
    domain::house::aggregates::house::HouseAggregate,
};
use event_bus::AsyncEventBus;

pub struct SaveHouseCommandHandler {
    pub house_repository: Arc<dyn HouseRepositoryAggregate>,
    pub event_bus: Arc<AsyncEventBus>,
}

impl SaveHouseCommandHandler {
    pub fn new(
        house_repository: Arc<dyn HouseRepositoryAggregate>,
        event_bus: Arc<AsyncEventBus>,
    ) -> Self {
        Self {
            house_repository,
            event_bus,
        }
    }

    pub async fn handle(&self, command: SaveHouseCommand) -> anyhow::Result<()> {
        let house = command.into_inner();
        let id = house.id.clone();

        if let (Some(ref community_id), Some(ref house_address)) =
            (house.community_id.clone(), house.house_address.clone())
        {
            if self
                .house_repository
                .exists_address(community_id, house_address, id.clone())
                .await?
            {
                return Err(anyhow::anyhow!("地址已存在, 您可以更新它"));
            }
        }

        match id {
            Some(id) => {
                let mut aggreagate = self.house_repository.find_by_id(&id).await?;
                let events = aggreagate.update(&house)?;

                self.house_repository.save(&aggreagate).await?;
                for event in events {
                    self.event_bus.publish(event).await;
                }

                Ok(())
            }

            None => {
                let (aggregate, event) = HouseAggregate::create(house)?;
                self.house_repository.create(aggregate).await?;
                self.event_bus.publish(event).await;

                Ok(())
            }
        }
    }
}
