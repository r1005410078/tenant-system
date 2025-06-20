use std::sync::Arc;

use event_bus::AsyncEventBus;
use serde::Deserialize;

use crate::{
    application::repositories::owner_repository_aggregate::OwnerRepositoryAggregate,
    domain::owner::{aggregates::owner::OwnerAggregate, value_objects::owner::HouseOwner},
};

#[derive(Debug, Clone, Deserialize)]
pub struct CreateOwnerCommand {
    // 业主ID
    pub id: String,
    // 业主姓名
    pub name: String,
    // 业主电话
    pub phone: Option<String>,
    // 业主身份证号
    pub id_card: Option<String>,
    // 业主身份证照片
    pub id_card_images: Option<Vec<String>>,
    // 业主情况
    pub description: Option<String>,
}

impl CreateOwnerCommand {
    fn to_data(&self) -> HouseOwner {
        HouseOwner {
            id: Some(self.id.clone()),
            name: Some(self.name.clone()),
            phone: self.phone.clone(),
            id_card: self.id_card.clone(),
            id_card_images: self.id_card_images.clone(),
            description: self.description.clone(),
        }
    }
}

pub struct CreateOwnerCommandHandler {
    owner_repository: Arc<dyn OwnerRepositoryAggregate>,
    event_bus: Arc<AsyncEventBus>,
}

impl CreateOwnerCommandHandler {
    pub fn new(
        owner_repository: Arc<dyn OwnerRepositoryAggregate>,
        event_bus: Arc<AsyncEventBus>,
    ) -> Self {
        Self {
            owner_repository,
            event_bus,
        }
    }

    pub async fn handle(&self, command: CreateOwnerCommand) -> anyhow::Result<()> {
        let (aggregate, event) = OwnerAggregate::create(&command.to_data())?;

        self.owner_repository.create(aggregate).await?;
        self.event_bus.publish(event).await;

        Ok(())
    }
}
