use std::sync::Arc;

use event_bus::AsyncEventBus;
use serde::Deserialize;

use crate::{
    application::repositories::user_aggreate_repository::UserAggregateRepository,
    domain::user::aggregates::user::UserAggregate,
};

#[derive(Debug, Deserialize, Clone)]
pub struct UpdateUserCommand {
    pub id: String,
    pub username: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub roles: Option<Vec<String>>,
}

pub struct UpdateUserCommandHandler {
    user_pool: Arc<dyn UserAggregateRepository>,
    event_bus: Arc<AsyncEventBus>,
}

impl UpdateUserCommandHandler {
    pub fn new(user_pool: Arc<dyn UserAggregateRepository>, event_bus: Arc<AsyncEventBus>) -> Self {
        Self {
            user_pool,
            event_bus,
        }
    }

    pub async fn handle(&self, command: UpdateUserCommand) -> anyhow::Result<UserAggregate> {
        let mut user_aggregate = self.user_pool.find_by_id(&command.id).await?;

        let user_event =
            user_aggregate.update(command.username, command.email, command.phone, None);

        self.user_pool.save(&user_aggregate).await?;
        self.event_bus.publish(user_event).await;

        Ok(user_aggregate)
    }
}
