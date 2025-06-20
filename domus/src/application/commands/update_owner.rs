use std::{str, sync::Arc};

use event_bus::AsyncEventBus;
use serde::Deserialize;

use crate::{
    application::repositories::owner_repository_aggregate::OwnerRepositoryAggregate,
    domain::owner::value_objects::owner::HouseOwner,
};

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateOwenerCommand {
    // 业主ID
    pub id: String,
    // 业主姓名
    pub name: Option<String>,
    // 业主电话
    pub phone: Option<String>,
    // 业主身份证号
    pub id_card: Option<String>,
    // 业主身份证照片
    pub id_card_images: Option<Vec<String>>,
    // 业主情况
    pub description: Option<String>,
}

impl UpdateOwenerCommand {
    pub fn to_data(&self) -> HouseOwner {
        HouseOwner {
            id: Some(self.id.clone()),
            name: self.name.clone(),
            phone: self.phone.clone(),
            id_card: self.id_card.clone(),
            id_card_images: self.id_card_images.clone(),
            description: self.description.clone(),
        }
    }
}

pub struct UpdateOwnerCommandHandler {
    owner_repository: Arc<dyn OwnerRepositoryAggregate>,
    event_bus: Arc<AsyncEventBus>,
}

impl UpdateOwnerCommandHandler {
    pub fn new(
        owner_repository: Arc<dyn OwnerRepositoryAggregate>,
        event_bus: Arc<AsyncEventBus>,
    ) -> Self {
        Self {
            owner_repository,
            event_bus,
        }
    }

    pub async fn handle(&self, command: UpdateOwenerCommand) -> anyhow::Result<()> {
        let mut aggregate = self.owner_repository.find_by_id(&command.id).await?;

        let event = aggregate.update(&command.to_data())?;

        self.owner_repository.save(&aggregate).await?;
        self.event_bus.publish(event).await;

        Ok(())
    }
}
