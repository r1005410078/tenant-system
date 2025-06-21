use std::sync::Arc;

use event_bus::AsyncEventBus;

use crate::{
    application::repositories::user_aggreate_repository::UserAggregateRepository,
    domain::user::events::user_deleted::{self, UserDeletedEvent},
};

pub struct DeleteUserCommand {
    pub user_id: String,
}

impl DeleteUserCommand {
    pub fn new(user_id: String) -> Self {
        Self { user_id }
    }
}

pub struct DeleteUserCommandHandler {
    user_poll: Arc<dyn UserAggregateRepository>,
    event_bus: Arc<AsyncEventBus>,
}

impl DeleteUserCommandHandler {
    pub fn new(user_poll: Arc<dyn UserAggregateRepository>, event_bus: Arc<AsyncEventBus>) -> Self {
        Self {
            user_poll,
            event_bus,
        }
    }

    pub async fn handle(&self, command: DeleteUserCommand) -> anyhow::Result<()> {
        let mut user_aggregate = self.user_poll.find_by_id(&command.user_id).await?;

        let user_deleted_event = user_aggregate.delete();

        self.user_poll.save(&user_aggregate).await?;
        self.event_bus.publish(user_deleted_event).await;
        Ok(())
    }
}
