use std::sync::Arc;

use event_bus::AsyncEventBus;

use crate::{
    application::{
        commands::{save_community::SaveCommunityCommand, save_owner::SaveOwnerCommand},
        repositories::owner_repository_aggregate::OwnerRepositoryAggregate,
    },
    domain::owner::{aggregates::owner::OwnerAggregate, value_objects::owner::HouseOwner},
};

pub struct SaveOwnerCommandHandler {
    owner_repository: Arc<dyn OwnerRepositoryAggregate>,
    event_bus: Arc<AsyncEventBus>,
}

impl SaveOwnerCommandHandler {
    pub fn new(
        owner_repository: Arc<dyn OwnerRepositoryAggregate>,
        event_bus: Arc<AsyncEventBus>,
    ) -> Self {
        Self {
            owner_repository,
            event_bus,
        }
    }

    pub async fn handle(&self, command: SaveOwnerCommand) -> anyhow::Result<String> {
        let owner = command.into_inner();
        let owner_id = owner.id.clone();

        match owner_id {
            Some(ref id) => {
                let mut aggregate = self.owner_repository.find_by_id(id).await?;

                // 身份证是否存在
                if let Some(id_card) = &owner.id_card {
                    if self
                        .owner_repository
                        .exists_id_card(id_card, Some(aggregate.owner_id.clone()))
                        .await?
                    {
                        return Err(anyhow::anyhow!("身份证号已存在"));
                    }
                }

                // 手机号是否存在
                if let Some(phone) = &owner.phone {
                    if self
                        .owner_repository
                        .exists_phone(phone, Some(aggregate.owner_id.clone()))
                        .await?
                    {
                        return Err(anyhow::anyhow!("手机号已存在"));
                    }
                }

                let event = aggregate.update(&owner)?;

                self.owner_repository.save(&aggregate).await?;
                self.event_bus.publish(event).await;

                Ok(aggregate.owner_id)
            }
            None => {
                // 身份证是否存在
                if let Some(id_card) = &owner.id_card {
                    if self.owner_repository.exists_id_card(id_card, None).await? {
                        return Err(anyhow::anyhow!("身份证号已存在"));
                    }
                }

                let (aggregate, event) = OwnerAggregate::create(&owner)?;

                // 手机号是否存在
                if self
                    .owner_repository
                    .exists_phone(owner.phone.as_ref().unwrap(), None)
                    .await?
                {
                    return Err(anyhow::anyhow!("手机号已存在"));
                }

                self.owner_repository.create(aggregate.clone()).await?;
                self.event_bus.publish(event).await;

                Ok(aggregate.owner_id)
            }
        }
    }
}
