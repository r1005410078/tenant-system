use std::sync::Arc;

use event_bus::AsyncEventBus;

use crate::application::repositories::community_repository_aggregate::CommunityRepositoryAggregate;

pub struct DeleteCommunityCommand {
    pub id: String,
}

impl DeleteCommunityCommand {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

pub struct DeleteCommunityCommandHandler {
    pub community_repository: Arc<dyn CommunityRepositoryAggregate>,
    pub event_bus: Arc<AsyncEventBus>,
}

impl DeleteCommunityCommandHandler {
    pub fn new(
        community_repository: Arc<dyn CommunityRepositoryAggregate>,
        event_bus: Arc<AsyncEventBus>,
    ) -> Self {
        Self {
            community_repository,
            event_bus,
        }
    }

    pub async fn handle(&self, community_id: String) -> anyhow::Result<()> {
        let mut aggreagate = self.community_repository.find_by_id(&community_id).await?;

        let event = aggreagate.delete();

        self.community_repository.save(&aggreagate).await?;
        self.event_bus.publish(event).await;

        Ok(())
    }
}
