use std::sync::Arc;

use event_bus::AsyncEventBus;

use crate::{
    application::{
        commands::create_community::CreateCommunityCommand,
        repositories::community_repository_aggregate::CommunityRepositoryAggregate,
    },
    domain::community::aggregates::community::CommunityAggregate,
};

pub struct CreateCommunityCommandHandler {
    pub community_repository: Arc<dyn CommunityRepositoryAggregate>,
    pub event_bus: Arc<AsyncEventBus>,
}

impl CreateCommunityCommandHandler {
    pub fn new(
        community_repository: Arc<dyn CommunityRepositoryAggregate>,
        event_bus: Arc<AsyncEventBus>,
    ) -> Self {
        Self {
            community_repository,
            event_bus,
        }
    }

    pub async fn handle(&self, command: CreateCommunityCommand) -> anyhow::Result<()> {
        let (aggregate, event) = CommunityAggregate::create(&command.to_data());

        // 检查小区是否已存在
        if self.community_repository.exists(&aggregate.address).await? {
            return Err(anyhow::anyhow!("Community already exists"));
        }

        self.community_repository.create(aggregate).await?;
        self.event_bus.publish(event).await;

        Ok(())
    }
}
