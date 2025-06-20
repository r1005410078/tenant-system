use std::sync::Arc;

use event_bus::AsyncEventBus;

use crate::application::{
    commands::update_community::UpdateCommunityCommand,
    repositories::community_repository_aggregate::CommunityRepositoryAggregate,
};

pub struct UpdateCommunityCommandHandler {
    pub community_repository: Arc<dyn CommunityRepositoryAggregate>,
    pub event_bus: Arc<AsyncEventBus>,
}

impl UpdateCommunityCommandHandler {
    pub fn new(
        community_repository: Arc<dyn CommunityRepositoryAggregate>,
        event_bus: Arc<AsyncEventBus>,
    ) -> Self {
        Self {
            community_repository,
            event_bus,
        }
    }

    pub async fn handle(&self, command: UpdateCommunityCommand) -> anyhow::Result<()> {
        if let Some(address) = command.address.clone() {
            if self
                .community_repository
                .exists_address(&address, Some(command.community_id.clone()))
                .await?
            {
                return Err(anyhow::anyhow!("小区地址已存在"));
            }
        }

        let mut aggreagate = self
            .community_repository
            .find_by_id(&command.community_id)
            .await?;

        let event = aggreagate.update(&command.to_data())?;

        self.community_repository.save(&aggreagate).await?;
        self.event_bus.publish(event).await;

        Ok(())
    }
}
