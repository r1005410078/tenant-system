pub use std::sync::Arc;

use event_bus::AsyncEventBus;

use crate::{
    application::{
        commands::save_community::SaveCommunityCommand,
        repositories::community_repository_aggregate::CommunityRepositoryAggregate,
    },
    domain::community::aggregates::community::CommunityAggregate,
};

pub struct SaveCommunityCommandHandler {
    pub community_repository: Arc<dyn CommunityRepositoryAggregate>,
    pub event_bus: Arc<AsyncEventBus>,
}

impl SaveCommunityCommandHandler {
    pub fn new(
        community_repository: Arc<dyn CommunityRepositoryAggregate>,
        event_bus: Arc<AsyncEventBus>,
    ) -> Self {
        Self {
            community_repository,
            event_bus,
        }
    }

    pub async fn handle(&self, command: SaveCommunityCommand) -> anyhow::Result<String> {
        let community = command.into_inner();

        let community_id = match community.id.clone() {
            Some(id) => id,
            None => uuid::Uuid::new_v4().to_string(),
        };

        let aggreagate = self.community_repository.find_by_id(&community_id).await?;

        match aggreagate {
            Some(mut aggreagate) => {
                if let Some(address) = community.address.clone() {
                    if self
                        .community_repository
                        .exists_address(&address, Some(community_id.clone()))
                        .await?
                    {
                        return Err(anyhow::anyhow!("小区地址已存在"));
                    }
                }

                let event = aggreagate.update(&community)?;

                self.community_repository.save(&aggreagate).await?;
                self.event_bus.publish(event).await;

                Ok(aggreagate.community_id)
            }

            None => {
                // 创建小区
                let (aggregate, event) = CommunityAggregate::create(&command.into_inner())?;

                // 检查小区是否已存在
                if self
                    .community_repository
                    .exists_address(&aggregate.address, None)
                    .await?
                {
                    return Err(anyhow::anyhow!("Community already exists at this address"));
                }

                self.community_repository.create(aggregate.clone()).await?;
                self.event_bus.publish(event).await;

                Ok(aggregate.community_id)
            }
        }
    }
}
