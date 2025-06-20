use std::sync::Arc;

use event_bus::AsyncEventBus;
use serde::Deserialize;

use crate::{
    application::repositories::owner_repository_aggregate::OwnerRepositoryAggregate,
    domain::owner::{aggregates::owner::OwnerAggregate, value_objects::owner::HouseOwner},
};

#[derive(Debug, Clone, Deserialize)]
pub struct CreateOwnerCommand {
    // 业主姓名
    pub name: String,
    // 业主电话
    pub phone: String,
    // 业主身份证号
    pub id_card: Option<String>,
    // 业主身份证照片
    pub id_card_images: Option<Vec<String>>,
    // 业主情况
    pub description: Option<String>,
}

impl CreateOwnerCommand {
    pub fn to_data(&self) -> HouseOwner {
        HouseOwner {
            id: None,
            name: Some(self.name.clone()),
            phone: Some(self.phone.clone()),
            id_card: self.id_card.clone(),
            id_card_images: self.id_card_images.clone(),
            description: self.description.clone(),
        }
    }

    pub fn from(owner: &HouseOwner) -> anyhow::Result<CreateOwnerCommand> {
        Ok(CreateOwnerCommand {
            // 业主名称
            name: owner.get_name()?,
            // 业主身份证号
            id_card: owner.id_card.clone(),
            // 业主电话
            phone: owner.get_phone()?,
            // 业主身份证照片
            id_card_images: owner.id_card_images.clone(),
            // 业主描述
            description: owner.description.clone(),
        })
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
        // 身份证是否存在
        if let Some(id_card) = &command.id_card {
            if self.owner_repository.exists_id_card(id_card, None).await? {
                return Err(anyhow::anyhow!("身份证号已存在"));
            }
        }

        // 手机号是否存在
        if self
            .owner_repository
            .exists_phone(&command.phone, None)
            .await?
        {
            return Err(anyhow::anyhow!("手机号已存在"));
        }

        let (aggregate, event) = OwnerAggregate::create(&command.to_data())?;

        self.owner_repository.create(aggregate).await?;
        self.event_bus.publish(event).await;

        Ok(())
    }
}
