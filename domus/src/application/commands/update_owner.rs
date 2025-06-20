use std::{str, sync::Arc};

use event_bus::AsyncEventBus;
use serde::Deserialize;

use crate::{
    application::repositories::owner_repository_aggregate::OwnerRepositoryAggregate,
    domain::owner::value_objects::owner::HouseOwner,
};

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateOwnerCommand {
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

impl UpdateOwnerCommand {
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

    pub fn from(owner: &HouseOwner) -> UpdateOwnerCommand {
        UpdateOwnerCommand {
            // 业主ID
            id: owner.id.clone().unwrap(),
            // 业主名称
            name: owner.name.clone(),
            // 业主身份证号
            id_card: owner.id_card.clone(),
            // 业主电话
            phone: owner.phone.clone(),
            // 业主身份证照片
            id_card_images: owner.id_card_images.clone(),
            // 业主描述
            description: owner.description.clone(),
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

    pub async fn handle(&self, command: UpdateOwnerCommand) -> anyhow::Result<()> {
        let mut aggregate = self.owner_repository.find_by_id(&command.id).await?;

        // 身份证是否存在
        if let Some(id_card) = &command.id_card {
            if self
                .owner_repository
                .exists_id_card(id_card, Some(aggregate.owner_id.clone()))
                .await?
            {
                return Err(anyhow::anyhow!("身份证号已存在"));
            }
        }

        // 手机号是否存在
        if let Some(phone) = &command.phone {
            if self
                .owner_repository
                .exists_phone(phone, Some(aggregate.owner_id.clone()))
                .await?
            {
                return Err(anyhow::anyhow!("手机号已存在"));
            }
        }

        let event = aggregate.update(&command.to_data())?;

        self.owner_repository.save(&aggregate).await?;
        self.event_bus.publish(event).await;

        Ok(())
    }
}
